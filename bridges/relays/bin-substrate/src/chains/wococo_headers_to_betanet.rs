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

//! Wococo-to-Betanet headers sync entrypoint.

use codec::Encode;
use sp_core::{Bytes, Pair};

use bp_header_chain::justification::GrandpaJustification;
use relay_betanet_client::{Betanet, SigningParams as BetanetSigningParams};
use relay_axlib_client::{Client, IndexOf, TransactionSignScheme, UnsignedTransaction};
use relay_utils::metrics::MetricsParams;
use relay_wococo_client::{SyncHeader as WococoSyncHeader, Wococo};
use axlib_relay_helper::finality_pipeline::{
	AxlibFinalitySyncPipeline, AxlibFinalityToAxlib,
};

/// Maximal saturating difference between `balance(now)` and `balance(now-24h)` to treat
/// relay as gone wild.
///
/// See `maximal_balance_decrease_per_day_is_sane` test for details.
/// Note that this is in plancks, so this corresponds to `1500 UNITS`.
pub(crate) const MAXIMAL_BALANCE_DECREASE_PER_DAY: bp_betanet::Balance = 1_500_000_000_000_000;

/// Wococo-to-Betanet finality sync pipeline.
pub(crate) type FinalityPipelineWococoFinalityToBetanet =
	AxlibFinalityToAxlib<Wococo, Betanet, BetanetSigningParams>;

#[derive(Clone, Debug)]
pub(crate) struct WococoFinalityToBetanet {
	finality_pipeline: FinalityPipelineWococoFinalityToBetanet,
}

impl WococoFinalityToBetanet {
	pub fn new(target_client: Client<Betanet>, target_sign: BetanetSigningParams) -> Self {
		Self {
			finality_pipeline: FinalityPipelineWococoFinalityToBetanet::new(
				target_client,
				target_sign,
			),
		}
	}
}

impl AxlibFinalitySyncPipeline for WococoFinalityToBetanet {
	type FinalitySyncPipeline = FinalityPipelineWococoFinalityToBetanet;

	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
		bp_wococo::BEST_FINALIZED_WOCOCO_HEADER_METHOD;

	type TargetChain = Betanet;

	fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
		crate::chains::add_axia_axctest_price_metrics::<Self::FinalitySyncPipeline>(params)
	}

	fn start_relay_guards(&self) {
		relay_axlib_client::guard::abort_on_spec_version_change(
			self.finality_pipeline.target_client.clone(),
			bp_betanet::VERSION.spec_version,
		);
		relay_axlib_client::guard::abort_when_account_balance_decreased(
			self.finality_pipeline.target_client.clone(),
			self.transactions_author(),
			MAXIMAL_BALANCE_DECREASE_PER_DAY,
		);
	}

	fn transactions_author(&self) -> bp_betanet::AccountId {
		(*self.finality_pipeline.target_sign.public().as_array_ref()).into()
	}

	fn make_submit_finality_proof_transaction(
		&self,
		era: bp_runtime::TransactionEraOf<Betanet>,
		transaction_nonce: IndexOf<Betanet>,
		header: WococoSyncHeader,
		proof: GrandpaJustification<bp_wococo::Header>,
	) -> Bytes {
		let call = relay_betanet_client::runtime::Call::BridgeGrandpaWococo(
			relay_betanet_client::runtime::BridgeGrandpaWococoCall::submit_finality_proof(
				Box::new(header.into_inner()),
				proof,
			),
		);
		let genesis_hash = *self.finality_pipeline.target_client.genesis_hash();
		let transaction = Betanet::sign_transaction(
			genesis_hash,
			&self.finality_pipeline.target_sign,
			era,
			UnsignedTransaction::new(call, transaction_nonce),
		);

		Bytes(transaction.encode())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::chains::axctest_headers_to_axia::tests::compute_maximal_balance_decrease_per_day;

	#[test]
	fn maximal_balance_decrease_per_day_is_sane() {
		// we expect Wococo -> Betanet relay to be running in all-headers mode
		let maximal_balance_decrease = compute_maximal_balance_decrease_per_day::<
			bp_axctest::Balance,
			bp_axctest::WeightToFee,
		>(bp_wococo::DAYS);
		assert!(
			MAXIMAL_BALANCE_DECREASE_PER_DAY >= maximal_balance_decrease,
			"Maximal expected loss per day {} is larger than hardcoded {}",
			maximal_balance_decrease,
			MAXIMAL_BALANCE_DECREASE_PER_DAY,
		);
	}
}
