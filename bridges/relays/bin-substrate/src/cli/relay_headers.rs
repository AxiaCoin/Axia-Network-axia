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

use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

use relay_utils::metrics::{GlobalMetrics, StandaloneMetric};
use axlib_relay_helper::finality_pipeline::AxlibFinalitySyncPipeline;

use crate::cli::{
	PrometheusParams, SourceConnectionParams, TargetConnectionParams, TargetSigningParams,
};

/// Start headers relayer process.
#[derive(StructOpt)]
pub struct RelayHeaders {
	/// A bridge instance to relay headers for.
	#[structopt(possible_values = RelayHeadersBridge::VARIANTS, case_insensitive = true)]
	bridge: RelayHeadersBridge,
	/// If passed, only mandatory headers (headers that are changing the GRANDPA authorities set)
	/// are relayed.
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

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
/// Headers relay bridge.
pub enum RelayHeadersBridge {
	MillauToRialto,
	RialtoToMillau,
	AlphanetToMillau,
	BetanetToWococo,
	WococoToBetanet,
	AxiaTestToAxia,
	AxiaToAxiaTest,
}

macro_rules! select_bridge {
	($bridge: expr, $generic: tt) => {
		match $bridge {
			RelayHeadersBridge::MillauToRialto => {
				type Source = relay_millau_client::Millau;
				type Target = relay_rialto_client::Rialto;
				type Finality = crate::chains::millau_headers_to_rialto::MillauFinalityToRialto;

				$generic
			},
			RelayHeadersBridge::RialtoToMillau => {
				type Source = relay_rialto_client::Rialto;
				type Target = relay_millau_client::Millau;
				type Finality = crate::chains::rialto_headers_to_millau::RialtoFinalityToMillau;

				$generic
			},
			RelayHeadersBridge::AlphanetToMillau => {
				type Source = relay_alphanet_client::Alphanet;
				type Target = relay_millau_client::Millau;
				type Finality = crate::chains::alphanet_headers_to_millau::AlphanetFinalityToMillau;

				$generic
			},
			RelayHeadersBridge::BetanetToWococo => {
				type Source = relay_betanet_client::Betanet;
				type Target = relay_wococo_client::Wococo;
				type Finality = crate::chains::betanet_headers_to_wococo::BetanetFinalityToWococo;

				$generic
			},
			RelayHeadersBridge::WococoToBetanet => {
				type Source = relay_wococo_client::Wococo;
				type Target = relay_betanet_client::Betanet;
				type Finality = crate::chains::wococo_headers_to_betanet::WococoFinalityToBetanet;

				$generic
			},
			RelayHeadersBridge::AxiaTestToAxia => {
				type Source = relay_axctest_client::AxiaTest;
				type Target = relay_axia_client::Axia;
				type Finality = crate::chains::axctest_headers_to_axia::AxiaTestFinalityToAxia;

				$generic
			},
			RelayHeadersBridge::AxiaToAxiaTest => {
				type Source = relay_axia_client::Axia;
				type Target = relay_axctest_client::AxiaTest;
				type Finality = crate::chains::axia_headers_to_axctest::AxiaFinalityToAxiaTest;

				$generic
			},
		}
	};
}

impl RelayHeaders {
	/// Run the command.
	pub async fn run(self) -> anyhow::Result<()> {
		select_bridge!(self.bridge, {
			let source_client = self.source.to_client::<Source>().await?;
			let target_client = self.target.to_client::<Target>().await?;
			let target_transactions_mortality = self.target_sign.target_transactions_mortality;
			let target_sign = self.target_sign.to_keypair::<Target>()?;
			let metrics_params = Finality::customize_metrics(self.prometheus_params.into())?;
			GlobalMetrics::new()?.register_and_spawn(&metrics_params.registry)?;

			let finality = Finality::new(target_client.clone(), target_sign);
			finality.start_relay_guards();

			axlib_relay_helper::finality_pipeline::run(
				finality,
				source_client,
				target_client,
				self.only_mandatory_headers,
				target_transactions_mortality,
				metrics_params,
			)
			.await
		})
	}
}
