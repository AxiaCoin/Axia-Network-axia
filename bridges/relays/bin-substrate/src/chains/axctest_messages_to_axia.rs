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

//! AxiaTest-to-Axia messages sync entrypoint.

use std::ops::RangeInclusive;

use codec::Encode;
use frame_support::weights::Weight;
use sp_core::{Bytes, Pair};

use bp_messages::MessageNonce;
use bridge_runtime_common::messages::target::FromBridgedChainMessagesProof;
use messages_relay::{message_lane::MessageLane, relay_strategy::MixStrategy};
use relay_axctest_client::{
	HeaderId as AxiaTestHeaderId, AxiaTest, SigningParams as AxiaTestSigningParams,
};
use relay_axia_client::{
	HeaderId as AxiaHeaderId, Axia, SigningParams as AxiaSigningParams,
};
use relay_axlib_client::{Chain, Client, TransactionSignScheme, UnsignedTransaction};
use axlib_relay_helper::{
	messages_lane::{
		select_delivery_transaction_limits, MessagesRelayParams, StandaloneMessagesMetrics,
		AxlibMessageLane, AxlibMessageLaneToAxlib,
	},
	messages_source::AxlibMessagesSource,
	messages_target::AxlibMessagesTarget,
	STALL_TIMEOUT,
};

/// AxiaTest-to-Axia message lane.
pub type MessageLaneAxiaTestMessagesToAxia =
	AxlibMessageLaneToAxlib<AxiaTest, AxiaTestSigningParams, Axia, AxiaSigningParams>;

#[derive(Clone)]
pub struct AxiaTestMessagesToAxia {
	message_lane: MessageLaneAxiaTestMessagesToAxia,
}

impl AxlibMessageLane for AxiaTestMessagesToAxia {
	type MessageLane = MessageLaneAxiaTestMessagesToAxia;

	const OUTBOUND_LANE_MESSAGE_DETAILS_METHOD: &'static str =
		bp_axia::TO_AXIA_MESSAGE_DETAILS_METHOD;
	const OUTBOUND_LANE_LATEST_GENERATED_NONCE_METHOD: &'static str =
		bp_axia::TO_AXIA_LATEST_GENERATED_NONCE_METHOD;
	const OUTBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
		bp_axia::TO_AXIA_LATEST_RECEIVED_NONCE_METHOD;

	const INBOUND_LANE_LATEST_RECEIVED_NONCE_METHOD: &'static str =
		bp_axctest::FROM_AXIATEST_LATEST_RECEIVED_NONCE_METHOD;
	const INBOUND_LANE_LATEST_CONFIRMED_NONCE_METHOD: &'static str =
		bp_axctest::FROM_AXIATEST_LATEST_CONFIRMED_NONCE_METHOD;
	const INBOUND_LANE_UNREWARDED_RELAYERS_STATE: &'static str =
		bp_axctest::FROM_AXIATEST_UNREWARDED_RELAYERS_STATE;

	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
		bp_axctest::BEST_FINALIZED_AXIATEST_HEADER_METHOD;
	const BEST_FINALIZED_TARGET_HEADER_ID_AT_SOURCE: &'static str =
		bp_axia::BEST_FINALIZED_AXIA_HEADER_METHOD;

	const MESSAGE_PALLET_NAME_AT_SOURCE: &'static str =
		bp_axctest::WITH_AXIA_MESSAGES_PALLET_NAME;
	const MESSAGE_PALLET_NAME_AT_TARGET: &'static str =
		bp_axia::WITH_AXIATEST_MESSAGES_PALLET_NAME;

	const PAY_INBOUND_DISPATCH_FEE_WEIGHT_AT_TARGET_CHAIN: Weight =
		bp_axia::PAY_INBOUND_DISPATCH_FEE_WEIGHT;

	type SourceChain = AxiaTest;
	type TargetChain = Axia;

	fn source_transactions_author(&self) -> bp_axctest::AccountId {
		(*self.message_lane.source_sign.public().as_array_ref()).into()
	}

