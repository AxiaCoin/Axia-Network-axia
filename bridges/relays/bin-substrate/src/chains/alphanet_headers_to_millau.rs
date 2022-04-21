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

//! Alphanet-to-Millau headers sync entrypoint.

use codec::Encode;
use sp_core::{Bytes, Pair};

use bp_header_chain::justification::GrandpaJustification;
use relay_millau_client::{Millau, SigningParams as MillauSigningParams};
use relay_axlib_client::{Client, IndexOf, TransactionSignScheme, UnsignedTransaction};
use relay_utils::metrics::MetricsParams;
use relay_alphanet_client::{SyncHeader as AlphanetSyncHeader, Alphanet};
use axlib_relay_helper::finality_pipeline::{
	AxlibFinalitySyncPipeline, AxlibFinalityToAxlib,
};

/// Alphanet-to-Millau finality sync pipeline.
pub(crate) type FinalityPipelineAlphanetFinalityToMillau =
	AxlibFinalityToAxlib<Alphanet, Millau, MillauSigningParams>;

#[derive(Clone, Debug)]
pub(crate) struct AlphanetFinalityToMillau {
	finality_pipeline: FinalityPipelineAlphanetFinalityToMillau,
}

impl AlphanetFinalityToMillau {
	pub fn new(target_client: Client<Millau>, target_sign: MillauSigningParams) -> Self {
		Self {
			finality_pipeline: FinalityPipelineAlphanetFinalityToMillau::new(
				target_client,
				target_sign,
			),
		}
	}
}

impl AxlibFinalitySyncPipeline for AlphanetFinalityToMillau {
	type FinalitySyncPipeline = FinalityPipelineAlphanetFinalityToMillau;

	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
		bp_alphanet::BEST_FINALIZED_ALPHANET_HEADER_METHOD;

	type TargetChain = Millau;

	fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
		crate::chains::add_axia_axctest_price_metrics::<Self::FinalitySyncPipeline>(params)
	}

	fn transactions_author(&self) -> bp_millau::AccountId {
		(*self.finality_pipeline.target_sign.public().as_array_ref()).into()
	}

	fn make_submit_finality_proof_transaction(
		&self,
		era: bp_runtime::TransactionEraOf<Millau>,
		transaction_nonce: IndexOf<Millau>,
		header: AlphanetSyncHeader,
		proof: GrandpaJustification<bp_alphanet::Header>,
	) -> Bytes {
		let call = millau_runtime::BridgeGrandpaCall::<
			millau_runtime::Runtime,
			millau_runtime::AlphanetGrandpaInstance,
		>::submit_finality_proof {
			finality_target: Box::new(header.into_inner()),
			justification: proof,
		}
		.into();

		let genesis_hash = *self.finality_pipeline.target_client.genesis_hash();
		let transaction = Millau::sign_transaction(
			genesis_hash,
			&self.finality_pipeline.target_sign,
			era,
			UnsignedTransaction::new(call, transaction_nonce),
		);

		Bytes(transaction.encode())
	}
}
