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

//! Types used to connect to the BetaNet-Axlib chain.

use codec::Encode;
use relay_axlib_client::{Chain, ChainBase, ChainWithBalances, TransactionSignScheme};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};
use std::time::Duration;

pub mod runtime;

/// BetaNet header id.
pub type HeaderId = relay_utils::HeaderId<bp_betanet::Hash, bp_betanet::BlockNumber>;

/// BetaNet header type used in headers sync.
pub type SyncHeader = relay_axlib_client::SyncHeader<bp_betanet::Header>;

/// BetaNet chain definition
#[derive(Debug, Clone, Copy)]
pub struct BetaNet;

impl ChainBase for BetaNet {
	type BlockNumber = bp_betanet::BlockNumber;
	type Hash = bp_betanet::Hash;
	type Hasher = bp_betanet::Hashing;
	type Header = bp_betanet::Header;
}

impl Chain for BetaNet {
	const NAME: &'static str = "BetaNet";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

	type AccountId = bp_betanet::AccountId;
	type Index = bp_betanet::Index;
	type SignedBlock = bp_betanet::SignedBlock;
	type Call = crate::runtime::Call;
	type Balance = bp_betanet::Balance;
}

impl ChainWithBalances for BetaNet {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		StorageKey(bp_betanet::account_info_storage_key(account_id))
	}
}

impl TransactionSignScheme for BetaNet {
	type Chain = BetaNet;
	type AccountKeyPair = sp_core::sr25519::Pair;
	type SignedTransaction = crate::runtime::UncheckedExtrinsic;

	fn sign_transaction(
		genesis_hash: <Self::Chain as ChainBase>::Hash,
		signer: &Self::AccountKeyPair,
		signer_nonce: <Self::Chain as Chain>::Index,
		call: <Self::Chain as Chain>::Call,
	) -> Self::SignedTransaction {
		let raw_payload = SignedPayload::new(
			call,
			bp_betanet::SignedExtensions::new(
				bp_betanet::VERSION,
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

		bp_betanet::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::MultiAddress::Id(signer.into_account()),
			signature.into(),
			extra,
		)
	}
}

/// BetaNet signing params.
pub type SigningParams = sp_core::sr25519::Pair;