	fn make_messages_receiving_proof_transaction(
		&self,
		best_block_id: AxiaTestHeaderId,
		transaction_nonce: bp_runtime::IndexOf<AxiaTest>,
		_generated_at_block: AxiaHeaderId,
		proof: <Self::MessageLane as MessageLane>::MessagesReceivingProof,
	) -> Bytes {
		let (relayers_state, proof) = proof;
		let call = relay_axctest_client::runtime::Call::BridgeAxiaMessages(
			relay_axctest_client::runtime::BridgeAxiaMessagesCall::receive_messages_delivery_proof(
				proof,
				relayers_state,
			),
		);
		let genesis_hash = *self.message_lane.source_client.genesis_hash();
		let transaction = AxiaTest::sign_transaction(
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
			"Prepared Axia -> AxiaTest confirmation transaction. Weight: <unknown>/{}, size: {}/{}",
			bp_axctest::max_extrinsic_weight(),
			transaction.encode().len(),
			bp_axctest::max_extrinsic_size(),
		);
		Bytes(transaction.encode())
	}

	fn target_transactions_author(&self) -> bp_axia::AccountId {
		(*self.message_lane.target_sign.public().as_array_ref()).into()
	}

	fn make_messages_delivery_transaction(
		&self,
		best_block_id: AxiaHeaderId,
		transaction_nonce: bp_runtime::IndexOf<Axia>,
		_generated_at_header: AxiaTestHeaderId,
		_nonces: RangeInclusive<MessageNonce>,
		proof: <Self::MessageLane as MessageLane>::MessagesProof,
	) -> Bytes {
		let (dispatch_weight, proof) = proof;
		let FromBridgedChainMessagesProof { ref nonces_start, ref nonces_end, .. } = proof;
		let messages_count = nonces_end - nonces_start + 1;

		let call = relay_axia_client::runtime::Call::BridgeAxiaTestMessages(
			relay_axia_client::runtime::BridgeAxiaTestMessagesCall::receive_messages_proof(
				self.message_lane.relayer_id_at_source.clone(),
				proof,
				messages_count as _,
				dispatch_weight,
			),
		);
		let genesis_hash = *self.message_lane.target_client.genesis_hash();
		let transaction = Axia::sign_transaction(
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
			"Prepared AxiaTest -> Axia delivery transaction. Weight: <unknown>/{}, size: {}/{}",
			bp_axia::max_extrinsic_weight(),
			transaction.encode().len(),
			bp_axia::max_extrinsic_size(),
		);
		Bytes(transaction.encode())
	}
}

/// AxiaTest node as messages source.
type AxiaTestSourceClient = AxlibMessagesSource<AxiaTestMessagesToAxia>;

/// Axia node as messages target.
type AxiaTargetClient = AxlibMessagesTarget<AxiaTestMessagesToAxia>;

