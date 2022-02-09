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

//! Types used to connect to the AlphaNet chain.

use codec::Encode;
use relay_axlib_client::{Chain, ChainBase, ChainWithBalances, TransactionSignScheme};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};
use std::time::Duration;

/// AlphaNet header id.
pub type HeaderId = relay_utils::HeaderId<bp_alphanet::Hash, bp_alphanet::BlockNumber>;

/// AlphaNet header type used in headers sync.
pub type SyncHeader = relay_axlib_client::SyncHeader<bp_alphanet::Header>;

/// AlphaNet chain definition
#[derive(Debug, Clone, Copy)]
pub struct AlphaNet;

impl ChainBase for AlphaNet {
	type BlockNumber = bp_alphanet::BlockNumber;
	type Hash = bp_alphanet::Hash;
	type Hasher = bp_alphanet::Hasher;
	type Header = bp_alphanet::Header;
}

impl Chain for AlphaNet {
	const NAME: &'static str = "AlphaNet";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

	type AccountId = bp_alphanet::AccountId;
	type Index = bp_alphanet::Nonce;
	type SignedBlock = bp_alphanet::SignedBlock;
	type Call = bp_alphanet::Call;
	type Balance = bp_alphanet::Balance;
}

impl ChainWithBalances for AlphaNet {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		StorageKey(bp_alphanet::account_info_storage_key(account_id))
	}
}

impl TransactionSignScheme for AlphaNet {
	type Chain = AlphaNet;
	type AccountKeyPair = sp_core::sr25519::Pair;
	type SignedTransaction = bp_alphanet::UncheckedExtrinsic;

	fn sign_transaction(
		genesis_hash: <Self::Chain as ChainBase>::Hash,
		signer: &Self::AccountKeyPair,
		signer_nonce: <Self::Chain as Chain>::Index,
		call: <Self::Chain as Chain>::Call,
	) -> Self::SignedTransaction {
		let raw_payload = SignedPayload::new(
			call,
			bp_alphanet::SignedExtensions::new(
				bp_alphanet::VERSION,
				sp_runtime::generic::Era::Immortal,
				genesis_hash,
				signer_nonce,
				0,
			),
		)
		.expect("SignedExtension never fails.");

		let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
		let signer: sp_runtime::MultiSigner = signer.public().into();
		let (call, extra, _) = raw_payload.deconstruct();

		bp_alphanet::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::MultiAddress::Id(signer.into_account()),
			signature.into(),
			extra,
		)
	}
}

/// AlphaNet signing params.
pub type SigningParams = sp_core::sr25519::Pair;
