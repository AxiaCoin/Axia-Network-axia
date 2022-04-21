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

//! Types used to connect to the Rialto-Axlib chain.

use relay_axlib_client::{Chain, ChainBase};
use std::time::Duration;

/// Rialto header id.
pub type HeaderId =
	relay_utils::HeaderId<rialto_allychain_runtime::Hash, rialto_allychain_runtime::BlockNumber>;

/// Rialto allychain definition
#[derive(Debug, Clone, Copy)]
pub struct RialtoAllychain;

impl ChainBase for RialtoAllychain {
	type BlockNumber = rialto_allychain_runtime::BlockNumber;
	type Hash = rialto_allychain_runtime::Hash;
	type Hasher = rialto_allychain_runtime::Hashing;
	type Header = rialto_allychain_runtime::Header;

	type AccountId = rialto_allychain_runtime::AccountId;
	type Balance = rialto_allychain_runtime::Balance;
	type Index = rialto_allychain_runtime::Index;
	type Signature = rialto_allychain_runtime::Signature;
}

impl Chain for RialtoAllychain {
	const NAME: &'static str = "RialtoAllychain";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(5);
	const STORAGE_PROOF_OVERHEAD: u32 = bp_rialto::EXTRA_STORAGE_PROOF_SIZE;
	const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 = bp_rialto::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE;

	type SignedBlock = rialto_allychain_runtime::SignedBlock;
	type Call = rialto_allychain_runtime::Call;
	type WeightToFee = bp_rialto::WeightToFee;
}
