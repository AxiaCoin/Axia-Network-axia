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

//! Autogenerated weights for `pallet_bridge_token_swap`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-06, STEPS: 50, REPEAT: 20
//! LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled
//! CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/millau-bridge-node
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bridge_token_swap
// --extrinsic=*
// --execution=wasm
// --wasm-execution=Compiled
// --heap-pages=4096
// --output=./modules/token-swap/src/weights.rs
// --template=./.maintain/millau-weight-template.hbs

#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for `pallet_bridge_token_swap`.
pub trait WeightInfo {
	fn create_swap() -> Weight;
	fn claim_swap() -> Weight;
	fn cancel_swap() -> Weight;
}

/// Weights for `pallet_bridge_token_swap` using the Millau node and recommended hardware.
pub struct MillauWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for MillauWeight<T> {
	fn create_swap() -> Weight {
		(116_040_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn claim_swap() -> Weight {
		(102_882_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn cancel_swap() -> Weight {
		(99_434_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn create_swap() -> Weight {
		(116_040_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn claim_swap() -> Weight {
		(102_882_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn cancel_swap() -> Weight {
		(99_434_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
