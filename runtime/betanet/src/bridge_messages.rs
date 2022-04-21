// Copyright 2020 Axia Technologies (UK) Ltd.
// This file is part of Axia.

// Axia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia.  If not, see <http://www.gnu.org/licenses/>.

//! Over-bridge messaging support for Betanet <> Wococo bridge.

pub use self::{at_betanet::*, at_wococo::*};

use bp_messages::{
	source_chain::TargetHeaderChain,
	target_chain::{ProvedMessages, SourceHeaderChain},
	InboundLaneData, LaneId, Message, MessageNonce,
};
use bp_betanet::{
	max_extrinsic_size, max_extrinsic_weight, EXTRA_STORAGE_PROOF_SIZE,
	MAXIMAL_ENCODED_ACCOUNT_ID_SIZE,
};
use bp_runtime::{ChainId, BETANET_CHAIN_ID, WOCOCO_CHAIN_ID};
use bridge_runtime_common::messages::{
	source as messages_source, target as messages_target, BridgedChainWithMessages,
	ChainWithMessages, MessageBridge, MessageTransaction, ThisChainWithMessages,
};
use frame_support::{
	traits::Get,
	weights::{Weight, WeightToFeePolynomial},
	RuntimeDebug,
};
use sp_std::{convert::TryFrom, marker::PhantomData, ops::RangeInclusive};

use betanet_runtime_constants::fee::WeightToFee;

/// Maximal number of pending outbound messages.
const MAXIMAL_PENDING_MESSAGES_AT_OUTBOUND_LANE: MessageNonce =
	bp_betanet::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE;
/// Maximal weight of single message delivery confirmation transaction on Betanet/Wococo chain.
///
/// This value is a result of `pallet_bridge_messages::Pallet::receive_messages_delivery_proof` weight formula
/// computation for the case when single message is confirmed. The result then must be rounded up to account
/// possible future runtime upgrades.
const MAX_SINGLE_MESSAGE_DELIVERY_CONFIRMATION_TX_WEIGHT: Weight = 2_000_000_000;
/// Increase of delivery transaction weight on Betanet/Wococo chain with every additional message byte.
///
/// This value is a result of `pallet_bridge_messages::WeightInfoExt::storage_proof_size_overhead(1)` call. The
/// result then must be rounded up to account possible future runtime upgrades.
const ADDITIONAL_MESSAGE_BYTE_DELIVERY_WEIGHT: Weight = 25_000;
/// Weight of single regular message delivery transaction on Betanet/Wococo chain.
///
/// This value is a result of `pallet_bridge_messages::Pallet::receive_messages_proof_weight()` call
/// for the case when single message of `pallet_bridge_messages::EXPECTED_DEFAULT_MESSAGE_LENGTH` bytes is delivered.
/// The message must have dispatch weight set to zero. The result then must be rounded up to account
/// possible future runtime upgrades.
const DEFAULT_MESSAGE_DELIVERY_TX_WEIGHT: Weight = 1_500_000_000;
/// Weight of pay-dispatch-fee operation for inbound messages at Betanet/Wococo chain.
///
/// This value corresponds to the result of `pallet_bridge_messages::WeightInfoExt::pay_inbound_dispatch_fee_overhead()`
/// call for your chain. Don't put too much reserve there, because it is used to **decrease**
/// `DEFAULT_MESSAGE_DELIVERY_TX_WEIGHT` cost. So putting large reserve would make delivery transactions cheaper.
const PAY_INBOUND_DISPATCH_FEE_WEIGHT: Weight = 600_000_000;
/// Number of bytes, included in the signed Betanet/Wococo transaction apart from the encoded call itself.
///
/// Can be computed by subtracting encoded call size from raw transaction size.
const TX_EXTRA_BYTES: u32 = 130;

/// Betanet chain as it is seen at Betanet.
pub type BetanetAtBetanet =
	BetanetLikeChain<AtBetanetWithWococoMessageBridge, crate::BetanetGrandpaInstance>;

/// Betanet chain as it is seen at Wococo.
pub type BetanetAtWococo =
	BetanetLikeChain<AtWococoWithBetanetMessageBridge, crate::BetanetGrandpaInstance>;

/// Wococo chain as it is seen at Wococo.
pub type WococoAtWococo =
	BetanetLikeChain<AtWococoWithBetanetMessageBridge, crate::WococoGrandpaInstance>;

/// Wococo chain as it is seen at Betanet.
pub type WococoAtBetanet =
	BetanetLikeChain<AtBetanetWithWococoMessageBridge, crate::WococoGrandpaInstance>;

