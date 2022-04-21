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

//! Axlib client as Axlib finality proof target. The chain we connect to should have
//! runtime that implements `<BridgedChainName>FinalityApi` to allow bridging with
//! <BridgedName> chain.

use crate::finality_pipeline::AxlibFinalitySyncPipeline;

use async_trait::async_trait;
use codec::Decode;
use finality_relay::{FinalitySyncPipeline, TargetClient};
use relay_axlib_client::{Chain, Client, Error as AxlibError};
use relay_utils::relay_loop::Client as RelayClient;

/// Axlib client as Axlib finality target.
pub struct AxlibFinalityTarget<C: Chain, P> {
	client: Client<C>,
	pipeline: P,
	transactions_mortality: Option<u32>,
}

impl<C: Chain, P> AxlibFinalityTarget<C, P> {
	/// Create new Axlib headers target.
	pub fn new(client: Client<C>, pipeline: P, transactions_mortality: Option<u32>) -> Self {
		AxlibFinalityTarget { client, pipeline, transactions_mortality }
	}
}

impl<C: Chain, P: AxlibFinalitySyncPipeline> Clone for AxlibFinalityTarget<C, P> {
	fn clone(&self) -> Self {
		AxlibFinalityTarget {
			client: self.client.clone(),
			pipeline: self.pipeline.clone(),
			transactions_mortality: self.transactions_mortality,
		}
	}
}

#[async_trait]
impl<C: Chain, P: AxlibFinalitySyncPipeline> RelayClient for AxlibFinalityTarget<C, P> {
	type Error = AxlibError;

	async fn reconnect(&mut self) -> Result<(), AxlibError> {
		self.client.reconnect().await
	}
}

#[async_trait]
impl<C, P> TargetClient<P::FinalitySyncPipeline> for AxlibFinalityTarget<C, P>
where
	C: Chain,
	P: AxlibFinalitySyncPipeline<TargetChain = C>,
	<P::FinalitySyncPipeline as FinalitySyncPipeline>::Number: Decode,
	<P::FinalitySyncPipeline as FinalitySyncPipeline>::Hash: Decode,
{
	async fn best_finalized_source_block_number(
		&self,
	) -> Result<<P::FinalitySyncPipeline as FinalitySyncPipeline>::Number, AxlibError> {
		// we can't continue to relay finality if target node is out of sync, because
		// it may have already received (some of) headers that we're going to relay
		self.client.ensure_synced().await?;

		Ok(crate::messages_source::read_client_state::<
			C,
			<P::FinalitySyncPipeline as FinalitySyncPipeline>::Hash,
			<P::FinalitySyncPipeline as FinalitySyncPipeline>::Number,
		>(&self.client, P::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET)
		.await?
		.best_finalized_peer_at_best_self
		.0)
	}

	async fn submit_finality_proof(
		&self,
		header: <P::FinalitySyncPipeline as FinalitySyncPipeline>::Header,
		proof: <P::FinalitySyncPipeline as FinalitySyncPipeline>::FinalityProof,
	) -> Result<(), AxlibError> {
		let transactions_author = self.pipeline.transactions_author();
		let pipeline = self.pipeline.clone();
		let transactions_mortality = self.transactions_mortality;
		self.client
			.submit_signed_extrinsic(
				transactions_author,
				move |best_block_id, transaction_nonce| {
					pipeline.make_submit_finality_proof_transaction(
						relay_axlib_client::TransactionEra::new(
							best_block_id,
							transactions_mortality,
						),
						transaction_nonce,
						header,
						proof,
					)
				},
			)
			.await
			.map(drop)
	}
}
