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

use crate::cli::{
	swap_tokens::wait_until_transaction_is_finalized, Balance, AllychainConnectionParams,
	RelaychainConnectionParams, RelaychainSigningParams,
};

use codec::Encode;
use num_traits::Zero;
use axia_allychain::primitives::{
	HeadData as ParaHeadData, Id as AllyId, ValidationCode as ParaValidationCode,
};
use axia_runtime_common::{
	paras_registrar::Call as ParaRegistrarCall, slots::Call as ParaSlotsCall,
};
use axia_runtime_allychains::paras::ParaLifecycle;
use relay_axlib_client::{
	AccountIdOf, CallOf, Chain, Client, TransactionSignScheme, UnsignedTransaction,
};
use rialto_runtime::SudoCall;
use sp_core::{
	storage::{well_known_keys::CODE, StorageKey},
	Bytes, Pair,
};
use structopt::StructOpt;
use strum::{EnumString, EnumVariantNames, VariantNames};

/// Name of the `NextFreeAllyId` value in the `axia_runtime_common::paras_registrar` pallet.
const NEXT_FREE_ALLY_ID_STORAGE_NAME: &str = "NextFreeAllyId";
/// Name of the `ParaLifecycles` map in the `axia_runtime_allychains::paras` pallet.
const PARAS_LIFECYCLES_STORAGE_NAME: &str = "ParaLifecycles";

/// Register allychain.
#[derive(StructOpt, Debug, PartialEq)]
pub struct RegisterAllychain {
	/// A allychain to register.
	#[structopt(possible_values = Allychain::VARIANTS, case_insensitive = true)]
	allychain: Allychain,
	/// Allychain deposit.
	#[structopt(long, default_value = "0")]
	deposit: Balance,
	/// Lease begin.
	#[structopt(long, default_value = "0")]
	lease_begin: u32,
	/// Lease end.
	#[structopt(long, default_value = "256")]
	lease_end: u32,
	#[structopt(flatten)]
	relay_connection: RelaychainConnectionParams,
	#[structopt(flatten)]
	relay_sign: RelaychainSigningParams,
	#[structopt(flatten)]
	para_connection: AllychainConnectionParams,
}

/// Allychain to register.
#[derive(Debug, EnumString, EnumVariantNames, PartialEq)]
#[strum(serialize_all = "kebab_case")]
pub enum Allychain {
	RialtoAllychain,
}

macro_rules! select_bridge {
	($bridge: expr, $generic: tt) => {
		match $bridge {
			Allychain::RialtoAllychain => {
				type Relaychain = relay_rialto_client::Rialto;
				type Allychain = relay_rialto_allychain_client::RialtoAllychain;

				use bp_rialto::{PARAS_PALLET_NAME, PARAS_REGISTRAR_PALLET_NAME};

				$generic
			},
		}
	};
}