/// Betanet/Wococo chain from message lane point of view.
#[derive(RuntimeDebug, Clone, Copy)]
pub struct BetanetLikeChain<B, GI> {
	_bridge_definition: PhantomData<B>,
	_at_this_chain_grandpa_pallet_instance: PhantomData<GI>,
}

impl<B, GI> ChainWithMessages for BetanetLikeChain<B, GI> {
	type Hash = crate::Hash;
	type AccountId = crate::AccountId;
	type Signer = primitives::v1::AccountPublic;
	type Signature = crate::Signature;
	type Weight = Weight;
	type Balance = crate::Balance;
}

impl<B, GI> ThisChainWithMessages for BetanetLikeChain<B, GI> {
	type Call = crate::Call;

	fn is_outbound_lane_enabled(lane: &LaneId) -> bool {
		*lane == [0, 0, 0, 0]
	}

	fn maximal_pending_messages_at_outbound_lane() -> MessageNonce {
		MAXIMAL_PENDING_MESSAGES_AT_OUTBOUND_LANE
	}

	fn estimate_delivery_confirmation_transaction() -> MessageTransaction<Weight> {
		let inbound_data_size = InboundLaneData::<crate::AccountId>::encoded_size_hint(
			MAXIMAL_ENCODED_ACCOUNT_ID_SIZE,
			1,
			1,
		)
		.unwrap_or(u32::MAX);

		MessageTransaction {
			dispatch_weight: MAX_SINGLE_MESSAGE_DELIVERY_CONFIRMATION_TX_WEIGHT,
			size: inbound_data_size
				.saturating_add(EXTRA_STORAGE_PROOF_SIZE)
				.saturating_add(TX_EXTRA_BYTES),
		}
	}

	fn transaction_payment(transaction: MessageTransaction<Weight>) -> crate::Balance {
		// current fee multiplier is used here
		bridge_runtime_common::messages::transaction_payment(
			crate::BlockWeights::get()
				.get(frame_support::weights::DispatchClass::Normal)
				.base_extrinsic,
			crate::TransactionByteFee::get(),
			pallet_transaction_payment::Pallet::<crate::Runtime>::next_fee_multiplier(),
			|weight| WeightToFee::calc(&weight),
			transaction,
		)
	}
}

impl<B, GI> BridgedChainWithMessages for BetanetLikeChain<B, GI> {
	fn maximal_extrinsic_size() -> u32 {
		max_extrinsic_size()
	}

	fn message_weight_limits(_message_payload: &[u8]) -> RangeInclusive<Weight> {
		// we don't want to relay too large messages + keep reserve for future upgrades
		let upper_limit =
			messages_target::maximal_incoming_message_dispatch_weight(max_extrinsic_weight());

		// we're charging for payload bytes in `With(Wococo | Betanet)MessageBridge::transaction_payment` function
		//
		// this bridge may be used to deliver all kind of messages, so we're not making any assumptions about
		// minimal dispatch weight here

		0..=upper_limit
	}

	fn estimate_delivery_transaction(
		message_payload: &[u8],
		include_pay_dispatch_fee_cost: bool,
		message_dispatch_weight: Weight,
	) -> MessageTransaction<Weight> {
		let message_payload_len = u32::try_from(message_payload.len()).unwrap_or(u32::MAX);
		let extra_bytes_in_payload = Weight::from(message_payload_len)
			.saturating_sub(pallet_bridge_messages::EXPECTED_DEFAULT_MESSAGE_LENGTH.into());

		MessageTransaction {
			dispatch_weight: extra_bytes_in_payload
				.saturating_mul(ADDITIONAL_MESSAGE_BYTE_DELIVERY_WEIGHT)
				.saturating_add(DEFAULT_MESSAGE_DELIVERY_TX_WEIGHT)
				.saturating_sub(if include_pay_dispatch_fee_cost {
					0
				} else {
					PAY_INBOUND_DISPATCH_FEE_WEIGHT
				})
				.saturating_add(message_dispatch_weight),
			size: message_payload_len
				.saturating_add(EXTRA_STORAGE_PROOF_SIZE)
				.saturating_add(TX_EXTRA_BYTES),
		}
	}

	fn transaction_payment(transaction: MessageTransaction<Weight>) -> crate::Balance {
		// current fee multiplier is used here
		bridge_runtime_common::messages::transaction_payment(
			crate::BlockWeights::get()
				.get(frame_support::weights::DispatchClass::Normal)
				.base_extrinsic,
			crate::TransactionByteFee::get(),
			pallet_transaction_payment::Pallet::<crate::Runtime>::next_fee_multiplier(),
			|weight| WeightToFee::calc(&weight),
			transaction,
		)
	}
}

