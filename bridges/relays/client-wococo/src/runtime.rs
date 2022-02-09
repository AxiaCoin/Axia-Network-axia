// Copyright 2019-2021 AXIA Technologies (UK) Ltd.
// This file is part of AXIA Bridges Common.

// AXIA Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Types that are specific to the Wococo runtime.

use bp_messages::{LaneId, UnrewardedRelayersState};
use bp_axia_core::AXIALike;
use bp_runtime::Chain;
use codec::{Decode, Encode};
use frame_support::weights::Weight;

/// Instance of messages pallet that is used to bridge with BetaNet chain.
pub type WithBetaNetMessagesInstance = pallet_bridge_messages::DefaultInstance;

/// Unchecked Wococo extrinsic.
pub type UncheckedExtrinsic = bp_axia_core::UncheckedExtrinsic<Call>;

/// BetaNet account ownership digest from Wococo.
///
/// The byte vector returned by this function should be signed with a BetaNet account private key.
/// This way, the owner of `wococo_account_id` on BetaNet proves that the BetaNet account private key
/// is also under his control.
pub fn wococo_to_betanet_account_ownership_digest<Call, AccountId, SpecVersion>(
	betanet_call: &Call,
	wococo_account_id: AccountId,
	betanet_spec_version: SpecVersion,
) -> Vec<u8>
where
	Call: codec::Encode,
	AccountId: codec::Encode,
	SpecVersion: codec::Encode,
{
	pallet_bridge_dispatch::account_ownership_digest(
		betanet_call,
		wococo_account_id,
		betanet_spec_version,
		bp_runtime::WOCOCO_CHAIN_ID,
		bp_runtime::BETANET_CHAIN_ID,
	)
}

/// Wococo Runtime `Call` enum.
///
/// The enum represents a subset of possible `Call`s we can send to BetaNet chain.
/// Ideally this code would be auto-generated from Metadata, because we want to
/// avoid depending directly on the ENTIRE runtime just to get the encoding of `Dispatchable`s.
///
/// All entries here (like pretty much in the entire file) must be kept in sync with BetaNet
/// `construct_runtime`, so that we maintain SCALE-compatibility.
///
/// See: https://github.com/axia/axia/blob/master/runtime/betanet/src/lib.rs
#[allow(clippy::large_enum_variant)]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
pub enum Call {
	/// System pallet.
	#[codec(index = 0)]
	System(SystemCall),
	/// BetaNet bridge pallet.
	#[codec(index = 40)]
	BridgeGrandpaBetaNet(BridgeGrandpaBetaNetCall),
	/// BetaNet messages pallet.
	#[codec(index = 43)]
	BridgeMessagesBetaNet(BridgeMessagesBetaNetCall),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum SystemCall {
	#[codec(index = 1)]
	remark(Vec<u8>),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum BridgeGrandpaBetaNetCall {
	#[codec(index = 0)]
	submit_finality_proof(
		<AXIALike as Chain>::Header,
		bp_header_chain::justification::GrandpaJustification<<AXIALike as Chain>::Header>,
	),
	#[codec(index = 1)]
	initialize(bp_header_chain::InitializationData<<AXIALike as Chain>::Header>),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum BridgeMessagesBetaNetCall {
	#[codec(index = 3)]
	send_message(
		LaneId,
		bp_message_dispatch::MessagePayload<
			bp_betanet::AccountId,
			bp_wococo::AccountId,
			bp_wococo::AccountPublic,
			Vec<u8>,
		>,
		bp_betanet::Balance,
	),
	#[codec(index = 5)]
	receive_messages_proof(
		bp_betanet::AccountId,
		bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<bp_betanet::Hash>,
		u32,
		Weight,
	),
	#[codec(index = 6)]
	receive_messages_delivery_proof(
		bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof<bp_betanet::Hash>,
		UnrewardedRelayersState,
	),
}

impl sp_runtime::traits::Dispatchable for Call {
	type Origin = ();
	type Config = ();
	type Info = ();
	type PostInfo = ();

	fn dispatch(self, _origin: Self::Origin) -> sp_runtime::DispatchResultWithInfo<Self::PostInfo> {
		unimplemented!("The Call is not expected to be dispatched.")
	}
}