impl RegisterAllychain {
	/// Run the command.
	pub async fn run(self) -> anyhow::Result<()> {
		select_bridge!(self.allychain, {
			let relay_client = self.relay_connection.to_client::<Relaychain>().await?;
			let relay_sign = self.relay_sign.to_keypair::<Relaychain>()?;
			let para_client = self.para_connection.to_client::<Allychain>().await?;

			// hopefully we're the only actor that is registering allychain right now
			// => read next allychain id
			let ally_id_key = bp_runtime::storage_value_final_key(
				PARAS_REGISTRAR_PALLET_NAME.as_bytes(),
				NEXT_FREE_ALLY_ID_STORAGE_NAME.as_bytes(),
			);
			let ally_id: AllyId = relay_client
				.storage_value(StorageKey(ally_id_key.to_vec()), None)
				.await?
				.unwrap_or(axia_primitives::v1::LOWEST_PUBLIC_ID)
				.max(axia_primitives::v1::LOWEST_PUBLIC_ID);
			log::info!(target: "bridge", "Going to reserve allychain id: {:?}", ally_id);

			// step 1: reserve a allychain id
			let relay_genesis_hash = *relay_client.genesis_hash();
			let relay_sudo_account: AccountIdOf<Relaychain> = relay_sign.public().into();
			let reserve_allychain_id_call: CallOf<Relaychain> =
				ParaRegistrarCall::reserve {}.into();
			let reserve_allychain_signer = relay_sign.clone();
			wait_until_transaction_is_finalized::<Relaychain>(
				relay_client
					.submit_and_watch_signed_extrinsic(
						relay_sudo_account.clone(),
						move |_, transaction_nonce| {
							Bytes(
								Relaychain::sign_transaction(
									relay_genesis_hash,
									&reserve_allychain_signer,
									relay_axlib_client::TransactionEra::immortal(),
									UnsignedTransaction::new(
										reserve_allychain_id_call,
										transaction_nonce,
									),
								)
								.encode(),
							)
						},
					)
					.await?,
			)
			.await?;
			log::info!(target: "bridge", "Reserved allychain id: {:?}", ally_id);

			// step 2: register allythread
			let para_genesis_header = para_client.header_by_number(Zero::zero()).await?;
			let para_code = para_client
				.raw_storage_value(StorageKey(CODE.to_vec()), Some(para_genesis_header.hash()))
				.await?
				.ok_or_else(|| {
					anyhow::format_err!("Cannot fetch validation code of {}", Allychain::NAME)
				})?
				.0;
			log::info!(
				target: "bridge",
				"Going to register allychain {:?}: genesis len = {} code len = {}",
				ally_id,
				para_genesis_header.encode().len(),
				para_code.len(),
			);
			let register_allythread_call: CallOf<Relaychain> = ParaRegistrarCall::register {
				id: ally_id,
				genesis_head: ParaHeadData(para_genesis_header.encode()),
				validation_code: ParaValidationCode(para_code),
			}
			.into();
			let register_allythread_signer = relay_sign.clone();
			wait_until_transaction_is_finalized::<Relaychain>(
				relay_client
					.submit_and_watch_signed_extrinsic(
						relay_sudo_account.clone(),
						move |_, transaction_nonce| {
							Bytes(
								Relaychain::sign_transaction(
									relay_genesis_hash,
									&register_allythread_signer,
									relay_axlib_client::TransactionEra::immortal(),
									UnsignedTransaction::new(
										register_allythread_call,
										transaction_nonce,
									),
								)
								.encode(),
							)
						},
					)
					.await?,
			)
			.await?;
			log::info!(target: "bridge", "Registered allychain: {:?}. Waiting for onboarding", ally_id);

			// wait until allythread is onboarded
			let para_state_key = bp_runtime::storage_map_final_key_twox64_concat(
				PARAS_PALLET_NAME,
				PARAS_LIFECYCLES_STORAGE_NAME,
				&ally_id.encode(),
			);
			wait_para_state(
				&relay_client,
				&para_state_key.0,
				&[ParaLifecycle::Onboarding, ParaLifecycle::Allythread],
				ParaLifecycle::Allythread,
			)
			.await?;

			// step 3: force allychain leases
			let lease_begin = self.lease_begin;
			let lease_end = self.lease_end;
			let para_deposit = self.deposit.cast().into();
			log::info!(
				target: "bridge",
				"Going to force leases of allychain {:?}: [{}; {}]",
				ally_id,
				lease_begin,
				lease_end,
			);
			let force_lease_call: CallOf<Relaychain> = SudoCall::sudo {
				call: Box::new(
					ParaSlotsCall::force_lease {
						para: ally_id,
						leaser: relay_sudo_account.clone(),
						amount: para_deposit,
						period_begin: lease_begin,
						period_count: lease_end.saturating_sub(lease_begin).saturating_add(1),
					}
					.into(),
				),
			}
			.into();
			let force_lease_signer = relay_sign.clone();
			relay_client
				.submit_signed_extrinsic(relay_sudo_account.clone(), move |_, transaction_nonce| {
					Bytes(
						Relaychain::sign_transaction(
							relay_genesis_hash,
							&force_lease_signer,
							relay_axlib_client::TransactionEra::immortal(),
							UnsignedTransaction::new(force_lease_call, transaction_nonce),
						)
						.encode(),
					)
				})
				.await?;
			log::info!(target: "bridge", "Registered allychain leases: {:?}. Waiting for onboarding", ally_id);

			// wait until allychain is onboarded
			wait_para_state(
				&relay_client,
				&para_state_key.0,
				&[
					ParaLifecycle::Onboarding,
					ParaLifecycle::UpgradingAllythread,
					ParaLifecycle::Allythread,
				],
				ParaLifecycle::Allychain,
			)
			.await?;

			Ok(())
		})
	}
}

/// Wait until allychain state is changed.
async fn wait_para_state<Relaychain: Chain>(
	relay_client: &Client<Relaychain>,
	para_state_key: &[u8],
	from_states: &[ParaLifecycle],
	to_state: ParaLifecycle,
) -> anyhow::Result<()> {
	loop {
		let para_state: ParaLifecycle = relay_client
			.storage_value(StorageKey(para_state_key.to_vec()), None)
			.await?
			.ok_or_else(|| {
				anyhow::format_err!(
					"Cannot fetch next free allychain lifecycle from the runtime storage of {}",
					Relaychain::NAME,
				)
			})?;
		if para_state == to_state {
			log::info!(target: "bridge", "Allychain state is now: {:?}", to_state);
			return Ok(())
		}
		if !from_states.contains(&para_state) {
			return Err(anyhow::format_err!("Invalid allychain lifecycle: {:?}", para_state))
		}

		log::info!(target: "bridge", "Allychain state: {:?}. Waiting for {:?}", para_state, to_state);
		async_std::task::sleep(Relaychain::AVERAGE_BLOCK_INTERVAL).await;
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn register_rialto_allychain() {
		let register_allychain = RegisterAllychain::from_iter(vec![
			"register-allychain",
			"rialto-allychain",
			"--allychain-host",
			"127.0.0.1",
			"--allychain-port",
			"11949",
			"--relaychain-host",
			"127.0.0.1",
			"--relaychain-port",
			"9944",
			"--relaychain-signer",
			"//Alice",
			"--deposit",
			"42",
			"--lease-begin",
			"100",
			"--lease-end",
			"200",
		]);

		assert_eq!(
			register_allychain,
			RegisterAllychain {
				allychain: Allychain::RialtoAllychain,
				deposit: Balance(42),
				lease_begin: 100,
				lease_end: 200,
				relay_connection: RelaychainConnectionParams {
					relaychain_host: "127.0.0.1".into(),
					relaychain_port: 9944,
					relaychain_secure: false,
				},
				relay_sign: RelaychainSigningParams {
					relaychain_signer: Some("//Alice".into()),
					relaychain_signer_password: None,
					relaychain_signer_file: None,
					relaychain_signer_password_file: None,
					relaychain_transactions_mortality: None,
				},
				para_connection: AllychainConnectionParams {
					allychain_host: "127.0.0.1".into(),
					allychain_port: 11949,
					allychain_secure: false,
				},
			}
		);
	}
}