impl<B, GI> TargetHeaderChain<messages_source::FromThisChainMessagePayload<B>, crate::AccountId>
	for BetanetLikeChain<B, GI>
where
	B: MessageBridge,
	B::ThisChain: ChainWithMessages<AccountId = crate::AccountId>,
	B::BridgedChain: ChainWithMessages<Hash = crate::Hash>,
	GI: 'static,
	crate::Runtime: pallet_bridge_grandpa::Config<GI>,
	<<crate::Runtime as pallet_bridge_grandpa::Config<GI>>::BridgedChain as bp_runtime::Chain>::Hash: From<crate::Hash>,
{
	type Error = &'static str;
	type MessagesDeliveryProof = messages_source::FromBridgedChainMessagesDeliveryProof<crate::Hash>;

	fn verify_message(payload: &messages_source::FromThisChainMessagePayload<B>) -> Result<(), Self::Error> {
		messages_source::verify_chain_message::<B>(payload)
	}

	fn verify_messages_delivery_proof(
		proof: Self::MessagesDeliveryProof,
	) -> Result<(LaneId, InboundLaneData<crate::AccountId>), Self::Error> {
		messages_source::verify_messages_delivery_proof::<B, crate::Runtime, GI>(proof)
	}
}

impl<B, GI> SourceHeaderChain<crate::Balance> for BetanetLikeChain<B, GI>
where
	B: MessageBridge,
	B::BridgedChain: ChainWithMessages<Balance = crate::Balance, Hash = crate::Hash>,
	GI: 'static,
	crate::Runtime: pallet_bridge_grandpa::Config<GI>,
	<<crate::Runtime as pallet_bridge_grandpa::Config<GI>>::BridgedChain as bp_runtime::Chain>::Hash: From<crate::Hash>,
{
	type Error = &'static str;
	type MessagesProof = messages_target::FromBridgedChainMessagesProof<crate::Hash>;

	fn verify_messages_proof(
		proof: Self::MessagesProof,
		messages_count: u32,
	) -> Result<ProvedMessages<Message<crate::Balance>>, Self::Error> {
		messages_target::verify_messages_proof::<B, crate::Runtime, GI>(proof, messages_count).and_then(verify_inbound_messages_lane)
	}
}

/// Error that happens when we are receiving incoming message via unexpected lane.
const INBOUND_LANE_DISABLED: &str = "The inbound message lane is disabled.";

/// Verify that lanes of inbound messages are enabled.
fn verify_inbound_messages_lane(
	messages: ProvedMessages<Message<crate::Balance>>,
) -> Result<ProvedMessages<Message<crate::Balance>>, &'static str> {
	let allowed_incoming_lanes = [[0, 0, 0, 0]];
	if messages.keys().any(|lane_id| !allowed_incoming_lanes.contains(lane_id)) {
		return Err(INBOUND_LANE_DISABLED)
	}
	Ok(messages)
}

/// The cost of delivery confirmation transaction.
pub struct GetDeliveryConfirmationTransactionFee;

impl Get<crate::Balance> for GetDeliveryConfirmationTransactionFee {
	fn get() -> crate::Balance {
		<BetanetAtBetanet as ThisChainWithMessages>::transaction_payment(
			BetanetAtBetanet::estimate_delivery_confirmation_transaction(),
		)
	}
}

/// This module contains definitions that are used by the messages pallet instance, "deployed" at Betanet.
mod at_betanet {
	use super::*;

	/// Message bridge that is "deployed" at Betanet chain and connecting it to Wococo chain.
	#[derive(RuntimeDebug, Clone, Copy)]
	pub struct AtBetanetWithWococoMessageBridge;

	impl MessageBridge for AtBetanetWithWococoMessageBridge {
		const THIS_CHAIN_ID: ChainId = BETANET_CHAIN_ID;
		const BRIDGED_CHAIN_ID: ChainId = WOCOCO_CHAIN_ID;
		const RELAYER_FEE_PERCENT: u32 = 10;
		const BRIDGED_MESSAGES_PALLET_NAME: &'static str =
			bp_wococo::WITH_BETANET_MESSAGES_PALLET_NAME;

		type ThisChain = BetanetAtBetanet;
		type BridgedChain = WococoAtBetanet;

		fn bridged_balance_to_this_balance(
			bridged_balance: bp_wococo::Balance,
		) -> bp_betanet::Balance {
			bridged_balance
		}
	}

