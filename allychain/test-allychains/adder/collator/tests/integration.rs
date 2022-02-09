// Copyright 2020 AXIA Technologies (UK) Ltd.
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

//! Integration test that ensures that we can build and include allychain
//! blocks of the adder allychain.

const PUPPET_EXE: &str = env!("CARGO_BIN_EXE_adder_collator_puppet_worker");

// If this test is failing, make sure to run all tests with the `real-overseer` feature being enabled.
#[axlib_test_utils::test]
async fn collating_using_adder_collator() {
	use futures::join;
	use axia_primitives::v1::Id as ParaId;
	use sp_keyring::AccountKeyring::*;

	let mut builder = sc_cli::LoggerBuilder::new("");
	builder.with_colors(false);
	builder.init().expect("Set up logger");

	let para_id = ParaId::from(100);

	// start alice
	let alice = axia_test_service::run_validator_node(
		tokio::runtime::Handle::current(),
		Alice,
		|| {},
		vec![],
		Some(PUPPET_EXE.into()),
	);

	// start bob
	let bob = axia_test_service::run_validator_node(
		tokio::runtime::Handle::current(),
		Bob,
		|| {},
		vec![alice.addr.clone()],
		Some(PUPPET_EXE.into()),
	);

	let collator = test_allychain_adder_collator::Collator::new();

	// register allychain
	alice
		.register_allychain(para_id, collator.validation_code().to_vec(), collator.genesis_head())
		.await
		.unwrap();

	// run the collator node
	let mut charlie = axia_test_service::run_collator_node(
		tokio::runtime::Handle::current(),
		Charlie,
		|| {},
		vec![alice.addr.clone(), bob.addr.clone()],
		collator.collator_key(),
	);

	charlie
		.register_collator(
			collator.collator_key(),
			para_id,
			collator.create_collation_function(charlie.task_manager.spawn_handle()),
		)
		.await;

	// Wait until the allychain has 4 blocks produced.
	collator.wait_for_blocks(4).await;

	// Wait until the collator received `12` seconded statements for its collations.
	collator.wait_for_seconded_collations(12).await;

	join!(
		alice.task_manager.clean_shutdown(),
		bob.task_manager.clean_shutdown(),
		charlie.task_manager.clean_shutdown(),
	);
}
