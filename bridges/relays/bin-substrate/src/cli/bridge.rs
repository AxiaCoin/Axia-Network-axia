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

use strum::{EnumString, EnumVariantNames};

#[derive(Debug, PartialEq, Eq, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
/// Supported full bridges (headers + messages).
pub enum FullBridge {
	MillauToRialto,
	RialtoToMillau,
	BetanetToWococo,
	WococoToBetanet,
	AxiaTestToAxia,
	AxiaToAxiaTest,
}

impl FullBridge {
	/// Return instance index of the bridge pallet in source runtime.
	pub fn bridge_instance_index(&self) -> u8 {
		match self {
			Self::MillauToRialto => MILLAU_TO_RIALTO_INDEX,
			Self::RialtoToMillau => RIALTO_TO_MILLAU_INDEX,
			Self::BetanetToWococo => BETANET_TO_WOCOCO_INDEX,
			Self::WococoToBetanet => WOCOCO_TO_BETANET_INDEX,
			Self::AxiaTestToAxia => AXIATEST_TO_AXIA_INDEX,
			Self::AxiaToAxiaTest => AXIA_TO_AXIATEST_INDEX,
		}
	}
}

pub const RIALTO_TO_MILLAU_INDEX: u8 = 0;
pub const MILLAU_TO_RIALTO_INDEX: u8 = 0;
pub const BETANET_TO_WOCOCO_INDEX: u8 = 0;
pub const WOCOCO_TO_BETANET_INDEX: u8 = 0;
pub const AXIATEST_TO_AXIA_INDEX: u8 = 0;
pub const AXIA_TO_AXIATEST_INDEX: u8 = 0;

/// The macro allows executing bridge-specific code without going fully generic.
///
/// It matches on the [`FullBridge`] enum, sets bridge-specific types or imports and injects
/// the `$generic` code at every variant.
#[macro_export]
macro_rules! select_full_bridge {
	($bridge: expr, $generic: tt) => {
		match $bridge {
			FullBridge::MillauToRialto => {
				type Source = relay_millau_client::Millau;
				#[allow(dead_code)]
				type Target = relay_rialto_client::Rialto;

				// Derive-account
				#[allow(unused_imports)]
				use bp_rialto::derive_account_from_millau_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::millau_messages_to_rialto::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_rialto::TO_RIALTO_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use millau_runtime::millau_to_rialto_account_ownership_digest as account_ownership_digest;

				$generic
			}
			FullBridge::RialtoToMillau => {
				type Source = relay_rialto_client::Rialto;
				#[allow(dead_code)]
				type Target = relay_millau_client::Millau;

				// Derive-account
				#[allow(unused_imports)]
				use bp_millau::derive_account_from_rialto_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::rialto_messages_to_millau::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_millau::TO_MILLAU_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;

				// Send-message
				#[allow(unused_imports)]
				use rialto_runtime::rialto_to_millau_account_ownership_digest as account_ownership_digest;

				$generic
			}
			FullBridge::BetanetToWococo => {
				type Source = relay_betanet_client::Betanet;
				#[allow(dead_code)]
				type Target = relay_wococo_client::Wococo;

				// Derive-account
				#[allow(unused_imports)]
				use bp_wococo::derive_account_from_betanet_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::betanet_messages_to_wococo::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_wococo::TO_WOCOCO_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_betanet_client::runtime::betanet_to_wococo_account_ownership_digest as account_ownership_digest;

				$generic
			}
			FullBridge::WococoToBetanet => {
				type Source = relay_wococo_client::Wococo;
				#[allow(dead_code)]
				type Target = relay_betanet_client::Betanet;

				// Derive-account
				#[allow(unused_imports)]
				use bp_betanet::derive_account_from_wococo_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::wococo_messages_to_betanet::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_betanet::TO_BETANET_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_wococo_client::runtime::wococo_to_betanet_account_ownership_digest as account_ownership_digest;

				$generic
			}
			FullBridge::AxiaTestToAxia => {
				type Source = relay_axctest_client::AxiaTest;
				#[allow(dead_code)]
				type Target = relay_axia_client::Axia;

				// Derive-account
				#[allow(unused_imports)]
				use bp_axia::derive_account_from_axctest_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::axctest_messages_to_axia::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_axia::TO_AXIA_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_axctest_client::runtime::axctest_to_axia_account_ownership_digest as account_ownership_digest;

				$generic
			}
			FullBridge::AxiaToAxiaTest => {
				type Source = relay_axia_client::Axia;
				#[allow(dead_code)]
				type Target = relay_axctest_client::AxiaTest;

				// Derive-account
				#[allow(unused_imports)]
				use bp_axctest::derive_account_from_axia_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::axia_messages_to_axctest::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_axctest::TO_AXIATEST_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_axia_client::runtime::axia_to_axctest_account_ownership_digest as account_ownership_digest;

				$generic
			}
		}
	};
}
