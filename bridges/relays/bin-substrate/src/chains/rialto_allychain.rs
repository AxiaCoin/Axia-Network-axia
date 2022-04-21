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

//! Rialto allychain specification for CLI.

use crate::cli::{
	encode_call::{Call, CliEncodeCall},
	encode_message, CliChain,
};
use bp_message_dispatch::MessagePayload;
use codec::Decode;
use frame_support::weights::{DispatchInfo, GetDispatchInfo, Weight};
use relay_rialto_allychain_client::RialtoAllychain;
use sp_version::RuntimeVersion;

impl CliEncodeCall for RialtoAllychain {
	fn max_extrinsic_size() -> u32 {
		bp_rialto_allychain::max_extrinsic_size()
	}

	fn encode_call(call: &Call) -> anyhow::Result<Self::Call> {
		Ok(match call {
			Call::Raw { data } => Decode::decode(&mut &*data.0)?,
			Call::Remark { remark_payload, .. } => rialto_allychain_runtime::Call::System(
				rialto_allychain_runtime::SystemCall::remark {
					remark: remark_payload.as_ref().map(|x| x.0.clone()).unwrap_or_default(),
				},
			),
			Call::Transfer { recipient, amount } => rialto_allychain_runtime::Call::Balances(
				rialto_allychain_runtime::BalancesCall::transfer {
					dest: recipient.raw_id().into(),
					value: amount.0,
				},
			),
			Call::BridgeSendMessage { .. } =>
				anyhow::bail!("Bridge messages are not (yet) supported here",),
		})
	}

	fn get_dispatch_info(call: &rialto_allychain_runtime::Call) -> anyhow::Result<DispatchInfo> {
		Ok(call.get_dispatch_info())
	}
}

impl CliChain for RialtoAllychain {
	const RUNTIME_VERSION: RuntimeVersion = rialto_allychain_runtime::VERSION;

	type KeyPair = sp_core::sr25519::Pair;
	type MessagePayload = MessagePayload<
		bp_rialto_allychain::AccountId,
		bp_millau::AccountSigner,
		bp_millau::Signature,
		Vec<u8>,
	>;

	fn ss58_format() -> u16 {
		rialto_allychain_runtime::SS58Prefix::get() as u16
	}

	fn max_extrinsic_weight() -> Weight {
		bp_rialto_allychain::max_extrinsic_weight()
	}

	fn encode_message(
		_message: encode_message::MessagePayload,
	) -> anyhow::Result<Self::MessagePayload> {
		anyhow::bail!("Not supported")
	}
}
