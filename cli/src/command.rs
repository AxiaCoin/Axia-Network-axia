// Copyright 2017-2020 AXIA Technologies (UK) Ltd.
// This file is part of AXIA.

// AXIA is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// AXIA is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with AXIA.  If not, see <http://www.gnu.org/licenses/>.

use crate::cli::{Cli, Subcommand};
use futures::future::TryFutureExt;
use log::info;
use sc_cli::{Role, RuntimeVersion, AxlibCli};
use service::{self, IdentifyVariant};
use sp_core::crypto::Ss58AddressFormatRegistry;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	AXIAService(#[from] service::Error),

	#[error(transparent)]
	AxlibCli(#[from] sc_cli::Error),

	#[error(transparent)]
	AxlibService(#[from] sc_service::Error),

	#[error("Other: {0}")]
	Other(String),
}

impl std::convert::From<String> for Error {
	fn from(s: String) -> Self {
		Self::Other(s)
	}
}

type Result<T> = std::result::Result<T, Error>;

fn get_exec_name() -> Option<String> {
	std::env::current_exe()
		.ok()
		.and_then(|pb| pb.file_name().map(|s| s.to_os_string()))
		.and_then(|s| s.into_string().ok())
}

impl AxlibCli for Cli {
	fn impl_name() -> String {
		"AXIA AXIA".into()
	}

	fn impl_version() -> String {
		env!("AXLIB_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/axia/axia/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2017
	}

	fn executable_name() -> String {
		"axia".into()
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id == "" {
			let n = get_exec_name().unwrap_or_default();
			["axia", "axiatest", "alphanet", "betanet"]
				.iter()
				.cloned()
				.find(|&chain| n.starts_with(chain))
				.unwrap_or("axia")
		} else {
			id
		};
		Ok(match id {
			"axiatest" => Box::new(service::chain_spec::axiatest_config()?),
			#[cfg(feature = "axiatest-native")]
			"axiatest-dev" => Box::new(service::chain_spec::axiatest_development_config()?),
			#[cfg(feature = "axiatest-native")]
			"axiatest-local" => Box::new(service::chain_spec::axiatest_local_testnet_config()?),
			#[cfg(feature = "axiatest-native")]
			"axiatest-staging" => Box::new(service::chain_spec::axiatest_staging_testnet_config()?),
			#[cfg(not(feature = "axiatest-native"))]
			name if name.starts_with("axiatest-") && !name.ends_with(".json") =>
				Err(format!("`{}` only supported with `axiatest-native` feature enabled.", name))?,
			"axia" => Box::new(service::chain_spec::axia_config()?),
			#[cfg(feature = "axia-native")]
			"axia-dev" | "dev" => Box::new(service::chain_spec::axia_development_config()?),
			#[cfg(feature = "axia-native")]
			"axia-local" => Box::new(service::chain_spec::axia_local_testnet_config()?),
			#[cfg(feature = "axia-native")]
			"axia-staging" => Box::new(service::chain_spec::axia_staging_testnet_config()?),
			"betanet" => Box::new(service::chain_spec::betanet_config()?),
			#[cfg(feature = "betanet-native")]
			"betanet-dev" => Box::new(service::chain_spec::betanet_development_config()?),
			#[cfg(feature = "betanet-native")]
			"betanet-local" => Box::new(service::chain_spec::betanet_local_testnet_config()?),
			#[cfg(feature = "betanet-native")]
			"betanet-staging" => Box::new(service::chain_spec::betanet_staging_testnet_config()?),
			#[cfg(not(feature = "betanet-native"))]
			name if name.starts_with("betanet-") && !name.ends_with(".json") =>
				Err(format!("`{}` only supported with `betanet-native` feature enabled.", name))?,
			"alphanet" => Box::new(service::chain_spec::alphanet_config()?),
			#[cfg(feature = "alphanet-native")]
			"alphanet-dev" => Box::new(service::chain_spec::alphanet_development_config()?),
			#[cfg(feature = "alphanet-native")]
			"alphanet-local" => Box::new(service::chain_spec::alphanet_local_testnet_config()?),
			#[cfg(feature = "alphanet-native")]
			"alphanet-staging" => Box::new(service::chain_spec::alphanet_staging_testnet_config()?),
			#[cfg(not(feature = "alphanet-native"))]
			name if name.starts_with("alphanet-") && !name.ends_with(".json") =>
				Err(format!("`{}` only supported with `alphanet-native` feature enabled.", name))?,
			"wococo" => Box::new(service::chain_spec::wococo_config()?),
			#[cfg(feature = "betanet-native")]
			"wococo-dev" => Box::new(service::chain_spec::wococo_development_config()?),
			#[cfg(feature = "betanet-native")]
			"wococo-local" => Box::new(service::chain_spec::wococo_local_testnet_config()?),
			#[cfg(not(feature = "betanet-native"))]
			name if name.starts_with("wococo-") =>
				Err(format!("`{}` only supported with `betanet-native` feature enabled.", name))?,
			path => {
				let path = std::path::PathBuf::from(path);

				let chain_spec = Box::new(service::AXIAChainSpec::from_json_file(path.clone())?)
					as Box<dyn service::ChainSpec>;

				// When `force_*` is given or the file name starts with the name of one of the known chains,
				// we use the chain spec for the specific chain.
				if self.run.force_betanet || chain_spec.is_betanet() || chain_spec.is_wococo() {
					Box::new(service::BetaNetChainSpec::from_json_file(path)?)
				} else if self.run.force_axiatest || chain_spec.is_axiatest() {
					Box::new(service::AXIATESTChainSpec::from_json_file(path)?)
				} else if self.run.force_alphanet || chain_spec.is_alphanet() {
					Box::new(service::AlphaNetChainSpec::from_json_file(path)?)
				} else {
					chain_spec
				}
			},
		})
	}

	fn native_runtime_version(spec: &Box<dyn service::ChainSpec>) -> &'static RuntimeVersion {
		#[cfg(feature = "axiatest-native")]
		if spec.is_axiatest() {
			return &service::axiatest_runtime::VERSION
		}

		#[cfg(feature = "alphanet-native")]
		if spec.is_alphanet() {
			return &service::alphanet_runtime::VERSION
		}

		#[cfg(feature = "betanet-native")]
		if spec.is_betanet() || spec.is_wococo() {
			return &service::betanet_runtime::VERSION
		}

		#[cfg(not(all(
			feature = "betanet-native",
			feature = "alphanet-native",
			feature = "axiatest-native"
		)))]
		let _ = spec;

		#[cfg(feature = "axia-native")]
		{
			return &service::axia_runtime::VERSION
		}

		#[cfg(not(feature = "axia-native"))]
		panic!("No runtime feature (axia, axiatest, alphanet, betanet) is enabled")
	}
}

