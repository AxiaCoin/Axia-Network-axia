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

#![cfg_attr(not(feature = "std"), no_std)]
// RuntimeApi generated functions
#![allow(clippy::too_many_arguments)]
// Runtime-generated DecodeLimit::decode_all_with_depth_limit
#![allow(clippy::unnecessary_mut_passed)]

use bp_messages::{LaneId, MessageDetails, MessageNonce, UnrewardedRelayersState};
use frame_support::weights::{
	Weight, WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub use bp_axia_core::*;

/// Betanet Chain
pub type Betanet = AxiaLike;

/// The target length of a session (how often authorities change) on Alphanet measured in of number
/// of blocks.
///
/// Note that since this is a target sessions may change before/after this time depending on network
/// conditions.
pub const SESSION_LENGTH: BlockNumber = 10 * time_units::MINUTES;

// NOTE: This needs to be kept up to date with the Betanet runtime found in the Axia repo.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: sp_version::create_runtime_str!("betanet"),
	impl_name: sp_version::create_runtime_str!("axia-betanet-v1.6"),
	authoring_version: 0,
	spec_version: 9100,
	impl_version: 0,
	apis: sp_version::create_apis_vec![[]],
	transaction_version: 0,
	state_version: 0,
};

// NOTE: This needs to be kept up to date with the Betanet runtime found in the Axia repo.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Balance> {
		const CENTS: Balance = 1_000_000_000_000 / 100;
		let p = CENTS;
		let q = 10 * Balance::from(ExtrinsicBaseWeight::get());
		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

// We use this to get the account on Betanet (target) which is derived from Wococo's (source)
// account.
pub fn derive_account_from_wococo_id(id: bp_runtime::SourceAccount<AccountId>) -> AccountId {
	let encoded_id = bp_runtime::derive_account_id(bp_runtime::WOCOCO_CHAIN_ID, id);
	AccountIdConverter::convert(encoded_id)
}

/// Name of the With-Wococo messages pallet instance in the Betanet runtime.
pub const WITH_WOCOCO_MESSAGES_PALLET_NAME: &str = "BridgeWococoMessages";

/// Name of the `BetanetFinalityApi::best_finalized` runtime method.
pub const BEST_FINALIZED_BETANET_HEADER_METHOD: &str = "BetanetFinalityApi_best_finalized";
/// Name of the `BetanetFinalityApi::is_known_header` runtime method.
pub const IS_KNOWN_BETANET_HEADER_METHOD: &str = "BetanetFinalityApi_is_known_header";

/// Name of the `ToBetanetOutboundLaneApi::estimate_message_delivery_and_dispatch_fee` runtime
/// method.
pub const TO_BETANET_ESTIMATE_MESSAGE_FEE_METHOD: &str =
	"ToBetanetOutboundLaneApi_estimate_message_delivery_and_dispatch_fee";
/// Name of the `ToBetanetOutboundLaneApi::message_details` runtime method.
pub const TO_BETANET_MESSAGE_DETAILS_METHOD: &str = "ToBetanetOutboundLaneApi_message_details";
/// Name of the `ToBetanetOutboundLaneApi::latest_generated_nonce` runtime method.
pub const TO_BETANET_LATEST_GENERATED_NONCE_METHOD: &str =
	"ToBetanetOutboundLaneApi_latest_generated_nonce";
/// Name of the `ToBetanetOutboundLaneApi::latest_received_nonce` runtime method.
pub const TO_BETANET_LATEST_RECEIVED_NONCE_METHOD: &str =
	"ToBetanetOutboundLaneApi_latest_received_nonce";

/// Name of the `FromBetanetInboundLaneApi::latest_received_nonce` runtime method.
pub const FROM_BETANET_LATEST_RECEIVED_NONCE_METHOD: &str =
	"FromBetanetInboundLaneApi_latest_received_nonce";
