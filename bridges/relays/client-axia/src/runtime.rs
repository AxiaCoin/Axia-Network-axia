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

//! Types that are specific to the Axia runtime.

use bp_messages::{LaneId, UnrewardedRelayersState};
use bp_axia_core::{AccountAddress, Balance, AxiaLike};
use bp_runtime::Chain;
use codec::{Compact, Decode, Encode};
use frame_support::weights::Weight;
use scale_info::TypeInfo;
use sp_runtime::FixedU128;

/// Unchecked Axia extrinsic.
pub type UncheckedExtrinsic = bp_axia_core::UncheckedExtrinsic<Call>;

/// AxiaTest account ownership digest from Axia.
///
/// The byte vector returned by this function should be signed with a AxiaTest account private key.
/// This way, the owner of `kusam_account_id` on Axia proves that the AxiaTest account private key
/// is also under his control.
pub fn axia_to_axctest_account_ownership_digest<Call, AccountId, SpecVersion>(
	axctest_call: &Call,
	kusam_account_id: AccountId,
	axctest_spec_version: SpecVersion,
) -> Vec<u8>
where
	Call: codec::Encode,
	AccountId: codec::Encode,
	SpecVersion: codec::Encode,
{
	pallet_bridge_dispatch::account_ownership_digest(
		axctest_call,
		kusam_account_id,
		axctest_spec_version,
		bp_runtime::AXIA_CHAIN_ID,
		bp_runtime::AXIATEST_CHAIN_ID,
	)
}

/// Axia Runtime `Call` enum.
///
/// The enum represents a subset of possible `Call`s we can send to Axia chain.
/// Ideally this code would be auto-generated from metadata, because we want to
/// avoid depending directly on the ENTIRE runtime just to get the encoding of `Dispatchable`s.
///
/// All entries here (like pretty much in the entire file) must be kept in sync with Axia
/// `construct_runtime`, so that we maintain SCALE-compatibility.
///
/// See: [link](https://github.com/axiatech/axctest/blob/master/runtime/kusam/src/lib.rs)
#[allow(clippy::large_enum_variant)]
#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum Call {
	/// System pallet.
	#[codec(index = 0)]
	System(SystemCall),
	/// Balances pallet.
	#[codec(index = 5)]
	Balances(BalancesCall),
	/// AxiaTest bridge pallet.
	#[codec(index = 110)]
	BridgeAxiaTestGrandpa(BridgeAxiaTestGrandpaCall),
	/// AxiaTest messages pallet.
	#[codec(index = 111)]
	BridgeAxiaTestMessages(BridgeAxiaTestMessagesCall),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
#[allow(non_camel_case_types)]
pub enum SystemCall {
	#[codec(index = 1)]
	remark(Vec<u8>),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
#[allow(non_camel_case_types)]
pub enum BalancesCall {
	#[codec(index = 0)]
	transfer(AccountAddress, Compact<Balance>),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
#[allow(non_camel_case_types)]
pub enum BridgeAxiaTestGrandpaCall {
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
pub enum BridgeAxiaTestMessagesCall {
	#[codec(index = 2)]
	update_pallet_parameter(BridgeAxiaTestMessagesParameter),
	#[codec(index = 3)]
	send_message(
		LaneId,
		bp_message_dispatch::MessagePayload<
			bp_axia::AccountId,
			bp_axctest::AccountId,
			bp_axctest::AccountPublic,
			Vec<u8>,
		>,
		bp_axia::Balance,
	),
	#[codec(index = 5)]
	receive_messages_proof(
		bp_axctest::AccountId,
		bridge_runtime_common::messages::target::FromBridgedChainMessagesProof<bp_axctest::Hash>,
		u32,
		Weight,
	),
	#[codec(index = 6)]
	receive_messages_delivery_proof(
		bridge_runtime_common::messages::source::FromBridgedChainMessagesDeliveryProof<
			bp_axctest::Hash,
		>,
		UnrewardedRelayersState,
	),
}

#[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, TypeInfo)]
pub enum BridgeAxiaTestMessagesParameter {
	#[codec(index = 0)]
	AxiaTestToAxiaConversionRate(FixedU128),
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
