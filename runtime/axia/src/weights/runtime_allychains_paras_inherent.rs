// Copyright 2017-2021 Axia Technologies (UK) Ltd.
// This file is part of Axia.

// Axia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_allychains::paras_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-10, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("axia-dev"), DB CACHE: 128

// Executed Command:
// target/release/axia
// benchmark
// --chain=axia-dev
// --steps=50
// --repeat=20
// --pallet=runtime_allychains::paras_inherent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/axia/src/weights/runtime_allychains_paras_inherent.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_allychains::paras_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_allychains::paras_inherent::WeightInfo for WeightInfo<T> {
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Allychains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: ParasShared ActiveValidatorIndices (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler AllythreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInherent OnChainVotes (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	fn enter_variable_disputes(v: u32, ) -> Weight {
		(404_051_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((309_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(23 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Allychains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: ParasShared ActiveValidatorIndices (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler AllythreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInclusion AvailabilityBitfields (r:0 w:1)
	// Storage: ParaInherent OnChainVotes (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	fn enter_bitfields() -> Weight {
		(446_635_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(23 as Weight))
			.saturating_add(T::DbWeight::get().writes(15 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Allychains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: ParasShared ActiveValidatorIndices (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler AllythreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInherent OnChainVotes (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		(1_103_699_000 as Weight)
			// Standard Error: 29_000
			.saturating_add((49_128_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(25 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
	// Storage: ParaInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: ParaScheduler AvailabilityCores (r:1 w:1)
	// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Paras Allychains (r:1 w:0)
	// Storage: ParaInclusion PendingAvailability (r:2 w:1)
	// Storage: ParaInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Session Validators (r:1 w:0)
	// Storage: ParasShared ActiveValidatorIndices (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	// Storage: ParaScheduler SessionStartBlock (r:1 w:0)
	// Storage: ParaScheduler AllythreadQueue (r:1 w:1)
	// Storage: ParaScheduler Scheduled (r:1 w:1)
	// Storage: ParaScheduler ValidatorGroups (r:1 w:0)
	// Storage: Paras CurrentCodeHash (r:1 w:0)
	// Storage: Paras FutureCodeHash (r:1 w:0)
	// Storage: Paras UpgradeRestrictionSignal (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: ParaInherent OnChainVotes (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Paras Heads (r:0 w:1)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		(42_700_804_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(27 as Weight))
			.saturating_add(T::DbWeight::get().writes(14 as Weight))
	}
}