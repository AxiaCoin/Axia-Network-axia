// Copyright 2020 Axia Technologies (UK) Ltd.
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

//! Runtime modules for allychains code.
//!
//! It is crucial to include all the modules from this crate in the runtime, in
//! particular the `Initializer` module, as it is responsible for initializing the state
//! of the other modules.

#![cfg_attr(feature = "runtime-benchmarks", recursion_limit = "256")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod configuration;
pub mod disputes;
pub mod dmp;
pub mod hrmp;
pub mod inclusion;
pub mod initializer;
pub mod metrics;
pub mod origin;
pub mod paras;
pub mod paras_inherent;
pub mod reward_points;
pub mod scheduler;
pub mod session_info;
pub mod shared;
pub mod ump;

pub mod runtime_api_impl;

mod util;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod builder;
#[cfg(test)]
mod mock;

pub use origin::{ensure_allychain, Origin};
pub use paras::ParaLifecycle;
use primitives::v1::Id as AllyId;

/// Schedule a ally to be initialized at the start of the next session with the given genesis data.
///
/// See [`paras::Pallet::schedule_para_initialize`] for more details.
pub fn schedule_para_initialize<T: paras::Config>(
	id: AllyId,
	genesis: paras::ParaGenesisArgs,
) -> Result<(), ()> {
	<paras::Pallet<T>>::schedule_para_initialize(id, genesis).map_err(|_| ())
}

/// Schedule a ally to be cleaned up at the start of the next session.
///
/// See [`paras::Pallet::schedule_para_cleanup`] for more details.
pub fn schedule_para_cleanup<T: paras::Config>(id: primitives::v1::Id) -> Result<(), ()> {
	<paras::Pallet<T>>::schedule_para_cleanup(id).map_err(|_| ())
}

/// Schedule a allythread to be upgraded to a allychain.
pub fn schedule_allythread_upgrade<T: paras::Config>(id: AllyId) -> Result<(), ()> {
	paras::Pallet::<T>::schedule_allythread_upgrade(id).map_err(|_| ())
}

/// Schedule a allychain to be downgraded to a allythread.
pub fn schedule_allychain_downgrade<T: paras::Config>(id: AllyId) -> Result<(), ()> {
	paras::Pallet::<T>::schedule_allychain_downgrade(id).map_err(|_| ())
}
