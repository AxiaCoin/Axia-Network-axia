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

//! AxiaTest-to-Axia headers sync entrypoint.

use codec::Encode;
use sp_core::{Bytes, Pair};

use bp_header_chain::justification::GrandpaJustification;
use relay_axctest_client::{AxiaTest, SyncHeader as AxiaTestSyncHeader};
use relay_axia_client::{Axia, SigningParams as AxiaSigningParams};
use relay_axlib_client::{Client, TransactionSignScheme, UnsignedTransaction};
use relay_utils::metrics::MetricsParams;
use axlib_relay_helper::finality_pipeline::{
	AxlibFinalitySyncPipeline, AxlibFinalityToAxlib,
};

/// Maximal saturating difference between `balance(now)` and `balance(now-24h)` to treat
/// relay as gone wild.
///
/// Actual value, returned by `maximal_balance_decrease_per_day_is_sane` test is approximately 21
/// AXC, but let's round up to 30 AXC here.
pub(crate) const MAXIMAL_BALANCE_DECREASE_PER_DAY: bp_axia::Balance = 30_000_000_000;

/// AxiaTest-to-Axia finality sync pipeline.
pub(crate) type FinalityPipelineAxiaTestFinalityToAxia =
	AxlibFinalityToAxlib<AxiaTest, Axia, AxiaSigningParams>;

#[derive(Clone, Debug)]
pub(crate) struct AxiaTestFinalityToAxia {
	finality_pipeline: FinalityPipelineAxiaTestFinalityToAxia,
}

impl AxiaTestFinalityToAxia {
	pub fn new(target_client: Client<Axia>, target_sign: AxiaSigningParams) -> Self {
		Self {
			finality_pipeline: FinalityPipelineAxiaTestFinalityToAxia::new(
				target_client,
				target_sign,
			),
		}
	}
}

impl AxlibFinalitySyncPipeline for AxiaTestFinalityToAxia {
	type FinalitySyncPipeline = FinalityPipelineAxiaTestFinalityToAxia;

	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str =
		bp_axctest::BEST_FINALIZED_AXIATEST_HEADER_METHOD;

	type TargetChain = Axia;

	fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
		crate::chains::add_axia_axctest_price_metrics::<Self::FinalitySyncPipeline>(params)
	}

	fn start_relay_guards(&self) {
		relay_axlib_client::guard::abort_on_spec_version_change(
			self.finality_pipeline.target_client.clone(),
			bp_axia::VERSION.spec_version,
		);
		relay_axlib_client::guard::abort_when_account_balance_decreased(
			self.finality_pipeline.target_client.clone(),
			self.transactions_author(),
			MAXIMAL_BALANCE_DECREASE_PER_DAY,
		);
	}

	fn transactions_author(&self) -> bp_axia::AccountId {
		(*self.finality_pipeline.target_sign.public().as_array_ref()).into()
	}

	fn make_submit_finality_proof_transaction(
		&self,
		era: bp_runtime::TransactionEraOf<Axia>,
		transaction_nonce: bp_runtime::IndexOf<Axia>,
		header: AxiaTestSyncHeader,
		proof: GrandpaJustification<bp_axctest::Header>,
	) -> Bytes {
		let call = relay_axia_client::runtime::Call::BridgeAxiaTestGrandpa(
			relay_axia_client::runtime::BridgeAxiaTestGrandpaCall::submit_finality_proof(
				Box::new(header.into_inner()),
				proof,
			),
		);
		let genesis_hash = *self.finality_pipeline.target_client.genesis_hash();
		let transaction = Axia::sign_transaction(
			genesis_hash,
			&self.finality_pipeline.target_sign,
			era,
			UnsignedTransaction::new(call, transaction_nonce),
		);

		Bytes(transaction.encode())
	}
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use frame_support::weights::WeightToFeePolynomial;
	use pallet_bridge_grandpa::weights::WeightInfo;

	pub fn compute_maximal_balance_decrease_per_day<B, W>(expected_source_headers_per_day: u32) -> B
	where
		B: From<u32> + std::ops::Mul<Output = B>,
		W: WeightToFeePolynomial<Balance = B>,
	{
		// we assume that the GRANDPA is not lagging here => ancestry length will be near to 0
		// (let's round up to 2)
		const AVG_VOTES_ANCESTRIES_LEN: u32 = 2;
		// let's assume number of validators is 1024 (more than on any existing well-known chain
		// atm) => number of precommits is *2/3 + 1
		const AVG_PRECOMMITS_LEN: u32 = 1024 * 2 / 3 + 1;

		// GRANDPA pallet weights. We're now using Rialto weights everywhere.
		//
		// Using Rialto runtime is slightly incorrect, because `DbWeight` of other runtimes may
		// differ from the `DbWeight` of Rialto runtime. But now (and most probably forever) it is
		// the same.
		type GrandpaPalletWeights =
			pallet_bridge_grandpa::weights::RialtoWeight<rialto_runtime::Runtime>;

		// The following formula shall not be treated as super-accurate - guard is to protect from
		// mad relays, not to protect from over-average loses.

		// increase number of headers a bit
		let expected_source_headers_per_day = expected_source_headers_per_day * 110 / 100;
		let single_source_header_submit_call_weight = GrandpaPalletWeights::submit_finality_proof(
			AVG_VOTES_ANCESTRIES_LEN,
			AVG_PRECOMMITS_LEN,
		);
		// for simplicity - add extra weight for base tx fee + fee that is paid for the tx size +
		// adjusted fee
		let single_source_header_submit_tx_weight = single_source_header_submit_call_weight * 3 / 2;
		let single_source_header_tx_cost = W::calc(&single_source_header_submit_tx_weight);
		single_source_header_tx_cost * B::from(expected_source_headers_per_day)
	}

	#[test]
	fn maximal_balance_decrease_per_day_is_sane() {
		// we expect AxiaTest -> Axia relay to be running in mandatory-headers-only mode
		// => we expect single header for every AxiaTest session
		let maximal_balance_decrease = compute_maximal_balance_decrease_per_day::<
			bp_axia::Balance,
			bp_axia::WeightToFee,
		>(bp_axctest::DAYS / bp_axctest::SESSION_LENGTH + 1);
		assert!(
			MAXIMAL_BALANCE_DECREASE_PER_DAY >= maximal_balance_decrease,
			"Maximal expected loss per day {} is larger than hardcoded {}",
			maximal_balance_decrease,
			MAXIMAL_BALANCE_DECREASE_PER_DAY,
		);
	}
}
