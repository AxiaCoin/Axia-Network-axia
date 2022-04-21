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

//! Types used to connect to the Alphanet chain.

use relay_axlib_client::{Chain, ChainBase, ChainWithBalances};
use sp_core::storage::StorageKey;
use std::time::Duration;

/// Alphanet header id.
pub type HeaderId = relay_utils::HeaderId<bp_alphanet::Hash, bp_alphanet::BlockNumber>;

/// Alphanet header type used in headers sync.
pub type SyncHeader = relay_axlib_client::SyncHeader<bp_alphanet::Header>;

/// Alphanet chain definition
#[derive(Debug, Clone, Copy)]
pub struct Alphanet;

impl ChainBase for Alphanet {
	type BlockNumber = bp_alphanet::BlockNumber;
	type Hash = bp_alphanet::Hash;
	type Hasher = bp_alphanet::Hasher;
	type Header = bp_alphanet::Header;

	type AccountId = bp_alphanet::AccountId;
	type Balance = bp_alphanet::Balance;
	type Index = bp_alphanet::Nonce;
	type Signature = bp_alphanet::Signature;
}

impl Chain for Alphanet {
	const NAME: &'static str = "Alphanet";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);
	const STORAGE_PROOF_OVERHEAD: u32 = bp_alphanet::EXTRA_STORAGE_PROOF_SIZE;
	const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 = bp_alphanet::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE;

	type SignedBlock = bp_alphanet::SignedBlock;
	type Call = bp_alphanet::Call;
	type WeightToFee = bp_alphanet::WeightToFee;
}

impl ChainWithBalances for Alphanet {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		StorageKey(bp_alphanet::account_info_storage_key(account_id))
	}
}