/// Run AxiaTest-to-Axia messages sync.
pub async fn run(
	params: MessagesRelayParams<
		AxiaTest,
		AxiaTestSigningParams,
		Axia,
		AxiaSigningParams,
		MixStrategy,
	>,
) -> anyhow::Result<()> {
	let stall_timeout = relay_axlib_client::bidirectional_transaction_stall_timeout(
		params.source_transactions_mortality,
		params.target_transactions_mortality,
		AxiaTest::AVERAGE_BLOCK_INTERVAL,
		Axia::AVERAGE_BLOCK_INTERVAL,
		STALL_TIMEOUT,
	);
	let relayer_id_at_axctest = (*params.source_sign.public().as_array_ref()).into();

	let lane_id = params.lane_id;
	let source_client = params.source_client;
	let target_client = params.target_client;
	let lane = AxiaTestMessagesToAxia {
		message_lane: AxlibMessageLaneToAxlib {
			source_client: source_client.clone(),
			source_sign: params.source_sign,
			source_transactions_mortality: params.source_transactions_mortality,
			target_client: target_client.clone(),
			target_sign: params.target_sign,
			target_transactions_mortality: params.target_transactions_mortality,
			relayer_id_at_source: relayer_id_at_axctest,
		},
	};

	// 2/3 is reserved for proofs and tx overhead
	let max_messages_size_in_single_batch = bp_axia::max_extrinsic_size() / 3;
	// we don't know exact weights of the Axia runtime. So to guess weights we'll be using
	// weights from Rialto and then simply dividing it by x2.
	let (max_messages_in_single_batch, max_messages_weight_in_single_batch) =
		select_delivery_transaction_limits::<
			pallet_bridge_messages::weights::RialtoWeight<rialto_runtime::Runtime>,
		>(
			bp_axia::max_extrinsic_weight(),
			bp_axia::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
		);
	let (max_messages_in_single_batch, max_messages_weight_in_single_batch) =
		(max_messages_in_single_batch / 2, max_messages_weight_in_single_batch / 2);

	log::info!(
		target: "bridge",
		"Starting AxiaTest -> Axia messages relay.\n\t\
			AxiaTest relayer account id: {:?}\n\t\
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
			source_tick: AxiaTest::AVERAGE_BLOCK_INTERVAL,
			target_tick: Axia::AVERAGE_BLOCK_INTERVAL,
			reconnect_delay: relay_utils::relay_loop::RECONNECT_DELAY,
			stall_timeout,
			delivery_params: messages_relay::message_lane_loop::MessageDeliveryParams {
				max_unrewarded_relayer_entries_at_target:
					bp_axia::MAX_UNREWARDED_RELAYER_ENTRIES_AT_INBOUND_LANE,
				max_unconfirmed_nonces_at_target:
					bp_axia::MAX_UNCONFIRMED_MESSAGES_AT_INBOUND_LANE,
				max_messages_in_single_batch,
				max_messages_weight_in_single_batch,
				max_messages_size_in_single_batch,
				relay_strategy: params.relay_strategy,
			},
		},
		AxiaTestSourceClient::new(
			source_client.clone(),
			lane.clone(),
			lane_id,
			params.target_to_source_headers_relay,
		),
		AxiaTargetClient::new(
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

/// Create standalone metrics for the AxiaTest -> Axia messages loop.
pub(crate) fn standalone_metrics(
	source_client: Client<AxiaTest>,
	target_client: Client<Axia>,
) -> anyhow::Result<StandaloneMessagesMetrics<AxiaTest, Axia>> {
	axlib_relay_helper::messages_lane::standalone_metrics(
		source_client,
		target_client,
		Some(crate::chains::axctest::TOKEN_ID),
		Some(crate::chains::axia::TOKEN_ID),
		Some(crate::chains::axia::axctest_to_axia_conversion_rate_params()),
		Some(crate::chains::axctest::axia_to_axctest_conversion_rate_params()),
	)
}

/// Update Axia -> AxiaTest conversion rate, stored in AxiaTest runtime storage.
pub(crate) async fn update_axia_to_axctest_conversion_rate(
	client: Client<AxiaTest>,
	signer: <AxiaTest as TransactionSignScheme>::AccountKeyPair,
	updated_rate: f64,
) -> anyhow::Result<()> {
	let genesis_hash = *client.genesis_hash();
	let signer_id = (*signer.public().as_array_ref()).into();
	client
		.submit_signed_extrinsic(signer_id, move |_, transaction_nonce| {
			Bytes(
				AxiaTest::sign_transaction(
					genesis_hash,
					&signer,
					relay_axlib_client::TransactionEra::immortal(),
					UnsignedTransaction::new(
						relay_axctest_client::runtime::Call::BridgeAxiaMessages(
							relay_axctest_client::runtime::BridgeAxiaMessagesCall::update_pallet_parameter(
								relay_axctest_client::runtime::BridgeAxiaMessagesParameter::AxiaToAxiaTestConversionRate(
									sp_runtime::FixedU128::from_float(updated_rate),
								)
							)
						),
						transaction_nonce,
					),
				)
					.encode(),
			)
		})
		.await
		.map(drop)
		.map_err(|err| anyhow::format_err!("{:?}", err))
}