	/// Message payload for Betanet -> Wococo messages as it is seen at the Betanet.
	pub type ToWococoMessagePayload =
		messages_source::FromThisChainMessagePayload<AtBetanetWithWococoMessageBridge>;

	/// Message verifier for Betanet -> Wococo messages at Betanet.
	pub type ToWococoMessageVerifier =
		messages_source::FromThisChainMessageVerifier<AtBetanetWithWococoMessageBridge>;

	/// Message payload for Wococo -> Betanet messages as it is seen at Betanet.
	pub type FromWococoMessagePayload =
		messages_target::FromBridgedChainMessagePayload<AtBetanetWithWococoMessageBridge>;

	/// Encoded Betanet Call as it comes from Wococo.
	pub type FromWococoEncodedCall =
		messages_target::FromBridgedChainEncodedMessageCall<crate::Call>;

	/// Call-dispatch based message dispatch for Wococo -> Betanet messages.
	pub type FromWococoMessageDispatch = messages_target::FromBridgedChainMessageDispatch<
		AtBetanetWithWococoMessageBridge,
		crate::Runtime,
		pallet_balances::Pallet<crate::Runtime>,
		crate::AtBetanetFromWococoMessagesDispatch,
	>;
}

/// This module contains definitions that are used by the messages pallet instance, "deployed" at Wococo.
mod at_wococo {
	use super::*;

	/// Message bridge that is "deployed" at Wococo chain and connecting it to Betanet chain.
	#[derive(RuntimeDebug, Clone, Copy)]
	pub struct AtWococoWithBetanetMessageBridge;

	impl MessageBridge for AtWococoWithBetanetMessageBridge {
		const THIS_CHAIN_ID: ChainId = WOCOCO_CHAIN_ID;
		const BRIDGED_CHAIN_ID: ChainId = BETANET_CHAIN_ID;
		const RELAYER_FEE_PERCENT: u32 = 10;
		const BRIDGED_MESSAGES_PALLET_NAME: &'static str =
			bp_betanet::WITH_WOCOCO_MESSAGES_PALLET_NAME;

		type ThisChain = WococoAtWococo;
		type BridgedChain = BetanetAtWococo;

		fn bridged_balance_to_this_balance(
			bridged_balance: bp_betanet::Balance,
		) -> bp_wococo::Balance {
			bridged_balance
		}
	}

	/// Message payload for Wococo -> Betanet messages as it is seen at the Wococo.
	pub type ToBetanetMessagePayload =
		messages_source::FromThisChainMessagePayload<AtWococoWithBetanetMessageBridge>;

	/// Message verifier for Wococo -> Betanet messages at Wococo.
	pub type ToBetanetMessageVerifier =
		messages_source::FromThisChainMessageVerifier<AtWococoWithBetanetMessageBridge>;

	/// Message payload for Betanet -> Wococo messages as it is seen at Wococo.
	pub type FromBetanetMessagePayload =
		messages_target::FromBridgedChainMessagePayload<AtWococoWithBetanetMessageBridge>;

	/// Encoded Wococo Call as it comes from Betanet.
	pub type FromBetanetEncodedCall =
		messages_target::FromBridgedChainEncodedMessageCall<crate::Call>;

	/// Call-dispatch based message dispatch for Betanet -> Wococo messages.
	pub type FromBetanetMessageDispatch = messages_target::FromBridgedChainMessageDispatch<
		AtWococoWithBetanetMessageBridge,
		crate::Runtime,
		pallet_balances::Pallet<crate::Runtime>,
		crate::AtWococoFromBetanetMessagesDispatch,
	>;
}

#[cfg(test)]
mod tests {
	use super::*;
	use bp_messages::{target_chain::ProvedLaneMessages, MessageData, MessageKey};
	use bridge_runtime_common::messages;
	use axia_scale_codec::{Decode, Encode};
	use sp_runtime::traits::TrailingZeroInput;

