// Copyright 2019-2021 Axia Technologies (UK) Ltd.
// This file is part of Axia Bridges Common.

// Axia Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Wococo-to-Betanet messages sync entrypoint.

use std::ops::RangeInclusive;

use codec::Encode;
use sp_core::{Bytes, Pair};

use bp_messages::MessageNonce;
use bridge_runtime_common::messages::target::FromBridgedChainMessagesProof;
use frame_support::weights::Weight;
use messages_relay::{message_lane::MessageLane, relay_strategy::MixStrategy};
use relay_betanet_client::{
	HeaderId as BetanetHeaderId, Betanet, SigningParams as BetanetSigningParams,
};
use relay_axlib_client::{Chain, Client, IndexOf, TransactionSignScheme, UnsignedTransaction};
use relay_wococo_client::{
	HeaderId as WococoHeaderId, SigningParams as WococoSigningParams, Wococo,
};
use axlib_relay_helper::{
	messages_lane::{
		select_delivery_transaction_limits, MessagesRelayParams, StandaloneMessagesMetrics,
		AxlibMessageLane, AxlibMessageLaneToAxlib,
	},
	messages_source::AxlibMessagesSource,
	messages_target::AxlibMessagesTarget,
	STALL_TIMEOUT,
};

/// Wococo-to-Betanet message lane.
pub type MessageLaneWococoMessagesToBetanet =
	AxlibMessageLaneToAxlib<Wococo, WococoSigningParams, Betanet, BetanetSigningParams>;

#[derive(Clone)]
pub struct WococoMessagesToBetanet {
	message_lane: MessageLaneWococoMessagesToBetanet,
}

impl AxlibMessageLane for WococoMessagesToBetanet {
	type MessageLane = MessageLaneWococoMessagesToBetanet;
	const OUTBOUND_LANE_MESSAGE_DETAILS_METHOD: &'static str =
		bp_betanet::TO_BETANET_MESSAGE_DETAILS_METHOD;
	const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
		bp_betanet::TO_BETANET_LATEST_GENERATED_NONCE_METHOD;
	const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
		bp_betanet::TO_BETANET_LATEST_RECEIVED_NONCE_METHOD;

	const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
		bp_wococo::FROM_WOCOCO_LATEST_RECEIVED_NONCE_METHOD;
	const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
		bp_wococo::FROM_WOCOCO_LATEST_CONFIRMED_NONCE_METHOD;
	const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
		bp_wococo::FROM_WOCOCO_UNREWARDED_RELAYERS_STATE;

	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
		bp_wococo::BEST_FINALIZED_WOCOCO_HEADER_METHOD;
	const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
		bp_betanet::BEST_FINALIZED_BETANET_HEADER_METHOD;

	const MESSAGE_PALLET_NAME_AT_SOURCE: &'static str = bp_wococo::WITH_BETANET_MESSAGES_PALLET_NAME;
	const MESSAGE_PALLET_NAME_AT_TARGET: &'static str = bp_betanet::WITH_WOCOCO_MESSAGES_PALLET_NAME;

	const PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN: Weight =
		bp_betanet::PAY_INBOUND_DISPATCH_FEE_WEIGHT;

	type SourceChain = Wococo;
	type TargetChain = Betanet;

	fn source_transactions_author(&self) -> bp_wococo::AccountId {
		(*self.message_lane.source_sign.public().as_array_ref()).into()
	}

	fn make_messages_receiving_proof_transaction(
		&self,
		best_block_id: WococoHeaderId,
		transaction_nonce: IndexOf<Wococo>,
		_generated_at_block: BetanetHeaderId,
		proof: <Self::MessageLane as MessageLane>::MessagesReceivingProof,
	) -> Bytes {
		let (relayers_state, proof) = proof;
		let call = relay_wococo_client::runtime::Call::BridgeMessagesBetanet(
			relay_wococo_client::runtime::BridgeMessagesBetanetCall::receive_messages_delivery_proof(
				proof,
				relayers_state,
			),
		);
		let genesis_hash = *self.message_lane.source_client.genesis_hash();
		let transaction = Wococo::sign_transaction(
			genesis_hash,
			&self.message_lane.source_sign,
			relay_axlib_client::TransactionEra::new(
				best_block_id,
				self.message_lane.source_transactions_mortality,
			),
			UnsignedTransaction::new(call, transaction_nonce),
		);
		log::trace!(
			target: "bridge",
			"Prepared Betanet -> Wococo confirmation transaction. Weight: <unknown>/{}, size: {}/{}",
			bp_wococo::max_extrinsic_weight(),
			transaction.encode().len(),
			bp_wococo::max_extrinsic_size(),
		);
		Bytes(transaction.encode())
	}

	fn target_transactions_author(&self) -> bp_betanet::AccountId {
		(*self.message_lane.target_sign.public().as_array_ref()).into()
	}

