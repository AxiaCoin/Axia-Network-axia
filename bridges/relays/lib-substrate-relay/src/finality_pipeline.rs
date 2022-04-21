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

//! Axlib-to-Axlib headers sync entrypoint.

use crate::{finality_target::AxlibFinalityTarget, STALL_TIMEOUT};

use bp_header_chain::justification::GrandpaJustification;
use bp_runtime::AccountIdOf;
use finality_relay::{FinalitySyncParams, FinalitySyncPipeline};
use relay_axlib_client::{
	finality_source::FinalitySource, BlockNumberOf, Chain, Client, HashOf, SyncHeader,
};
use relay_utils::{metrics::MetricsParams, BlockNumberBase};
use sp_core::Bytes;
use std::{fmt::Debug, marker::PhantomData};

/// Default limit of recent finality proofs.
///
/// Finality delay of 4096 blocks is unlikely to happen in practice in
/// Axlib+GRANDPA based chains (good to know).
pub(crate) const RECENT_FINALITY_PROOFS_LIMIT: usize = 4096;

/// Headers sync pipeline for Axlib <-> Axlib relays.
pub trait AxlibFinalitySyncPipeline: 'static + Clone + Debug + Send + Sync {
	/// Pipeline for syncing finalized Source chain headers to Target chain.
	type FinalitySyncPipeline: FinalitySyncPipeline;

	/// Name of the runtime method that returns id of best finalized source header at target chain.
	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str;

	/// Chain with GRANDPA bridge pallet.
	type TargetChain: Chain;

	/// Customize metrics exposed by headers sync loop.
	fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
		Ok(params)
	}

	/// Start finality relay guards.
	///
	/// Different finality bridges may have different set of guards - e.g. on ephemeral chains we
	/// don't need a version guards, on test chains we don't care that much about relayer account
	/// balance, ... So the implementation is left to the specific bridges.
	fn start_relay_guards(&self) {}

	/// Returns id of account that we're using to sign transactions at target chain.
	fn transactions_author(&self) -> AccountIdOf<Self::TargetChain>;

	/// Make submit header transaction.
	fn make_submit_finality_proof_transaction(
		&self,
		era: bp_runtime::TransactionEraOf<Self::TargetChain>,
		transaction_nonce: bp_runtime::IndexOf<Self::TargetChain>,
		header: <Self::FinalitySyncPipeline as FinalitySyncPipeline>::Header,
		proof: <Self::FinalitySyncPipeline as FinalitySyncPipeline>::FinalityProof,
	) -> Bytes;
}

/// Axlib-to-Axlib finality proof pipeline.
#[derive(Clone)]
pub struct AxlibFinalityToAxlib<SourceChain, TargetChain: Chain, TargetSign> {
	/// Client for the target chain.
	pub target_client: Client<TargetChain>,
	/// Data required to sign target chain transactions.
	pub target_sign: TargetSign,
	/// Unused generic arguments dump.
	_marker: PhantomData<SourceChain>,
}

impl<SourceChain, TargetChain: Chain, TargetSign> Debug
	for AxlibFinalityToAxlib<SourceChain, TargetChain, TargetSign>
{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		f.debug_struct("AxlibFinalityToAxlib")
			.field("target_client", &self.target_client)
			.finish()
	}
}

impl<SourceChain, TargetChain: Chain, TargetSign>
	AxlibFinalityToAxlib<SourceChain, TargetChain, TargetSign>
{
	/// Create new Axlib-to-Axlib headers pipeline.
	pub fn new(target_client: Client<TargetChain>, target_sign: TargetSign) -> Self {
		AxlibFinalityToAxlib { target_client, target_sign, _marker: Default::default() }
	}
}

impl<SourceChain, TargetChain, TargetSign> FinalitySyncPipeline
	for AxlibFinalityToAxlib<SourceChain, TargetChain, TargetSign>
where
	SourceChain: Clone + Chain + Debug,
	BlockNumberOf<SourceChain>: BlockNumberBase,
	TargetChain: Clone + Chain + Debug,
	TargetSign: 'static + Clone + Send + Sync,
{
	const SOURCE_NAME: &'static str = SourceChain::NAME;
	const TARGET_NAME: &'static str = TargetChain::NAME;

	type Hash = HashOf<SourceChain>;
	type Number = BlockNumberOf<SourceChain>;
	type Header = SyncHeader<SourceChain::Header>;
	type FinalityProof = GrandpaJustification<SourceChain::Header>;
}

/// Run Axlib-to-Axlib finality sync.
pub async fn run<SourceChain, TargetChain, P>(
	pipeline: P,
	source_client: Client<SourceChain>,
	target_client: Client<TargetChain>,
	only_mandatory_headers: bool,
	transactions_mortality: Option<u32>,
	metrics_params: MetricsParams,
) -> anyhow::Result<()>
where
	P: AxlibFinalitySyncPipeline<TargetChain = TargetChain>,
	P::FinalitySyncPipeline: FinalitySyncPipeline<
		Hash = HashOf<SourceChain>,
		Number = BlockNumberOf<SourceChain>,
		Header = SyncHeader<SourceChain::Header>,
		FinalityProof = GrandpaJustification<SourceChain::Header>,
	>,
	SourceChain: Clone + Chain,
	BlockNumberOf<SourceChain>: BlockNumberBase,
	TargetChain: Clone + Chain,
{
	log::info!(
		target: "bridge",
		"Starting {} -> {} finality proof relay",
		SourceChain::NAME,
		TargetChain::NAME,
	);

	finality_relay::run(
		FinalitySource::new(source_client, None),
		AxlibFinalityTarget::new(target_client, pipeline, transactions_mortality),
		FinalitySyncParams {
			tick: std::cmp::max(
				SourceChain::AVERAGE_BLOCK_INTERVAL,
				TargetChain::AVERAGE_BLOCK_INTERVAL,
			),
			recent_finality_proofs_limit: RECENT_FINALITY_PROOFS_LIMIT,
			stall_timeout: relay_axlib_client::transaction_stall_timeout(
				transactions_mortality,
				TargetChain::AVERAGE_BLOCK_INTERVAL,
				STALL_TIMEOUT,
			),
			only_mandatory_headers,
		},
		metrics_params,
		futures::future::pending(),
	)
	.await
	.map_err(|e| anyhow::format_err!("{}", e))
}