fn set_default_ss58_version(spec: &Box<dyn service::ChainSpec>) {
	let ss58_version = if spec.is_axiatest() {
		Ss58AddressFormatRegistry::AxiatestAccount
	} else if spec.is_alphanet() {
		Ss58AddressFormatRegistry::AxlibAccount
	} else {
		Ss58AddressFormatRegistry::AxiaAccount
	}
	.into();

	sp_core::crypto::set_default_ss58_version(ss58_version);
}

const DEV_ONLY_ERROR_PATTERN: &'static str =
	"can only use subcommand with --chain [axia-dev, axiatest-dev, alphanet-dev, betanet-dev, wococo-dev], got ";

fn ensure_dev(spec: &Box<dyn service::ChainSpec>) -> std::result::Result<(), String> {
	if spec.is_dev() {
		Ok(())
	} else {
		Err(format!("{}{}", DEV_ONLY_ERROR_PATTERN, spec.id()))
	}
}

/// Launch a node, accepting arguments just like a regular node,
/// accepts an alternative overseer generator, to adjust behavior
/// for integration tests as needed.
#[cfg(feature = "malus")]
pub fn run_node(cli: Cli, overseer_gen: impl service::OverseerGen) -> Result<()> {
	run_node_inner(cli, overseer_gen)
}

fn run_node_inner(cli: Cli, overseer_gen: impl service::OverseerGen) -> Result<()> {
	let runner = cli.create_runner(&cli.run.base).map_err(Error::from)?;
	let chain_spec = &runner.config().chain_spec;

	set_default_ss58_version(chain_spec);

	let grandpa_pause = if cli.run.grandpa_pause.is_empty() {
		None
	} else {
		Some((cli.run.grandpa_pause[0], cli.run.grandpa_pause[1]))
	};

	if chain_spec.is_axiatest() {
		info!("----------------------------");
		info!("This chain is not in any way");
		info!("      endorsed by the       ");
		info!("     AXIATEST FOUNDATION      ");
		info!("----------------------------");
	}

	let jaeger_agent = cli.run.jaeger_agent;

	runner.run_node_until_exit(move |config| async move {
		let role = config.role.clone();

		match role {
			Role::Light => Err(Error::Other("Light client not enabled".into())),
			_ => service::build_full(
				config,
				service::IsCollator::No,
				grandpa_pause,
				cli.run.no_beefy,
				jaeger_agent,
				None,
				overseer_gen,
			)
			.map(|full| full.task_manager)
			.map_err(Into::into),
		}
	})
}