/// Name of the `FromBetanetInboundLaneApi::latest_onfirmed_nonce` runtime method.
pub const FROM_BETANET_LATEST_CONFIRMED_NONCE_METHOD: &str =
	"FromBetanetInboundLaneApi_latest_confirmed_nonce";
/// Name of the `FromBetanetInboundLaneApi::unrewarded_relayers_state` runtime method.
pub const FROM_BETANET_UNREWARDED_RELAYERS_STATE: &str =
	"FromBetanetInboundLaneApi_unrewarded_relayers_state";

/// Weight of pay-dispatch-fee operation for inbound messages at Betanet chain.
///
/// This value corresponds to the result of
/// `pallet_bridge_messages::WeightInfoExt::pay_inbound_dispatch_fee_overhead()` call for your
/// chain. Don't put too much reserve there, because it is used to **decrease**
/// `DEFAULT_MESSAGE_DELIVERY_TX_WEIGHT` cost. So putting large reserve would make delivery
/// transactions cheaper.
pub const PAY_INBOUND_DISPATCH_FEE_WEIGHT: Weight = 600_000_000;

sp_api::decl_runtime_apis! {
	/// API for querying information about the finalized Betanet headers.
	///
	/// This API is implemented by runtimes that are bridging with the Betanet chain, not the
	/// Betanet runtime itself.
	pub trait BetanetFinalityApi {
		/// Returns number and hash of the best finalized header known to the bridge module.
		fn best_finalized() -> (BlockNumber, Hash);
		/// Returns true if the header is known to the runtime.
		fn is_known_header(hash: Hash) -> bool;
	}

	/// Outbound message lane API for messages that are sent to Betanet chain.
	///
	/// This API is implemented by runtimes that are sending messages to Betanet chain, not the
	/// Betanet runtime itself.
	pub trait ToBetanetOutboundLaneApi<OutboundMessageFee: Parameter, OutboundPayload: Parameter> {
		/// Estimate message delivery and dispatch fee that needs to be paid by the sender on
		/// this chain.
		///
		/// Returns `None` if message is too expensive to be sent to Betanet from this chain.
		///
		/// Please keep in mind that this method returns the lowest message fee required for message
		/// to be accepted to the lane. It may be good idea to pay a bit over this price to account
		/// future exchange rate changes and guarantee that relayer would deliver your message
		/// to the target chain.
		fn estimate_message_delivery_and_dispatch_fee(
			lane_id: LaneId,
			payload: OutboundPayload,
		) -> Option<OutboundMessageFee>;
		/// Returns dispatch weight, encoded payload size and delivery+dispatch fee of all
		/// messages in given inclusive range.
		///
		/// If some (or all) messages are missing from the storage, they'll also will
		/// be missing from the resulting vector. The vector is ordered by the nonce.
		fn message_details(
			lane: LaneId,
			begin: MessageNonce,
			end: MessageNonce,
		) -> Vec<MessageDetails<OutboundMessageFee>>;
		/// Returns nonce of the latest message, received by bridged chain.
		fn latest_received_nonce(lane: LaneId) -> MessageNonce;
		/// Returns nonce of the latest message, generated by given lane.
		fn latest_generated_nonce(lane: LaneId) -> MessageNonce;
	}

	/// Inbound message lane API for messages sent by Betanet chain.
	///
	/// This API is implemented by runtimes that are receiving messages from Betanet chain, not the
	/// Betanet runtime itself.
	pub trait FromBetanetInboundLaneApi {
		/// Returns nonce of the latest message, received by given lane.
		fn latest_received_nonce(lane: LaneId) -> MessageNonce;
		/// Nonce of the latest message that has been confirmed to the bridged chain.
		fn latest_confirmed_nonce(lane: LaneId) -> MessageNonce;
		/// State of the unrewarded relayers set at given lane.
		fn unrewarded_relayers_state(lane: LaneId) -> UnrewardedRelayersState;
	}
}
