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
	WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub use bp_axia_core::*;

/// Axia Chain
pub type Axia = AxiaLike;

// NOTE: This needs to be kept up to date with the Axia runtime found in the Axia repo.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: sp_version::create_runtime_str!("axia"),
	impl_name: sp_version::create_runtime_str!("axia-axia"),
	authoring_version: 0,
	spec_version: 9100,
	impl_version: 0,
	apis: sp_version::create_apis_vec![[]],
	transaction_version: 7,
};

// NOTE: This needs to be kept up to date with the Axia runtime found in the Axia repo.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		const CENTS: Balance = 10_000_000_000 / 100;
		// in Axia, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
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

// We use this to get the account on Axia (target) which is derived from AxiaTest's (source)
// account.
pub fn derive_account_from_axctest_id(id: bp_runtime::SourceAccount<AccountId>) -> AccountId {
	let encoded_id = bp_runtime::derive_account_id(bp_runtime::AXIATEST_CHAIN_ID, id);
	AccountIdConverter::convert(encoded_id)
}

/// Per-byte fee for Axia transactions.
pub const TRANSACTION_BYTE_FEE: Balance = 10 * 10_000_000_000 / 100 / 1_000;

/// Existential deposit on Axia.
pub const EXISTENTIAL_DEPOSIT: Balance = 10_000_000_000;

/// The target length of a session (how often authorities change) on Axia measured in of number
/// of blocks.
///
/// Note that since this is a target sessions may change before/after this time depending on network
/// conditions.
pub const SESSION_LENGTH: BlockNumber = 4 * time_units::HOURS;

/// Name of the With-AxiaTest messages pallet instance in the Axia runtime.
pub const WITH_AXIATEST_MESSAGES_PALLET_NAME: &str = "BridgeAxiaTestMessages";

/// Name of the AXCT->AXC conversion rate stored in the Axia runtime.
pub const AXIATEST_TO_AXIA_CONVERSION_RATE_PARAMETER_NAME: &str =
	"AxiaTestToAxiaConversionRate";

/// Name of the `AxiaFinalityApi::best_finalized` runtime method.
pub const BEST_FINALIZED_AXIA_HEADER_METHOD: &str = "AxiaFinalityApi_best_finalized";
/// Name of the `AxiaFinalityApi::is_known_header` runtime method.
pub const IS_KNOWN_AXIA_HEADER_METHOD: &str = "AxiaFinalityApi_is_known_header";

/// Name of the `ToAxiaOutboundLaneApi::estimate_message_delivery_and_dispatch_fee` runtime
/// method.
pub const TO_AXIA_ESTIMATE_MESSAGE_FEE_METHOD: &str =
	"ToAxiaOutboundLaneApi_estimate_message_delivery_and_dispatch_fee";
/// Name of the `ToAxiaOutboundLaneApi::message_details` runtime method.
pub const TO_AXIA_MESSAGE_DETAILS_METHOD: &str = "ToAxiaOutboundLaneApi_message_details";
/// Name of the `ToAxiaOutboundLaneApi::latest_generated_nonce` runtime method.
pub const TO_AXIA_LATEST_GENERATED_NONCE_METHOD: &str =
	"ToAxiaOutboundLaneApi_latest_generated_nonce";
/// Name of the `ToAxiaOutboundLaneApi::latest_received_nonce` runtime method.
pub const TO_AXIA_LATEST_RECEIVED_NONCE_METHOD: &str =
	"ToAxiaOutboundLaneApi_latest_received_nonce";

/// Name of the `FromAxiaInboundLaneApi::latest_received_nonce` runtime method.
pub const FROM_AXIA_LATEST_RECEIVED_NONCE_METHOD: &str =
	"FromAxiaInboundLaneApi_latest_received_nonce";
/// Name of the `FromAxiaInboundLaneApi::latest_onfirmed_nonce` runtime method.
pub const FROM_AXIA_LATEST_CONFIRMED_NONCE_METHOD: &str =
	"FromAxiaInboundLaneApi_latest_confirmed_nonce";
/// Name of the `FromAxiaInboundLaneApi::unrewarded_relayers_state` runtime method.
pub const FROM_AXIA_UNREWARDED_RELAYERS_STATE: &str =
	"FromAxiaInboundLaneApi_unrewarded_relayers_state";

sp_api::decl_runtime_apis! {
	/// API for querying information about the finalized Axia headers.
	///
	/// This API is implemented by runtimes that are bridging with the Axia chain, not the
	/// Axia runtime itself.
	pub trait AxiaFinalityApi {
		/// Returns number and hash of the best finalized header known to the bridge module.
		fn best_finalized() -> (BlockNumber, Hash);
		/// Returns true if the header is known to the runtime.
		fn is_known_header(hash: Hash) -> bool;
	}

	/// Outbound message lane API for messages that are sent to Axia chain.
	///
	/// This API is implemented by runtimes that are sending messages to Axia chain, not the
	/// Axia runtime itself.
	pub trait ToAxiaOutboundLaneApi<OutboundMessageFee: Parameter, OutboundPayload: Parameter> {
		/// Estimate message delivery and dispatch fee that needs to be paid by the sender on
		/// this chain.
		///
		/// Returns `None` if message is too expensive to be sent to Axia from this chain.
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

	/// Inbound message lane API for messages sent by Axia chain.
	///
	/// This API is implemented by runtimes that are receiving messages from Axia chain, not the
	/// Axia runtime itself.
	pub trait FromAxiaInboundLaneApi {
		/// Returns nonce of the latest message, received by given lane.
		fn latest_received_nonce(lane: LaneId) -> MessageNonce;
		/// Nonce of the latest message that has been confirmed to the bridged chain.
		fn latest_confirmed_nonce(lane: LaneId) -> MessageNonce;
		/// State of the unrewarded relayers set at given lane.
		fn unrewarded_relayers_state(lane: LaneId) -> UnrewardedRelayersState;
	}
}
