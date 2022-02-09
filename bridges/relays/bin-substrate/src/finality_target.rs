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

//! Axlib client as Axlib finality proof target. The chain we connect to should have
//! runtime that implements `<BridgedChainName>FinalityApi` to allow bridging with
//! <BridgedName> chain.

use crate::finality_pipeline::AxlibFinalitySyncPipeline;

use async_trait::async_trait;
use codec::Decode;
use finality_relay::TargetClient;
use relay_axlib_client::{Chain, Client, Error as AxlibError};
use relay_utils::relay_loop::Client as RelayClient;

/// Axlib client as Axlib finality target.
pub struct AxlibFinalityTarget<C: Chain, P> {
	client: Client<C>,
	pipeline: P,
}

impl<C: Chain, P> AxlibFinalityTarget<C, P> {
	/// Create new Axlib headers target.
	pub fn new(client: Client<C>, pipeline: P) -> Self {
		AxlibFinalityTarget { client, pipeline }
	}
}

impl<C: Chain, P: AxlibFinalitySyncPipeline> Clone for AxlibFinalityTarget<C, P> {
	fn clone(&self) -> Self {
		AxlibFinalityTarget {
			client: self.client.clone(),
			pipeline: self.pipeline.clone(),
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
impl<C, P> TargetClient<P> for AxlibFinalityTarget<C, P>
where
	C: Chain,
	P::Number: Decode,
	P::Hash: Decode,
	P: AxlibFinalitySyncPipeline<TargetChain = C>,
{
	async fn best_finalized_source_block_number(&self) -> Result<P::Number, AxlibError> {
		// we can't continue to relay finality if target node is out of sync, because
		// it may have already received (some of) headers that we're going to relay
		self.client.ensure_synced().await?;

		Ok(crate::messages_source::read_client_state::<C, P::Hash, P::Number>(
			&self.client,
			P::BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET,
		)
		.await?
		.best_finalized_peer_at_best_self
		.0)
	}

	async fn submit_finality_proof(&self, header: P::Header, proof: P::FinalityProof) -> Result<(), AxlibError> {
		self.client
			.submit_signed_extrinsic(self.pipeline.transactions_author(), move |transaction_nonce| {
				self.pipeline
					.make_submit_finality_proof_transaction(transaction_nonce, header, proof)
			})
			.await
			.map(drop)
	}
}
