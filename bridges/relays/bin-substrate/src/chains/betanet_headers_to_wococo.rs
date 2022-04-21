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

//! Betanet-to-Wococo headers sync entrypoint.

use codec::Encode;
use sp_core::{Bytes, Pair};

use bp_header_chain::justification::GrandpaJustification;
use relay_betanet_client::{Betanet, SyncHeader as BetanetSyncHeader};
use relay_axlib_client::{Client, IndexOf, TransactionSignScheme, UnsignedTransaction};
use relay_utils::metrics::MetricsParams;
use relay_wococo_client::{SigningParams as WococoSigningParams, Wococo};
use axlib_relay_helper::finality_pipeline::{
	AxlibFinalitySyncPipeline, AxlibFinalityToAxlib,
};

use crate::chains::wococo_headers_to_betanet::MAXIMAL_BALANCE_DECREASE_PER_DAY;

/// Betanet-to-Wococo finality sync pipeline.
pub(crate) type FinalityPipelineBetanetFinalityToWococo =
	AxlibFinalityToAxlib<Betanet, Wococo, WococoSigningParams>;

#[derive(Clone, Debug)]
pub(crate) struct BetanetFinalityToWococo {
	finality_pipeline: FinalityPipelineBetanetFinalityToWococo,
}

impl BetanetFinalityToWococo {
	pub fn new(target_client: Client<Wococo>, target_sign: WococoSigningParams) -> Self {
		Self {
			finality_pipeline: FinalityPipelineBetanetFinalityToWococo::new(
				target_client,
				target_sign,
			),
		}
	}
}

impl AxlibFinalitySyncPipeline for BetanetFinalityToWococo {
	type FinalitySyncPipeline = FinalityPipelineBetanetFinalityToWococo;

	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
		bp_betanet::BEST_FINALIZED_BETANET_HEADER_METHOD;

	type TargetChain = Wococo;

	fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
		crate::chains::add_axia_axctest_price_metrics::<Self::FinalitySyncPipeline>(params)
	}

	fn start_relay_guards(&self) {
		relay_axlib_client::guard::abort_on_spec_version_change(
			self.finality_pipeline.target_client.clone(),
			bp_wococo::VERSION.spec_version,
		);
		relay_axlib_client::guard::abort_when_account_balance_decreased(
			self.finality_pipeline.target_client.clone(),
			self.transactions_author(),
			MAXIMAL_BALANCE_DECREASE_PER_DAY,
		);
	}

	fn transactions_author(&self) -> bp_wococo::AccountId {
		(*self.finality_pipeline.target_sign.public().as_array_ref()).into()
	}

	fn make_submit_finality_proof_transaction(
		&self,
		era: bp_runtime::TransactionEraOf<Wococo>,
		transaction_nonce: IndexOf<Wococo>,
		header: BetanetSyncHeader,
		proof: GrandpaJustification<bp_betanet::Header>,
	) -> Bytes {
		let call = relay_wococo_client::runtime::Call::BridgeGrandpaBetanet(
			relay_wococo_client::runtime::BridgeGrandpaBetanetCall::submit_finality_proof(
				Box::new(header.into_inner()),
				proof,
			),
		);
		let genesis_hash = *self.finality_pipeline.target_client.genesis_hash();
		let transaction = Wococo::sign_transaction(
			genesis_hash,
			&self.finality_pipeline.target_sign,
			era,
			UnsignedTransaction::new(call, transaction_nonce),
		);

		Bytes(transaction.encode())
	}
}
