// Copyright 2021 Axia Technologies (UK) Ltd.
// This file is part of Axia.

// Axia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia.  If not, see <http://www.gnu.org/licenses/>.

//! Remote tests for bags-list pallet.

use clap::{ArgEnum, Parser};
use std::convert::TryInto;

#[derive(Clone, Debug, ArgEnum)]
#[clap(rename_all = "PascalCase")]
enum Command {
	CheckMigration,
	SanityCheck,
	Snapshot,
}

#[derive(Clone, Debug, ArgEnum)]
#[clap(rename_all = "PascalCase")]
enum Runtime {
	Axia,
	AxiaTest,
	Alphanet,
}

#[derive(Parser)]
struct Cli {
	#[clap(long, short, default_value = "wss://axctest-rpc.axia.io:443")]
	uri: String,
	#[clap(long, short, ignore_case = true, arg_enum, default_value = "axctest")]
	runtime: Runtime,
	#[clap(long, short, ignore_case = true, arg_enum, default_value = "SanityCheck")]
	command: Command,
	#[clap(long, short)]
	snapshot_limit: Option<usize>,
}

#[tokio::main]
async fn main() {
	let options = Cli::parse();
	sp_tracing::try_init_simple();

	log::info!(
		target: "remote-ext-tests",
		"using runtime {:?} / command: {:?}",
		options.runtime,
		options.command
	);

	use pallet_bags_list_remote_tests::*;
	match options.runtime {
		Runtime::Axia => sp_core::crypto::set_default_ss58_version(
			<axia_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
		Runtime::AxiaTest => sp_core::crypto::set_default_ss58_version(
			<axctest_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
		Runtime::Alphanet => sp_core::crypto::set_default_ss58_version(
			<alphanet_runtime::Runtime as frame_system::Config>::SS58Prefix::get()
				.try_into()
				.unwrap(),
		),
	};

	match (options.runtime, options.command) {
		(Runtime::AxiaTest, Command::CheckMigration) => {
			use axctest_runtime::{Block, Runtime};
			use axctest_runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "AXCT", options.uri.clone()).await;
		},
		(Runtime::AxiaTest, Command::SanityCheck) => {
			use axctest_runtime::{Block, Runtime};
			use axctest_runtime_constants::currency::UNITS;
			sanity_check::execute::<Runtime, Block>(UNITS as u64, "AXCT", options.uri.clone()).await;
		},
		(Runtime::AxiaTest, Command::Snapshot) => {
			use axctest_runtime::{Block, Runtime};
			use axctest_runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},

		(Runtime::Alphanet, Command::CheckMigration) => {
			use alphanet_runtime::{Block, Runtime};
			use alphanet_runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "WND", options.uri.clone()).await;
		},
		(Runtime::Alphanet, Command::SanityCheck) => {
			use alphanet_runtime::{Block, Runtime};
			use alphanet_runtime_constants::currency::UNITS;
			sanity_check::execute::<Runtime, Block>(UNITS as u64, "WND", options.uri.clone()).await;
		},
		(Runtime::Alphanet, Command::Snapshot) => {
			use alphanet_runtime::{Block, Runtime};
			use alphanet_runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},

		(Runtime::Axia, Command::CheckMigration) => {
			use axia_runtime::{Block, Runtime};
			use axia_runtime_constants::currency::UNITS;
			migration::execute::<Runtime, Block>(UNITS as u64, "AXC", options.uri.clone()).await;
		},
		(Runtime::Axia, Command::SanityCheck) => {
			use axia_runtime::{Block, Runtime};
			use axia_runtime_constants::currency::UNITS;
			sanity_check::execute::<Runtime, Block>(UNITS as u64, "AXC", options.uri.clone()).await;
		},
		(Runtime::Axia, Command::Snapshot) => {
			use axia_runtime::{Block, Runtime};
			use axia_runtime_constants::currency::UNITS;
			snapshot::execute::<Runtime, Block>(
				options.snapshot_limit,
				UNITS.try_into().unwrap(),
				options.uri.clone(),
			)
			.await;
		},
	}
}
