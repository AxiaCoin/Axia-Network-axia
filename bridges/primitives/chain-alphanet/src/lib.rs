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
use scale_info::TypeInfo;
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub use bp_axia_core::*;

/// Alphanet Chain
pub type Alphanet = AxiaLike;

// NOTE: This needs to be kept up to date with the Alphanet runtime found in the Axia repo.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		const CENTS: Balance = 1_000_000_000_000 / 1_000;
		// in Alphanet, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
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

// NOTE: This needs to be kept up to date with the Alphanet runtime found in the Axia repo.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: sp_version::create_runtime_str!("alphanet"),
	impl_name: sp_version::create_runtime_str!("axia-alphanet"),
	authoring_version: 2,
	spec_version: 51,
	impl_version: 0,
	apis: sp_version::create_apis_vec![[]],
	transaction_version: 5,
};

/// Alphanet Runtime `Call` enum.
///
/// We are not currently submitting any Alphanet transactions => it is empty.
#[derive(
	axia_scale_codec::Encode, axia_scale_codec::Decode, Debug, PartialEq, Eq, Clone, TypeInfo,
)]
pub enum Call {}

impl sp_runtime::traits::Dispatchable for Call {
	type Origin = ();
	type Config = ();
	type Info = ();
	type PostInfo = ();

	fn dispatch(self, _origin: Self::Origin) -> sp_runtime::DispatchResultWithInfo<Self::PostInfo> {
		unimplemented!("The Call is not expected to be dispatched.")
	}
}

// We use this to get the account on Alphanet (target) which is derived from Betanet's (source)
// account.
pub fn derive_account_from_betanet_id(id: bp_runtime::SourceAccount<AccountId>) -> AccountId {
	let encoded_id = bp_runtime::derive_account_id(bp_runtime::BETANET_CHAIN_ID, id);
	AccountIdConverter::convert(encoded_id)
}

/// Name of the `AlphanetFinalityApi::best_finalized` runtime method.
pub const BEST_FINALIZED_ALPHANET_HEADER_METHOD: &str = "AlphanetFinalityApi_best_finalized";
/// Name of the `AlphanetFinalityApi::is_known_header` runtime method.
pub const IS_KNOWN_ALPHANET_HEADER_METHOD: &str = "AlphanetFinalityApi_is_known_header";

/// Name of the `ToAlphanetOutboundLaneApi::estimate_message_delivery_and_dispatch_fee` runtime
/// method.
pub const TO_ALPHANET_ESTIMATE_MESSAGE_FEE_METHOD: &str =
	"ToAlphanetOutboundLaneApi_estimate_message_delivery_and_dispatch_fee";
/// Name of the `ToAlphanetOutboundLaneApi::message_details` runtime method.
pub const TO_ALPHANET_MESSAGE_DETAILS_METHOD: &str = "ToAlphanetOutboundLaneApi_message_details";
/// Name of the `ToAlphanetOutboundLaneApi::latest_generated_nonce` runtime method.
pub const TO_ALPHANET_LATEST_GENERATED_NONCE_METHOD: &str =
	"ToAlphanetOutboundLaneApi_latest_generated_nonce";
/// Name of the `ToAlphanetOutboundLaneApi::latest_received_nonce` runtime method.
pub const TO_ALPHANET_LATEST_RECEIVED_NONCE_METHOD: &str =
	"ToAlphanetOutboundLaneApi_latest_received_nonce";

/// Name of the `FromAlphanetInboundLaneApi::latest_received_nonce` runtime method.
pub const FROM_ALPHANET_LATEST_RECEIVED_NONCE_METHOD: &str =
	"FromAlphanetInboundLaneApi_latest_received_nonce";
/// Name of the `FromAlphanetInboundLaneApi::latest_onfirmed_nonce` runtime method.
pub const FROM_ALPHANET_LATEST_CONFIRMED_NONCE_METHOD: &str =
	"FromAlphanetInboundLaneApi_latest_confirmed_nonce";
/// Name of the `FromAlphanetInboundLaneApi::unrewarded_relayers_state` runtime method.
pub const FROM_ALPHANET_UNREWARDED_RELAYERS_STATE: &str =
	"FromAlphanetInboundLaneApi_unrewarded_relayers_state";

/// The target length of a session (how often authorities change) on Alphanet measured in of number
/// of blocks.
///
/// Note that since this is a target sessions may change before/after this time depending on network
/// conditions.
pub const SESSION_LENGTH: BlockNumber = 10 * time_units::MINUTES;

sp_api::decl_runtime_apis! {
	/// API for querying information about the finalized Alphanet headers.
	///
	/// This API is implemented by runtimes that are bridging with the Alphanet chain, not the
	/// Alphanet runtime itself.
	pub trait AlphanetFinalityApi {
		/// Returns number and hash of the best finalized header known to the bridge module.
		fn best_finalized() -> (BlockNumber, Hash);
		/// Returns true if the header is known to the runtime.
		fn is_known_header(hash: Hash) -> bool;
	}

	/// Outbound message lane API for messages that are sent to Alphanet chain.
	///
	/// This API is implemented by runtimes that are sending messages to Alphanet chain, not the
	/// Alphanet runtime itself.
	pub trait ToAlphanetOutboundLaneApi<OutboundMessageFee: Parameter, OutboundPayload: Parameter> {
		/// Estimate message delivery and dispatch fee that needs to be paid by the sender on
		/// this chain.
		///
		/// Returns `None` if message is too expensive to be sent to Alphanet from this chain.
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

	/// Inbound message lane API for messages sent by Alphanet chain.
	///
	/// This API is implemented by runtimes that are receiving messages from Alphanet chain, not the
	/// Alphanet runtime itself.
	pub trait FromAlphanetInboundLaneApi {
		/// Returns nonce of the latest message, received by given lane.
		fn latest_received_nonce(lane: LaneId) -> MessageNonce;
		/// Nonce of the latest message that has been confirmed to the bridged chain.
		fn latest_confirmed_nonce(lane: LaneId) -> MessageNonce;
		/// State of the unrewarded relayers set at given lane.
		fn unrewarded_relayers_state(lane: LaneId) -> UnrewardedRelayersState;
	}
}
