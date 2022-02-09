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

//! AlphaNet-to-Millau headers sync entrypoint.

use crate::finality_pipeline::{AxlibFinalitySyncPipeline, AxlibFinalityToAxlib};

use bp_header_chain::justification::GrandpaJustification;
use codec::Encode;
use relay_millau_client::{Millau, SigningParams as MillauSigningParams};
use relay_axlib_client::{Chain, TransactionSignScheme};
use relay_utils::metrics::MetricsParams;
use relay_alphanet_client::{SyncHeader as AlphaNetSyncHeader, AlphaNet};
use sp_core::{Bytes, Pair};

/// AlphaNet-to-Millau finality sync pipeline.
pub(crate) type AlphaNetFinalityToMillau = AxlibFinalityToAxlib<AlphaNet, Millau, MillauSigningParams>;

impl AxlibFinalitySyncPipeline for AlphaNetFinalityToMillau {
	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str = bp_alphanet::BEST_FINALIZED_ALPHANET_HEADER_METHOD;

	type TargetChain = Millau;

	fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
		crate::chains::add_axia_axiatest_price_metrics::<Self>(params)
	}

	fn transactions_author(&self) -> bp_millau::AccountId {
		(*self.target_sign.public().as_array_ref()).into()
	}

	fn make_submit_finality_proof_transaction(
		&self,
		transaction_nonce: <Millau as Chain>::Index,
		header: AlphaNetSyncHeader,
		proof: GrandpaJustification<bp_alphanet::Header>,
	) -> Bytes {
		let call = millau_runtime::BridgeGrandpaAlphaNetCall::<
			millau_runtime::Runtime,
			millau_runtime::AlphaNetGrandpaInstance,
		>::submit_finality_proof(header.into_inner(), proof)
		.into();

		let genesis_hash = *self.target_client.genesis_hash();
		let transaction = Millau::sign_transaction(genesis_hash, &self.target_sign, transaction_nonce, call);

		Bytes(transaction.encode())
	}
}