	#[test]
	fn ensure_betanet_messages_weights_are_correct() {
		// **NOTE**: the main purpose of this test is to be sure that any message that is sumbitted
		// to (any) inbound lane in Betanet<>Wococo bridge can be delivered to the bridged chain.
		// Since we deal with testnets here, in case of failure + urgency:
		//
		// 1) ping bridges team about this failure (see the CODEOWNERS file if you're unsure who to ping);
		// 2) comment/#[ignore] the test.

		// we don't have any knowledge of messages-at-Betanet weights, so we'll be using
		// weights of one of our testnets, which should be accurate enough
		type Weights = pallet_bridge_messages::weights::RialtoWeight<crate::Runtime>;

		pallet_bridge_messages::ensure_weights_are_correct::<Weights>(
			DEFAULT_MESSAGE_DELIVERY_TX_WEIGHT,
			ADDITIONAL_MESSAGE_BYTE_DELIVERY_WEIGHT,
			MAX_SINGLE_MESSAGE_DELIVERY_CONFIRMATION_TX_WEIGHT,
			PAY_INBOUND_DISPATCH_FEE_WEIGHT,
			crate::RocksDbWeight::get(),
		);

		let max_incoming_message_proof_size = bp_betanet::EXTRA_STORAGE_PROOF_SIZE.saturating_add(
			messages::target::maximal_incoming_message_size(bp_betanet::max_extrinsic_size()),
		);
		pallet_bridge_messages::ensure_able_to_receive_message::<Weights>(
			bp_betanet::max_extrinsic_size(),
			bp_betanet::max_extrinsic_weight(),
			max_incoming_message_proof_size,
			messages::target::maximal_incoming_message_dispatch_weight(
				bp_betanet::max_extrinsic_weight(),
			),
		);

		let max_incoming_inbound_lane_data_proof_size =
			bp_messages::InboundLaneData::<()>::encoded_size_hint(
				bp_betanet::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE,
				bp_betanet::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE as _,
				bp_betanet::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE as _,
			)
			.unwrap_or(u32::MAX);
		pallet_bridge_messages::ensure_able_to_receive_confirmation::<Weights>(
			bp_betanet::max_extrinsic_size(),
			bp_betanet::max_extrinsic_weight(),
			max_incoming_inbound_lane_data_proof_size,
			bp_betanet::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
			bp_betanet::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE,
			crate::RocksDbWeight::get(),
		);
	}

	#[test]
	fn ensure_betanet_tx_extra_bytes_constant_is_correct() {
		// **NOTE**: this test checks that we're computing transaction fee (for bridged chain, which, in
		// case of Betanet<>Wococo, means any chain) on-chain properly. If this assert fails:
		//
		// 1) just fix the `TX_EXTRA_BYTES` constant to actual (or sightly rounded up) value;
		// 2) (only if it has changed significantly (> x2 times)) ping the bridges team (see the CODEOWNERS
		//    file if you're unsure who to ping)

		let signed_extra: crate::SignedExtra = (
			frame_system::CheckNonZeroSender::new(),
			frame_system::CheckSpecVersion::new(),
			frame_system::CheckTxVersion::new(),
			frame_system::CheckGenesis::new(),
			frame_system::CheckMortality::from(sp_runtime::generic::Era::mortal(
				u64::MAX,
				u64::MAX,
			)),
			frame_system::CheckNonce::from(primitives::v1::Nonce::MAX),
			frame_system::CheckWeight::new(),
			pallet_transaction_payment::ChargeTransactionPayment::from(
				primitives::v1::Balance::MAX,
			),
		);
		let mut zeroes = TrailingZeroInput::zeroes();
		let extra_bytes_in_transaction = signed_extra.encoded_size() +
			crate::Address::decode(&mut zeroes).unwrap().encoded_size() +
			crate::Signature::decode(&mut zeroes).unwrap().encoded_size();
		assert!(
			TX_EXTRA_BYTES as usize >= extra_bytes_in_transaction,
			"Hardcoded number of extra bytes in Betanet transaction {} is lower than actual value: {}",
			TX_EXTRA_BYTES,
			extra_bytes_in_transaction,
		);
	}

	fn proved_messages(lane_id: LaneId) -> ProvedMessages<Message<crate::Balance>> {
		vec![(
			lane_id,
			ProvedLaneMessages {
				lane_state: None,
				messages: vec![Message {
					key: MessageKey { lane_id, nonce: 0 },
					data: MessageData { payload: vec![], fee: 0 },
				}],
			},
		)]
		.into_iter()
		.collect()
	}

	#[test]
	fn verify_inbound_messages_lane_succeeds() {
		assert_eq!(
			verify_inbound_messages_lane(proved_messages([0, 0, 0, 0])),
			Ok(proved_messages([0, 0, 0, 0])),
		);
	}

	#[test]
	fn verify_inbound_messages_lane_fails() {
		assert_eq!(
			verify_inbound_messages_lane(proved_messages([0, 0, 0, 1])),
			Err(INBOUND_LANE_DISABLED),
		);

		let proved_messages = proved_messages([0, 0, 0, 0])
			.into_iter()
			.chain(proved_messages([0, 0, 0, 1]))
			.collect();
		assert_eq!(verify_inbound_messages_lane(proved_messages), Err(INBOUND_LANE_DISABLED),);
	}
}
