// Copyright 2019-2020 AXIA Technologies (UK) Ltd.
// This file is part of AXIA.

// AXIA is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA.  If not, see <http://www.gnu.org/licenses/>.

//! Utilities for writing allychain WASM.

/// Load the validation params from memory when implementing a Rust allychain.
///
/// Offset and length must have been provided by the validation
/// function's entry point.
#[cfg(not(feature = "std"))]
pub unsafe fn load_params(params: *const u8, len: usize) -> crate::primitives::ValidationParams {
	let mut slice = sp_std::slice::from_raw_parts(params, len);

	axia_scale_codec::Decode::decode(&mut slice).expect("Invalid input data")
}

/// Allocate the validation result in memory, getting the return-pointer back.
///
/// As described in the crate docs, this is a pointer to the appended length
/// of the vector.
#[cfg(not(feature = "std"))]
pub fn write_result(result: &crate::primitives::ValidationResult) -> u64 {
	sp_core::to_axlib_wasm_fn_return_value(&result)
}