	fn make_messages_delivery_transaction(
		&self,
		best_block_id: WococoHeaderId,
		transaction_nonce: IndexOf<Betanet>,
		_generated_at_header: WococoHeaderId,
		_nonces: RangeInclusive<MessageNonce>,
		proof: <Self::MessageLane as MessageLane>::MessagesProof,
	) -> Bytes {
		let (dispatch_weight, proof) = proof;
		let FromBridgedChainMessagesProof { ref nonces_start, ref nonces_end, .. } = proof;
		let messages_count = nonces_end - nonces_start + 1;

		let call = relay_betanet_client::runtime::Call::BridgeMessagesWococo(
			relay_betanet_client::runtime::BridgeMessagesWococoCall::receive_messages_proof(
				self.message_lane.relayer_id_at_source.clone(),
				proof,
				messages_count as _,
				dispatch_weight,
			),
		);
		let genesis_hash = *self.message_lane.target_client.genesis_hash();
		let transaction = Betanet::sign_transaction(
			genesis_hash,
			&self.message_lane.target_sign,
			relay_axlib_client::TransactionEra::new(
				best_block_id,
				self.message_lane.target_transactions_mortality,
			),
			UnsignedTransaction::new(call, transaction_nonce),
		);
		log::trace!(
			target: "bridge",
			"Prepared Wococo -> Betanet delivery transaction. Weight: <unknown>/{}, size: {}/{}",
			bp_betanet::max_extrinsic_weight(),
			transaction.encode().len(),
			bp_betanet::max_extrinsic_size(),
		);
		Bytes(transaction.encode())
	}
}

/// Wococo node as messages source.
type WococoSourceClient = AxlibMessagesSource<WococoMessagesToBetanet>;

/// Betanet node as messages target.
type BetanetTargetClient = AxlibMessagesTarget<WococoMessagesToBetanet>;

/// Run Wococo-to-Betanet messages sync.
pub async fn run(
	params: MessagesRelayParams<
		Wococo,
		WococoSigningParams,
		Betanet,
		BetanetSigningParams,
		MixStrategy,
	>,
) -> anyhow::Result<()> {
	let stall_timeout = relay_axlib_client::bidirectional_transaction_stall_timeout(
		params.source_transactions_mortality,
		params.target_transactions_mortality,
		Wococo::AVERAGE_BLOCK_INTERVAL,
		Betanet::AVERAGE_BLOCK_INTERVAL,
		STALL_TIMEOUT,
	);
	let relayer_id_at_wococo = (*params.source_sign.public().as_array_ref()).into();

	let lane_id = params.lane_id;
	let source_client = params.source_client;
	let target_client = params.target_client;
	let lane = WococoMessagesToBetanet {
		message_lane: AxlibMessageLaneToAxlib {
			source_client: source_client.clone(),
			source_sign: params.source_sign,
			source_transactions_mortality: params.source_transactions_mortality,
			target_client: target_client.clone(),
			target_sign: params.target_sign,
			target_transactions_mortality: params.target_transactions_mortality,
			relayer_id_at_source: relayer_id_at_wococo,
		},
	};

	// 2/3 is reserved for proofs and tx overhead
	let max_messages_size_in_single_batch = bp_betanet::max_extrinsic_size() / 3;
	// we don't know exact weights of the Betanet runtime. So to guess weights we'll be using
	// weights from Rialto and then simply dividing it by x2.
	let (max_messages_in_single_batch, max_messages_weight_in_single_batch) =
		select_delivery_transaction_limits::<
			pallet_bridge_messages::weights::RialtoWeight<rialto_runtime::Runtime>,
		>(
			bp_betanet::max_extrinsic_weight(),
			bp_betanet::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
		);
	let (max_messages_in_single_batch, max_messages_weight_in_single_batch) =
		(max_messages_in_single_batch / 2, max_messages_weight_in_single_batch / 2);

	log::info!(
		target: "bridge",
		"Starting Wococo -> Betanet messages relay.\n\t\
			Wococo relayer account id: {:?}\n\t\
			Max messages in single transaction: {}\n\t\
			Max messages size in single transaction: {}\n\t\
			Max messages weight in single transaction: {}\n\t\
			Tx mortality: {:?}/{:?}\n\t\
			Stall timeout: {:?}",
		lane.message_lane.relayer_id_at_source,
		max_messages_in_single_batch,
		max_messages_size_in_single_batch,
		max_messages_weight_in_single_batch,
		params.source_transactions_mortality,
		params.target_transactions_mortality,
		stall_timeout,
	);

	let standalone_metrics = params
		.standalone_metrics
		.map(Ok)
		.unwrap_or_else(|| standalone_metrics(source_client.clone(), target_client.clone()))?;
	messages_relay::message_lane_loop::run(
		messages_relay::message_lane_loop::Params {
			lane: lane_id,
			source_tick: Wococo::AVERAGE_BLOCK_INTERVAL,
			target_tick: Betanet::AVERAGE_BLOCK_INTERVAL,
			reconnect_delay: relay_utils::relay_loop::RECONNECT_DELAY,
			stall_timeout,
			delivery_params: messages_relay::message_lane_loop::MessageDeliveryParams {
				max_unrewarded_relayer_entries_at_target:
					bp_betanet::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
				max_unconfirmed_nonces_at_target:
					bp_betanet::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE,
				max_messages_in_single_batch,
				max_messages_weight_in_single_batch,
				max_messages_size_in_single_batch,
				relay_strategy: params.relay_strategy,
			},
		},
		WococoSourceClient::new(
			source_client.clone(),
			lane.clone(),
			lane_id,
			params.target_to_source_headers_relay,
		),
		BetanetTargetClient::new(
			target_client,
			lane,
			lane_id,
			standalone_metrics.clone(),
			params.source_to_target_headers_relay,
		),
		standalone_metrics.register_and_spawn(params.metrics_params)?,
		futures::future::pending(),
	)
	.await
	.map_err(Into::into)
}

/// Create standalone metrics for the Wococo -> Betanet messages loop.
pub(crate) fn standalone_metrics(
	source_client: Client<Wococo>,
	target_client: Client<Betanet>,
) -> anyhow::Result<StandaloneMessagesMetrics<Wococo, Betanet>> {
	axlib_relay_helper::messages_lane::standalone_metrics(
		source_client,
		target_client,
		None,
		None,
		None,
		None,
	)
}
