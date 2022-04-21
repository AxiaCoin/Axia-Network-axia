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

//! Types that are specific to the Betanet runtime.

use bp_messages::{LaneId, UnrewardedRelayersState};
use bp_axia_core::AxiaLike;
use bp_runtime::Chain;
use codec::{Decode, Encode};
use frame_support::weights::Weight;
use scale_info::TypeInfo;

/// Unchecked Betanet extrinsic.
pub type UncheckedExtrinsic = bp_axia_core::UncheckedExtrinsic<Call>;

/// Wococo account ownership digest from Betanet.
///
/// The byte vector returned by this function should be signed with a Wococo account private key.
/// This way, the owner of `betanet_account_id` on Betanet proves that the Wococo account private key
/// is also under his control.
pub fn betanet_to_wococo_account_ownership_digest<Call, AccountId, SpecVersion>(
	wococo_call: &Call,
	betanet_account_id: AccountId,
	wococo_spec_version: SpecVersion,
) -> Vec<u8>
where
	Call: codec::Encode,
	AccountId: codec::Encode,
	SpecVersion: codec::Encode,
{
	pallet_bridge_dispatch::account_ownership_digest(
		wococo_call,
		betanet_account_id,
		wococo_spec_version,
		bp_runtime::BETANET_CHAIN_ID,
		bp_runtime::WOCOCO_CHAIN_ID,
	)
}

/// Betanet Runtime `Call` enum.
///
/// The enum represents a subset of possible `Call`s we can send to Betanet chain.
/// Ideally this code would be auto-generated from metadata, because we want to
/// avoid depending directly on the ENTIRE runtime just to get the encoding of `Dispatchable`s.
///
/// All entries here (like pretty much in the entire file) must be kept in sync with Betanet
/// `construct_runtime`, so that we maintain SCALE-compatibility.
///
/// See: [link](https://github.com/axiatech/axia/blob/master/runtime/betanet/src/lib.rs)
#[allow(clippy::large_enum_variant)]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum Call {
	/// System pallet.
	#[codec(index = 0)]
	System(SystemCall),
	/// Wococo bridge pallet.
	#[codec(index = 41)]
	BridgeGrandpaWococo(BridgeGrandpaWococoCall),
	/// Wococo messages pallet.
	#[codec(index = 44)]
	BridgeMessagesWococo(BridgeMessagesWococoCall),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
#[allow(non_camel_case_types)]
pub enum SystemCall {
	#[codec(index = 1)]
	remark(Vec<u8>),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
#[allow(non_camel_case_types)]
pub enum BridgeGrandpaWococoCall {
	#[codec(index = 0)]
	submit_finality_proof(
		Box<<AxiaLike as Chain>::Header>,
		bp_header_chain::justification::GrandpaJustification<<AxiaLike as Chain>::Header>,
	),
	#[codec(index = 1)]
	initialize(bp_header_chain::InitializationData<<AxiaLike as Chain>::Header>),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
#[allow(non_camel_case_types)]
pub enum BridgeMessagesWococoCall {
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
		bp_wococo::AccountId,
		bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<bp_wococo::Hash>,
		u32,
		Weight,
	),
	#[codec(index = 6)]
	receive_messages_delivery_proof(
		bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof<
			bp_wococo::Hash,
		>,
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
