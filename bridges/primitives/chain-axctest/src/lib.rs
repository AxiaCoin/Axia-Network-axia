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

/// AxiaTest Chain
pub type AxiaTest = AxiaLike;

// NOTE: This needs to be kept up to date with the AxiaTest runtime found in the Axia repo.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: sp_version::create_runtime_str!("axctest"),
	impl_name: sp_version::create_runtime_str!("axia-axctest"),
	authoring_version: 2,
	spec_version: 9100,
	impl_version: 0,
	apis: sp_version::create_apis_vec![[]],
	transaction_version: 5,
};

// NOTE: This needs to be kept up to date with the AxiaTest runtime found in the Axia repo.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		const CENTS: Balance = 1_000_000_000_000 / 30_000;
		// in AxiaTest, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
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

// We use this to get the account on AxiaTest (target) which is derived from Axia's (source)
// account.
pub fn derive_account_from_axia_id(id: bp_runtime::SourceAccount<AccountId>) -> AccountId {
	let encoded_id = bp_runtime::derive_account_id(bp_runtime::AXIA_CHAIN_ID, id);
	AccountIdConverter::convert(encoded_id)
}

/// Per-byte fee for AxiaTest transactions.
pub const TRANSACTION_BYTE_FEE: Balance = 10 * 1_000_000_000_000 / 30_000 / 1_000;

/// Existential deposit on AxiaTest.
pub const EXISTENTIAL_DEPOSIT: Balance = 1_000_000_000_000 / 30_000;

/// The target length of a session (how often authorities change) on AxiaTest measured in of number of
/// blocks.
///
/// Note that since this is a target sessions may change before/after this time depending on network
/// conditions.
pub const SESSION_LENGTH: BlockNumber = time_units::HOURS;

/// Name of the With-Axia messages pallet instance in the AxiaTest runtime.
pub const WITH_AXIA_MESSAGES_PALLET_NAME: &str = "BridgeAxiaMessages";

/// Name of the AXC->AXCT conversion rate stored in the AxiaTest runtime.
pub const AXIA_TO_AXIATEST_CONVERSION_RATE_PARAMETER_NAME: &str =
	"AxiaToAxiaTestConversionRate";

/// Name of the `AxiaTestFinalityApi::best_finalized` runtime method.
pub const BEST_FINALIZED_AXIATEST_HEADER_METHOD: &str = "AxiaTestFinalityApi_best_finalized";
/// Name of the `AxiaTestFinalityApi::is_known_header` runtime method.
pub const IS_KNOWN_AXIATEST_HEADER_METHOD: &str = "AxiaTestFinalityApi_is_known_header";

/// Name of the `ToAxiaTestOutboundLaneApi::estimate_message_delivery_and_dispatch_fee` runtime
/// method.
pub const TO_AXIATEST_ESTIMATE_MESSAGE_FEE_METHOD: &str =
	"ToAxiaTestOutboundLaneApi_estimate_message_delivery_and_dispatch_fee";
/// Name of the `ToAxiaTestOutboundLaneApi::message_details` runtime method.
pub const TO_AXIATEST_MESSAGE_DETAILS_METHOD: &str = "ToAxiaTestOutboundLaneApi_message_details";
/// Name of the `ToAxiaTestOutboundLaneApi::latest_generated_nonce` runtime method.
pub const TO_AXIATEST_LATEST_GENERATED_NONCE_METHOD: &str =
	"ToAxiaTestOutboundLaneApi_latest_generated_nonce";
/// Name of the `ToAxiaTestOutboundLaneApi::latest_received_nonce` runtime method.
pub const TO_AXIATEST_LATEST_RECEIVED_NONCE_METHOD: &str =
	"ToAxiaTestOutboundLaneApi_latest_received_nonce";

/// Name of the `FromAxiaTestInboundLaneApi::latest_received_nonce` runtime method.
pub const FROM_AXIATEST_LATEST_RECEIVED_NONCE_METHOD: &str =
	"FromAxiaTestInboundLaneApi_latest_received_nonce";
/// Name of the `FromAxiaTestInboundLaneApi::latest_onfirmed_nonce` runtime method.
pub const FROM_AXIATEST_LATEST_CONFIRMED_NONCE_METHOD: &str =
	"FromAxiaTestInboundLaneApi_latest_confirmed_nonce";
/// Name of the `FromAxiaTestInboundLaneApi::unrewarded_relayers_state` runtime method.
pub const FROM_AXIATEST_UNREWARDED_RELAYERS_STATE: &str =
	"FromAxiaTestInboundLaneApi_unrewarded_relayers_state";

sp_api::decl_runtime_apis! {
	/// API for querying information about the finalized AxiaTest headers.
	///
	/// This API is implemented by runtimes that are bridging with the AxiaTest chain, not the
	/// AxiaTest runtime itself.
	pub trait AxiaTestFinalityApi {
		/// Returns number and hash of the best finalized header known to the bridge module.
		fn best_finalized() -> (BlockNumber, Hash);
		/// Returns true if the header is known to the runtime.
		fn is_known_header(hash: Hash) -> bool;
	}

	/// Outbound message lane API for messages that are sent to AxiaTest chain.
	///
	/// This API is implemented by runtimes that are sending messages to AxiaTest chain, not the
	/// AxiaTest runtime itself.
	pub trait ToAxiaTestOutboundLaneApi<OutboundMessageFee: Parameter, OutboundPayload: Parameter> {
		/// Estimate message delivery and dispatch fee that needs to be paid by the sender on
		/// this chain.
		///
		/// Returns `None` if message is too expensive to be sent to AxiaTest from this chain.
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

	/// Inbound message lane API for messages sent by AxiaTest chain.
	///
	/// This API is implemented by runtimes that are receiving messages from AxiaTest chain, not the
	/// AxiaTest runtime itself.
	pub trait FromAxiaTestInboundLaneApi {
		/// Returns nonce of the latest message, received by given lane.
		fn latest_received_nonce(lane: LaneId) -> MessageNonce;
		/// Nonce of the latest message that has been confirmed to the bridged chain.
		fn latest_confirmed_nonce(lane: LaneId) -> MessageNonce;
		/// State of the unrewarded relayers set at given lane.
		fn unrewarded_relayers_state(lane: LaneId) -> UnrewardedRelayersState;
	}
}
