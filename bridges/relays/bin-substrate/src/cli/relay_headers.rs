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

use crate::cli::{PrometheusParams, SourceConnectionParams, TargetConnectionParams, TargetSigningParams};
use crate::finality_pipeline::AxlibFinalitySyncPipeline;
use structopt::{clap::arg_enum, StructOpt};

/// Start headers relayer process.
#[derive(StructOpt)]
pub struct RelayHeaders {
	/// A bridge instance to relay headers for.
	#[structopt(possible_values = &RelayHeadersBridge::variants(), case_insensitive = true)]
	bridge: RelayHeadersBridge,
	/// If passed, only mandatory headers (headers that are changing the GRANDPA authorities set) are relayed.
	#[structopt(long)]
	only_mandatory_headers: bool,
	#[structopt(flatten)]
	source: SourceConnectionParams,
	#[structopt(flatten)]
	target: TargetConnectionParams,
	#[structopt(flatten)]
	target_sign: TargetSigningParams,
	#[structopt(flatten)]
	prometheus_params: PrometheusParams,
}

// TODO [#851] Use kebab-case.
arg_enum! {
	#[derive(Debug)]
	/// Headers relay bridge.
	pub enum RelayHeadersBridge {
		MillauToRialto,
		RialtoToMillau,
		AlphaNetToMillau,
		BetaNetToWococo,
		WococoToBetaNet,
	}
}

macro_rules! select_bridge {
	($bridge: expr, $generic: tt) => {
		match $bridge {
			RelayHeadersBridge::MillauToRialto => {
				type Source = relay_millau_client::Millau;
				type Target = relay_rialto_client::Rialto;
				type Finality = crate::chains::millau_headers_to_rialto::MillauFinalityToRialto;

				$generic
			}
			RelayHeadersBridge::RialtoToMillau => {
				type Source = relay_rialto_client::Rialto;
				type Target = relay_millau_client::Millau;
				type Finality = crate::chains::rialto_headers_to_millau::RialtoFinalityToMillau;

				$generic
			}
			RelayHeadersBridge::AlphaNetToMillau => {
				type Source = relay_alphanet_client::AlphaNet;
				type Target = relay_millau_client::Millau;
				type Finality = crate::chains::alphanet_headers_to_millau::AlphaNetFinalityToMillau;

				$generic
			}
			RelayHeadersBridge::BetaNetToWococo => {
				type Source = relay_betanet_client::BetaNet;
				type Target = relay_wococo_client::Wococo;
				type Finality = crate::chains::betanet_headers_to_wococo::BetaNetFinalityToWococo;

				$generic
			}
			RelayHeadersBridge::WococoToBetaNet => {
				type Source = relay_wococo_client::Wococo;
				type Target = relay_betanet_client::BetaNet;
				type Finality = crate::chains::wococo_headers_to_betanet::WococoFinalityToBetaNet;

				$generic
			}
		}
	};
}

impl RelayHeaders {
	/// Run the command.
	pub async fn run(self) -> anyhow::Result<()> {
		select_bridge!(self.bridge, {
			let source_client = self.source.to_client::<Source>().await?;
			let target_client = self.target.to_client::<Target>().await?;
			let target_sign = self.target_sign.to_keypair::<Target>()?;
			let metrics_params = Finality::customize_metrics(self.prometheus_params.into())?;
			let finality = Finality::new(target_client.clone(), target_sign);
			finality.start_relay_guards();

			crate::finality_pipeline::run(
				finality,
				source_client,
				target_client,
				self.only_mandatory_headers,
				metrics_params,
			)
			.await
		})
	}
}
