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

//! Allychains support in Rialto runtime.

use crate::{AccountId, Balance, Balances, BlockNumber, Event, Origin, Registrar, Runtime, Slots};

use frame_support::{parameter_types, weights::Weight};
use frame_system::EnsureRoot;
use axia_primitives::v1::ValidatorIndex;
use axia_runtime_common::{paras_registrar, paras_sudo_wrapper, slots};
use axia_runtime_allychains::{
	configuration as allychains_configuration, dmp as allychains_dmp, hrmp as allychains_hrmp,
	inclusion as allychains_inclusion, initializer as allychains_initializer,
	origin as allychains_origin, paras as allychains_paras,
	paras_inherent as allychains_paras_inherent, scheduler as allychains_scheduler,
	session_info as allychains_session_info, shared as allychains_shared, ump as allychains_ump,
};

/// Special `RewardValidators` that does nothing ;)
pub struct RewardValidators;
impl axia_runtime_allychains::inclusion::RewardValidators for RewardValidators {
	fn reward_backing(_: impl IntoIterator<Item = ValidatorIndex>) {}
	fn reward_bitfields(_: impl IntoIterator<Item = ValidatorIndex>) {}
}

// all required allychain modules from `axia-runtime-allychains` crate

impl allychains_configuration::Config for Runtime {
	type WeightInfo = allychains_configuration::TestWeightInfo;
}

impl allychains_dmp::Config for Runtime {}

impl allychains_hrmp::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type Currency = Balances;
}

impl allychains_inclusion::Config for Runtime {
	type Event = Event;
	type RewardValidators = RewardValidators;
	type DisputesHandler = ();
}

impl allychains_initializer::Config for Runtime {
	type Randomness = pallet_babe::RandomnessFromOneEpochAgo<Runtime>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
}

impl allychains_origin::Config for Runtime {}

impl allychains_paras::Config for Runtime {
	type Origin = Origin;
	type Event = Event;
	type WeightInfo = allychains_paras::TestWeightInfo;
}

impl allychains_paras_inherent::Config for Runtime {
	type WeightInfo = allychains_paras_inherent::TestWeightInfo;
}

impl allychains_scheduler::Config for Runtime {}

impl allychains_session_info::Config for Runtime {}

impl allychains_shared::Config for Runtime {}

parameter_types! {
	pub const FirstMessageFactorPercent: u64 = 100;
}

impl allychains_ump::Config for Runtime {
	type Event = Event;
	type UmpSink = ();
	type FirstMessageFactorPercent = FirstMessageFactorPercent;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

// required onboarding pallets. We're not going to use auctions or crowdloans, so they're missing

parameter_types! {
	pub const ParaDeposit: Balance = 0;
	pub const DataDepositPerByte: Balance = 0;
}

impl paras_registrar::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type Currency = Balances;
	type OnSwap = Slots;
	type ParaDeposit = ParaDeposit;
	type DataDepositPerByte = DataDepositPerByte;
	type WeightInfo = paras_registrar::TestWeightInfo;
}

parameter_types! {
	pub const LeasePeriod: BlockNumber = 10 * bp_rialto::MINUTES;
}

impl slots::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type Registrar = Registrar;
	type LeasePeriod = LeasePeriod;
	type WeightInfo = slots::TestWeightInfo;
	type LeaseOffset = ();
}

impl paras_sudo_wrapper::Config for Runtime {}

pub struct ZeroWeights;

impl axia_runtime_common::paras_registrar::WeightInfo for ZeroWeights {
	fn reserve() -> Weight {
		0
	}
	fn register() -> Weight {
		0
	}
	fn force_register() -> Weight {
		0
	}
	fn deregister() -> Weight {
		0
	}
	fn swap() -> Weight {
		0
	}
}

impl axia_runtime_common::slots::WeightInfo for ZeroWeights {
	fn force_lease() -> Weight {
		0
	}
	fn manage_lease_period_start(_c: u32, _t: u32) -> Weight {
		0
	}
	fn clear_all_leases() -> Weight {
		0
	}
	fn trigger_onboard() -> Weight {
		0
	}
}
