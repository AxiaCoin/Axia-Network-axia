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

use codec::Decode;
use frame_support::weights::{DispatchClass, DispatchInfo, Pays, Weight};
use relay_axctest_client::AxiaTest;
use sp_core::storage::StorageKey;
use sp_runtime::{FixedPointNumber, FixedU128};
use sp_version::RuntimeVersion;

use crate::cli::{
	bridge,
	encode_call::{Call, CliEncodeCall},
	encode_message, CliChain,
};

/// Weight of the `system::remark` call at AxiaTest.
///
/// This weight is larger (x2) than actual weight at current AxiaTest runtime to avoid unsuccessful
/// calls in the future. But since it is used only in tests (and on test chains), this is ok.
pub(crate) const SYSTEM_REMARK_CALL_WEIGHT: Weight = 2 * 1_345_000;

/// Id of AxiaTest token that is used to fetch token price.
pub(crate) const TOKEN_ID: &str = "axctest";

impl CliEncodeCall for AxiaTest {
	fn max_extrinsic_size() -> u32 {
		bp_axctest::max_extrinsic_size()
	}

	fn encode_call(call: &Call) -> anyhow::Result<Self::Call> {
		Ok(match call {
			Call::Remark { remark_payload, .. } => relay_axctest_client::runtime::Call::System(
				relay_axctest_client::runtime::SystemCall::remark(
					remark_payload.as_ref().map(|x| x.0.clone()).unwrap_or_default(),
				),
			),
			Call::BridgeSendMessage { lane, payload, fee, bridge_instance_index } =>
				match *bridge_instance_index {
					bridge::AXIATEST_TO_AXIA_INDEX => {
						let payload = Decode::decode(&mut &*payload.0)?;
						relay_axctest_client::runtime::Call::BridgeAxiaMessages(
							relay_axctest_client::runtime::BridgeAxiaMessagesCall::send_message(
								lane.0, payload, fee.0,
							),
						)
					},
					_ => anyhow::bail!(
						"Unsupported target bridge pallet with instance index: {}",
						bridge_instance_index
					),
				},
			_ => anyhow::bail!("Unsupported AxiaTest call: {:?}", call),
		})
	}

	fn get_dispatch_info(
		call: &relay_axctest_client::runtime::Call,
	) -> anyhow::Result<DispatchInfo> {
		match *call {
			relay_axctest_client::runtime::Call::System(
				relay_axctest_client::runtime::SystemCall::remark(_),
			) => Ok(DispatchInfo {
				weight: crate::chains::axctest::SYSTEM_REMARK_CALL_WEIGHT,
				class: DispatchClass::Normal,
				pays_fee: Pays::Yes,
			}),
			_ => anyhow::bail!("Unsupported AxiaTest call: {:?}", call),
		}
	}
}

impl CliChain for AxiaTest {
	const RUNTIME_VERSION: RuntimeVersion = bp_axctest::VERSION;

	type KeyPair = sp_core::sr25519::Pair;
	type MessagePayload = ();

	fn ss58_format() -> u16 {
		42
	}

	fn max_extrinsic_weight() -> Weight {
		bp_axctest::max_extrinsic_weight()
	}

	fn encode_message(
		_message: encode_message::MessagePayload,
	) -> anyhow::Result<Self::MessagePayload> {
		anyhow::bail!("Sending messages from AxiaTest is not yet supported.")
	}
}

/// Storage key and initial value of Axia -> AxiaTest conversion rate.
pub(crate) fn axia_to_axctest_conversion_rate_params() -> (StorageKey, FixedU128) {
	(
		bp_runtime::storage_parameter_key(
			bp_axctest::AXIA_TO_AXIATEST_CONVERSION_RATE_PARAMETER_NAME,
		),
		// starting relay before this parameter will be set to some value may cause troubles
		FixedU128::from_inner(FixedU128::DIV),
	)
}
