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

//! Types used to connect to the AXIA chain.

use relay_axlib_client::{Chain, ChainBase};
use std::time::Duration;

/// AXIA header id.
pub type HeaderId = relay_utils::HeaderId<bp_axia::Hash, bp_axia::BlockNumber>;

/// AXIA chain definition
#[derive(Debug, Clone, Copy)]
pub struct AXIA;

impl ChainBase for AXIA {
	type BlockNumber = bp_axia::BlockNumber;
	type Hash = bp_axia::Hash;
	type Hasher = bp_axia::Hasher;
	type Header = bp_axia::Header;
}

impl Chain for AXIA {
	const NAME: &'static str = "AXIA";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

	type AccountId = bp_axia::AccountId;
	type Index = bp_axia::Nonce;
	type SignedBlock = bp_axia::SignedBlock;
	type Call = ();
	type Balance = bp_axia::Balance;
}

/// AXIA header type used in headers sync.
pub type SyncHeader = relay_axlib_client::SyncHeader<bp_axia::Header>;
