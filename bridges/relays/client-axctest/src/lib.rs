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

//! Types used to connect to the AxiaTest chain.

use codec::Encode;
use relay_axlib_client::{
	Chain, ChainBase, ChainWithBalances, TransactionEraOf, TransactionSignScheme,
	UnsignedTransaction,
};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};
use std::time::Duration;

pub mod runtime;

/// AxiaTest header id.
pub type HeaderId = relay_utils::HeaderId<bp_axctest::Hash, bp_axctest::BlockNumber>;

/// AxiaTest chain definition
#[derive(Debug, Clone, Copy)]
pub struct AxiaTest;

impl ChainBase for AxiaTest {
	type BlockNumber = bp_axctest::BlockNumber;
	type Hash = bp_axctest::Hash;
	type Hasher = bp_axctest::Hasher;
	type Header = bp_axctest::Header;

	type AccountId = bp_axctest::AccountId;
	type Balance = bp_axctest::Balance;
	type Index = bp_axctest::Nonce;
	type Signature = bp_axctest::Signature;
}

impl Chain for AxiaTest {
	const NAME: &'static str = "AxiaTest";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);
	const STORAGE_PROOF_OVERHEAD: u32 = bp_axctest::EXTRA_STORAGE_PROOF_SIZE;
	const MAXIMAL_ENCODED_ACCOUNT_ID_SIZE: u32 = bp_axctest::MAXIMAL_ENCODED_ACCOUNT_ID_SIZE;

	type SignedBlock = bp_axctest::SignedBlock;
	type Call = crate::runtime::Call;
	type WeightToFee = bp_axctest::WeightToFee;
}

impl ChainWithBalances for AxiaTest {
	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		StorageKey(bp_axctest::account_info_storage_key(account_id))
	}
}

impl TransactionSignScheme for AxiaTest {
	type Chain = AxiaTest;
	type AccountKeyPair = sp_core::sr25519::Pair;
	type SignedTransaction = crate::runtime::UncheckedExtrinsic;

	fn sign_transaction(
		genesis_hash: <Self::Chain as ChainBase>::Hash,
		signer: &Self::AccountKeyPair,
		era: TransactionEraOf<Self::Chain>,
		unsigned: UnsignedTransaction<Self::Chain>,
	) -> Self::SignedTransaction {
		let raw_payload = SignedPayload::new(
			unsigned.call,
			bp_axctest::SignedExtensions::new(
				bp_axctest::VERSION,
				era,
				genesis_hash,
				unsigned.nonce,
				unsigned.tip,
			),
		)
		.expect("SignedExtension never fails.");

		let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
		let signer: sp_runtime::MultiSigner = signer.public().into();
		let (call, extra, _) = raw_payload.deconstruct();

		bp_axctest::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::MultiAddress::Id(signer.into_account()),
			signature.into(),
			extra,
		)
	}

	fn is_signed(tx: &Self::SignedTransaction) -> bool {
		tx.signature.is_some()
	}

	fn is_signed_by(signer: &Self::AccountKeyPair, tx: &Self::SignedTransaction) -> bool {
		tx.signature
			.as_ref()
			.map(|(address, _, _)| {
				*address == bp_axctest::AccountId::from(*signer.public().as_array_ref()).into()
			})
			.unwrap_or(false)
	}

	fn parse_transaction(tx: Self::SignedTransaction) -> Option<UnsignedTransaction<Self::Chain>> {
		let extra = &tx.signature.as_ref()?.2;
		Some(UnsignedTransaction { call: tx.function, nonce: extra.nonce(), tip: extra.tip() })
	}
}

/// AxiaTest header type used in headers sync.
pub type SyncHeader = relay_axlib_client::SyncHeader<bp_axctest::Header>;

/// AxiaTest signing params.
pub type SigningParams = sp_core::sr25519::Pair;
