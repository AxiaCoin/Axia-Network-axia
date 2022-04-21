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

use crate::cli::{SourceConnectionParams, TargetConnectionParams, TargetSigningParams};
use bp_header_chain::InitializationData;
use bp_runtime::Chain as ChainBase;
use codec::Encode;
use relay_axlib_client::{Chain, TransactionSignScheme, UnsignedTransaction};
use sp_core::{Bytes, Pair};
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

/// Initialize bridge pallet.
#[derive(StructOpt)]
pub struct InitBridge {
	/// A bridge instance to initialize.
	#[structopt(possible_values = InitBridgeName::VARIANTS, case_insensitive = true)]
	bridge: InitBridgeName,
	#[structopt(flatten)]
	source: SourceConnectionParams,
	#[structopt(flatten)]
	target: TargetConnectionParams,
	#[structopt(flatten)]
	target_sign: TargetSigningParams,
}

#[derive(Debug, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
/// Bridge to initialize.
pub enum InitBridgeName {
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
			InitBridgeName::MillauToRialto => {
				type Source = relay_millau_client::Millau;
				type Target = relay_rialto_client::Rialto;

				fn encode_init_bridge(
					init_data: InitializationData<<Source as ChainBase>::Header>,
				) -> <Target as Chain>::Call {
					rialto_runtime::SudoCall::sudo {
						call: Box::new(
							rialto_runtime::BridgeGrandpaMillauCall::initialize { init_data }
								.into(),
						),
					}
					.into()
				}

				$generic
			},
			InitBridgeName::RialtoToMillau => {
				type Source = relay_rialto_client::Rialto;
				type Target = relay_millau_client::Millau;

				fn encode_init_bridge(
					init_data: InitializationData<<Source as ChainBase>::Header>,
				) -> <Target as Chain>::Call {
					let initialize_call = millau_runtime::BridgeGrandpaCall::<
						millau_runtime::Runtime,
						millau_runtime::RialtoGrandpaInstance,
					>::initialize {
						init_data,
					};
					millau_runtime::SudoCall::sudo { call: Box::new(initialize_call.into()) }.into()
				}

				$generic
			},
			InitBridgeName::AlphanetToMillau => {
				type Source = relay_alphanet_client::Alphanet;
				type Target = relay_millau_client::Millau;

				fn encode_init_bridge(
					init_data: InitializationData<<Source as ChainBase>::Header>,
				) -> <Target as Chain>::Call {
					// at Alphanet -> Millau initialization we're not using sudo, because otherwise
					// our deployments may fail, because we need to initialize both Rialto -> Millau
					// and Alphanet -> Millau bridge. => since there's single possible sudo account,
					// one of transaction may fail with duplicate nonce error
					millau_runtime::BridgeGrandpaCall::<
						millau_runtime::Runtime,
						millau_runtime::AlphanetGrandpaInstance,
					>::initialize {
						init_data,
					}
					.into()
				}

				$generic
			},
			InitBridgeName::BetanetToWococo => {
				type Source = relay_betanet_client::Betanet;
				type Target = relay_wococo_client::Wococo;

				fn encode_init_bridge(
					init_data: InitializationData<<Source as ChainBase>::Header>,
				) -> <Target as Chain>::Call {
					relay_wococo_client::runtime::Call::BridgeGrandpaBetanet(
						relay_wococo_client::runtime::BridgeGrandpaBetanetCall::initialize(
							init_data,
						),
					)
				}

				$generic
			},
			InitBridgeName::WococoToBetanet => {
				type Source = relay_wococo_client::Wococo;
				type Target = relay_betanet_client::Betanet;

				fn encode_init_bridge(
					init_data: InitializationData<<Source as ChainBase>::Header>,
				) -> <Target as Chain>::Call {
					relay_betanet_client::runtime::Call::BridgeGrandpaWococo(
						relay_betanet_client::runtime::BridgeGrandpaWococoCall::initialize(
							init_data,
						),
					)
				}

				$generic
			},
			InitBridgeName::AxiaTestToAxia => {
				type Source = relay_axctest_client::AxiaTest;
				type Target = relay_axia_client::Axia;

				fn encode_init_bridge(
					init_data: InitializationData<<Source as ChainBase>::Header>,
				) -> <Target as Chain>::Call {
					relay_axia_client::runtime::Call::BridgeAxiaTestGrandpa(
						relay_axia_client::runtime::BridgeAxiaTestGrandpaCall::initialize(
							init_data,
						),
					)
				}

				$generic
			},
			InitBridgeName::AxiaToAxiaTest => {
				type Source = relay_axia_client::Axia;
				type Target = relay_axctest_client::AxiaTest;

				fn encode_init_bridge(
					init_data: InitializationData<<Source as ChainBase>::Header>,
				) -> <Target as Chain>::Call {
					relay_axctest_client::runtime::Call::BridgeAxiaGrandpa(
						relay_axctest_client::runtime::BridgeAxiaGrandpaCall::initialize(
							init_data,
						),
					)
				}

				$generic
			},
		}
	};
}

impl InitBridge {
	/// Run the command.
	pub async fn run(self) -> anyhow::Result<()> {
		select_bridge!(self.bridge, {
			let source_client = self.source.to_client::<Source>().await?;
			let target_client = self.target.to_client::<Target>().await?;
			let target_sign = self.target_sign.to_keypair::<Target>()?;

			axlib_relay_helper::headers_initialize::initialize(
				source_client,
				target_client.clone(),
				target_sign.public().into(),
				move |transaction_nonce, initialization_data| {
					Bytes(
						Target::sign_transaction(
							*target_client.genesis_hash(),
							&target_sign,
							relay_axlib_client::TransactionEra::immortal(),
							UnsignedTransaction::new(
								encode_init_bridge(initialization_data),
								transaction_nonce,
							),
						)
						.encode(),
					)
				},
			)
			.await;

			Ok(())
		})
	}
}
