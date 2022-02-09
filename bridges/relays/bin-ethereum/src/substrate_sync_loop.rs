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

//! Rialto-Axlib -> Ethereum PoA synchronization.

use crate::ethereum_client::EthereumHighLevelRpc;
use crate::rpc_errors::RpcError;

use async_trait::async_trait;
use codec::Encode;
use headers_relay::{
	sync::HeadersSyncParams,
	sync_loop::TargetClient,
	sync_types::{HeadersSyncPipeline, QueuedHeader, SourceHeader, SubmittedHeaders},
};
use relay_ethereum_client::{
	types::Address, Client as EthereumClient, ConnectionParams as EthereumConnectionParams,
	SigningParams as EthereumSigningParams,
};
use relay_rialto_client::{HeaderId as RialtoHeaderId, Rialto, SyncHeader as RialtoSyncHeader};
use relay_axlib_client::{
	headers_source::HeadersSource, Chain as AxlibChain, Client as AxlibClient,
	ConnectionParams as AxlibConnectionParams,
};
use relay_utils::{metrics::MetricsParams, relay_loop::Client as RelayClient};
use sp_runtime::EncodedJustification;

use std::fmt::Debug;
use std::{collections::HashSet, time::Duration};

pub mod consts {
	use super::*;

	/// Interval at which we check new Ethereum blocks.
	pub const ETHEREUM_TICK_INTERVAL: Duration = Duration::from_secs(5);
	/// Max Ethereum headers we want to have in all 'before-submitted' states.
	pub const MAX_FUTURE_HEADERS_TO_DOWNLOAD: usize = 8;
	/// Max Ethereum headers count we want to have in 'submitted' state.
	pub const MAX_SUBMITTED_HEADERS: usize = 4;
	/// Max depth of in-memory headers in all states. Past this depth they will be forgotten (pruned).
	pub const PRUNE_DEPTH: u32 = 256;
}

/// Axlib synchronization parameters.
#[derive(Debug)]
pub struct AxlibSyncParams {
	/// Axlib connection params.
	pub sub_params: AxlibConnectionParams,
	/// Ethereum connection params.
	pub eth_params: EthereumConnectionParams,
	/// Ethereum signing params.
	pub eth_sign: EthereumSigningParams,
	/// Ethereum bridge contract address.
	pub eth_contract_address: Address,
	/// Synchronization parameters.
	pub sync_params: HeadersSyncParams,
	/// Metrics parameters.
	pub metrics_params: MetricsParams,
}

/// Axlib synchronization pipeline.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub struct AxlibHeadersSyncPipeline;

impl HeadersSyncPipeline for AxlibHeadersSyncPipeline {
	const SOURCE_NAME: &'static str = "Axlib";
	const TARGET_NAME: &'static str = "Ethereum";

	type Hash = rialto_runtime::Hash;
	type Number = rialto_runtime::BlockNumber;
	type Header = RialtoSyncHeader;
	type Extra = ();
	type Completion = EncodedJustification;

	fn estimate_size(source: &QueuedHeader<Self>) -> usize {
		source.header().encode().len()
	}
}

/// Queued axlib header ID.
pub type QueuedRialtoHeader = QueuedHeader<AxlibHeadersSyncPipeline>;

/// Rialto node as headers source.
type AxlibHeadersSource = HeadersSource<Rialto, AxlibHeadersSyncPipeline>;

/// Ethereum client as Axlib headers target.
#[derive(Clone)]
struct EthereumHeadersTarget {
	/// Ethereum node client.
	client: EthereumClient,
	/// Bridge contract address.
	contract: Address,
	/// Ethereum signing params.
	sign_params: EthereumSigningParams,
}

impl EthereumHeadersTarget {
	fn new(client: EthereumClient, contract: Address, sign_params: EthereumSigningParams) -> Self {
		Self {
			client,
			contract,
			sign_params,
		}
	}
}

#[async_trait]
impl RelayClient for EthereumHeadersTarget {
	type Error = RpcError;

	async fn reconnect(&mut self) -> Result<(), RpcError> {
		self.client.reconnect().await.map_err(Into::into)
	}
}

#[async_trait]
impl TargetClient<AxlibHeadersSyncPipeline> for EthereumHeadersTarget {
	async fn best_header_id(&self) -> Result<RialtoHeaderId, RpcError> {
		// we can't continue to relay headers if Ethereum node is out of sync, because
		// it may have already received (some of) headers that we're going to relay
		self.client.ensure_synced().await?;

		self.client.best_axlib_block(self.contract).await
	}

	async fn is_known_header(&self, id: RialtoHeaderId) -> Result<(RialtoHeaderId, bool), RpcError> {
		self.client.axlib_header_known(self.contract, id).await
	}

	async fn submit_headers(&self, headers: Vec<QueuedRialtoHeader>) -> SubmittedHeaders<RialtoHeaderId, RpcError> {
		self.client
			.submit_axlib_headers(self.sign_params.clone(), self.contract, headers)
			.await
	}

	async fn incomplete_headers_ids(&self) -> Result<HashSet<RialtoHeaderId>, RpcError> {
		self.client.incomplete_axlib_headers(self.contract).await
	}

	async fn complete_header(
		&self,
		id: RialtoHeaderId,
		completion: EncodedJustification,
	) -> Result<RialtoHeaderId, RpcError> {
		self.client
			.complete_axlib_header(self.sign_params.clone(), self.contract, id, completion)
			.await
	}

	async fn requires_extra(&self, header: QueuedRialtoHeader) -> Result<(RialtoHeaderId, bool), RpcError> {
		Ok((header.header().id(), false))
	}
}

/// Run Axlib headers synchronization.
pub async fn run(params: AxlibSyncParams) -> Result<(), RpcError> {
	let AxlibSyncParams {
		sub_params,
		eth_params,
		eth_sign,
		eth_contract_address,
		sync_params,
		metrics_params,
	} = params;

	let eth_client = EthereumClient::new(eth_params).await;
	let sub_client = AxlibClient::<Rialto>::new(sub_params).await;

	let target = EthereumHeadersTarget::new(eth_client, eth_contract_address, eth_sign);
	let source = AxlibHeadersSource::new(sub_client);

	headers_relay::sync_loop::run(
		source,
		Rialto::AVERAGE_BLOCK_INTERVAL,
		target,
		consts::ETHEREUM_TICK_INTERVAL,
		(),
		sync_params,
		metrics_params,
		futures::future::pending(),
	)
	.await
	.map_err(RpcError::SyncLoop)?;

	Ok(())
}