/// Parses axia specific CLI arguments and run the service.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => run_node_inner(cli, service::RealOverseerGen),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			Ok(runner.sync_run(|config| cmd.run(config.chain_spec, config.network))?)
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd).map_err(Error::AxlibCli)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) =
					service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, import_queue).map_err(Error::AxlibCli), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, _, _, task_manager) =
					service::new_chain_ops(&mut config, None).map_err(Error::AXIAService)?;
				Ok((cmd.run(client, config.database).map_err(Error::AxlibCli), task_manager))
			})?)
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, _, _, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, config.chain_spec).map_err(Error::AxlibCli), task_manager))
			})?)
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, _, import_queue, task_manager) =
					service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, import_queue).map_err(Error::AxlibCli), task_manager))
			})?)
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			Ok(runner.sync_run(|config| cmd.run(config.database))?)
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			Ok(runner.async_run(|mut config| {
				let (client, backend, _, task_manager) = service::new_chain_ops(&mut config, None)?;
				Ok((cmd.run(client, backend).map_err(Error::AxlibCli), task_manager))
			})?)
		},
		Some(Subcommand::PvfPrepareWorker(cmd)) => {
			let mut builder = sc_cli::LoggerBuilder::new("");
			builder.with_colors(false);
			let _ = builder.init();

			#[cfg(target_os = "android")]
			{
				return Err(sc_cli::Error::Input(
					"PVF preparation workers are not supported under this platform".into(),
				)
				.into())
			}

			#[cfg(not(target_os = "android"))]
			{
				axia_node_core_pvf::prepare_worker_entrypoint(&cmd.socket_path);
				Ok(())
			}
		},
		Some(Subcommand::PvfExecuteWorker(cmd)) => {
			let mut builder = sc_cli::LoggerBuilder::new("");
			builder.with_colors(false);
			let _ = builder.init();

			#[cfg(target_os = "android")]
			{
				return Err(sc_cli::Error::Input(
					"PVF execution workers are not supported under this platform".into(),
				)
				.into())
			}

			#[cfg(not(target_os = "android"))]
			{
				axia_node_core_pvf::execute_worker_entrypoint(&cmd.socket_path);
				Ok(())
			}
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			ensure_dev(chain_spec).map_err(Error::Other)?;

			#[cfg(feature = "axiatest-native")]
			if chain_spec.is_axiatest() {
				return Ok(runner.sync_run(|config| {
					cmd.run::<service::axiatest_runtime::Block, service::AXIATESTExecutorDispatch>(
						config,
					)
					.map_err(|e| Error::AxlibCli(e))
				})?)
			}

			#[cfg(feature = "alphanet-native")]
			if chain_spec.is_alphanet() {
				return Ok(runner.sync_run(|config| {
					cmd.run::<service::alphanet_runtime::Block, service::AlphaNetExecutorDispatch>(
						config,
					)
					.map_err(|e| Error::AxlibCli(e))
				})?)
			}

			// else we assume it is axia.
			#[cfg(feature = "axia-native")]
			{
				return Ok(runner.sync_run(|config| {
					cmd.run::<service::axia_runtime::Block, service::AXIAExecutorDispatch>(
						config,
					)
					.map_err(|e| Error::AxlibCli(e))
				})?)
			}
			#[cfg(not(feature = "axia-native"))]
			panic!("No runtime feature (axia, axiatest, alphanet, betanet) is enabled")
		},
		Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			use sc_service::TaskManager;
			let registry = &runner.config().prometheus_config.as_ref().map(|cfg| &cfg.registry);
			let task_manager = TaskManager::new(runner.config().tokio_handle.clone(), *registry)
				.map_err(|e| Error::AxlibService(sc_service::Error::Prometheus(e)))?;

			ensure_dev(chain_spec).map_err(Error::Other)?;

			#[cfg(feature = "axiatest-native")]
			if chain_spec.is_axiatest() {
				return runner.async_run(|config| {
					Ok((
						cmd.run::<service::axiatest_runtime::Block, service::AXIATESTExecutorDispatch>(
							config,
						)
						.map_err(Error::AxlibCli),
						task_manager,
					))
				})
			}

			#[cfg(feature = "alphanet-native")]
			if chain_spec.is_alphanet() {
				return runner.async_run(|config| {
					Ok((
						cmd.run::<service::alphanet_runtime::Block, service::AlphaNetExecutorDispatch>(
							config,
						)
						.map_err(Error::AxlibCli),
						task_manager,
					))
				})
			}
			// else we assume it is axia.
			#[cfg(feature = "axia-native")]
			{
				return runner.async_run(|config| {
					Ok((
						cmd.run::<service::axia_runtime::Block, service::AXIAExecutorDispatch>(
							config,
						)
						.map_err(Error::AxlibCli),
						task_manager,
					))
				})
			}
			#[cfg(not(feature = "axia-native"))]
			panic!("No runtime feature (axia, axiatest, alphanet, betanet) is enabled")
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err(Error::Other(
			"TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
				.into(),
		)
		.into()),
	}?;
	Ok(())
}
