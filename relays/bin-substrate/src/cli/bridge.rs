// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

use strum::{EnumString, EnumVariantNames};

#[derive(Debug, PartialEq, Eq, EnumString, EnumVariantNames)]
#[strum(serialize_all = "kebab_case")]
/// Supported full bridges (headers + messages).
pub enum FullBridge {
	MillauToRialto,
	RialtoToMillau,
	RococoToWococo,
	WococoToRococo,
	KusamaToPolkadot,
	PolkadotToKusama,
}

impl FullBridge {
	/// Return instance index of the bridge pallet in source runtime.
	pub fn bridge_instance_index(&self) -> u8 {
		match self {
			Self::MillauToRialto => MILLAU_TO_RIALTO_INDEX,
			Self::RialtoToMillau => RIALTO_TO_MILLAU_INDEX,
			Self::RococoToWococo => ROCOCO_TO_WOCOCO_INDEX,
			Self::WococoToRococo => WOCOCO_TO_ROCOCO_INDEX,
			Self::KusamaToPolkadot => KUSAMA_TO_POLKADOT_INDEX,
			Self::PolkadotToKusama => POLKADOT_TO_KUSAMA_INDEX,
		}
	}
}

pub const RIALTO_TO_MILLAU_INDEX: u8 = 0;
pub const MILLAU_TO_RIALTO_INDEX: u8 = 0;
pub const ROCOCO_TO_WOCOCO_INDEX: u8 = 0;
pub const WOCOCO_TO_ROCOCO_INDEX: u8 = 0;
pub const KUSAMA_TO_POLKADOT_INDEX: u8 = 0;
pub const POLKADOT_TO_KUSAMA_INDEX: u8 = 0;

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
				use crate::chains::millau_messages_to_rialto::MillauMessagesToRialto as MessagesLane;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_rialto::TO_RIALTO_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use millau_runtime::millau_to_rialto_account_ownership_digest as account_ownership_digest;

				#[allow(dead_code)]
				const SOURCE_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(millau_runtime::VERSION);
				#[allow(dead_code)]
				const TARGET_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(rialto_runtime::VERSION);

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
				use crate::chains::rialto_messages_to_millau::RialtoMessagesToMillau as MessagesLane;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_millau::TO_MILLAU_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;

				// Send-message
				#[allow(unused_imports)]
				use rialto_runtime::rialto_to_millau_account_ownership_digest as account_ownership_digest;

				#[allow(dead_code)]
				const SOURCE_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(rialto_runtime::VERSION);
				#[allow(dead_code)]
				const TARGET_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(millau_runtime::VERSION);

				$generic
			}
			FullBridge::RococoToWococo => {
				type Source = relay_rococo_client::Rococo;
				#[allow(dead_code)]
				type Target = relay_wococo_client::Wococo;

				// Derive-account
				#[allow(unused_imports)]
				use bp_wococo::derive_account_from_rococo_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::rococo_messages_to_wococo::RococoMessagesToWococo as MessagesLane;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_wococo::TO_WOCOCO_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_rococo_client::runtime::rococo_to_wococo_account_ownership_digest as account_ownership_digest;

				#[allow(dead_code)]
				const SOURCE_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(bp_rococo::VERSION);
				#[allow(dead_code)]
				const TARGET_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(bp_wococo::VERSION);

				$generic
			}
			FullBridge::WococoToRococo => {
				type Source = relay_wococo_client::Wococo;
				#[allow(dead_code)]
				type Target = relay_rococo_client::Rococo;

				// Derive-account
				#[allow(unused_imports)]
				use bp_rococo::derive_account_from_wococo_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::wococo_messages_to_rococo::WococoMessagesToRococo as MessagesLane;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_rococo::TO_ROCOCO_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_wococo_client::runtime::wococo_to_rococo_account_ownership_digest as account_ownership_digest;

				#[allow(dead_code)]
				const SOURCE_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(bp_wococo::VERSION);
				#[allow(dead_code)]
				const TARGET_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(bp_rococo::VERSION);

				$generic
			}
			FullBridge::KusamaToPolkadot => {
				type Source = relay_kusama_client::Kusama;
				#[allow(dead_code)]
				type Target = relay_polkadot_client::Polkadot;

				// Derive-account
				#[allow(unused_imports)]
				use bp_polkadot::derive_account_from_kusama_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::kusama_messages_to_polkadot::KusamaMessagesToPolkadot as MessagesLane;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_polkadot::TO_POLKADOT_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_kusama_client::runtime::kusama_to_polkadot_account_ownership_digest as account_ownership_digest;

				#[allow(dead_code)]
				const SOURCE_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(bp_kusama::VERSION);
				#[allow(dead_code)]
				const TARGET_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(bp_polkadot::VERSION);

				$generic
			}
			FullBridge::PolkadotToKusama => {
				type Source = relay_polkadot_client::Polkadot;
				#[allow(dead_code)]
				type Target = relay_kusama_client::Kusama;

				// Derive-account
				#[allow(unused_imports)]
				use bp_kusama::derive_account_from_polkadot_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::polkadot_messages_to_kusama::PolkadotMessagesToKusama as MessagesLane;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_kusama::TO_KUSAMA_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_polkadot_client::runtime::polkadot_to_kusama_account_ownership_digest as account_ownership_digest;

				#[allow(dead_code)]
				const SOURCE_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(bp_polkadot::VERSION);
				#[allow(dead_code)]
				const TARGET_RUNTIME_VERSION: Option<sp_version::RuntimeVersion> = Some(bp_kusama::VERSION);

				$generic
			}
		}
	};
}
