// Copyright 2020 AXIA Technologies (UK) Ltd.
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

//! Binary used for Simnet nodes, supports all runtimes, although only axia is implemented currently.
//! This binary accepts all the CLI args the axia binary does, Only difference is it uses
//! manual-seal™ and babe for block authorship, it has a no-op verifier, so all blocks received over the network
//! are imported and executed straight away. Block authorship/Finalization maybe done by calling the
//!  `engine_createBlock` & `engine_FinalizeBlock` rpc methods respectively.

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	axia_simnet::run(|node| async {
		node.until_shutdown().await;
		Ok(())
	})
}
