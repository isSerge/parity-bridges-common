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

use anyhow::anyhow;
use codec::Decode;
use frame_support::weights::{DispatchClass, DispatchInfo, Pays, Weight};
use relay_rococo_client::Rococo;
use sp_version::RuntimeVersion;

use crate::cli::{
	bridge,
	encode_call::{Call, CliEncodeCall},
	encode_message, CliChain,
};

/// Weight of the `system::remark` call at Rococo.
///
/// This weight is larger (x2) than actual weight at current Rococo runtime to avoid unsuccessful
/// calls in the future. But since it is used only in tests (and on test chains), this is ok.
pub(crate) const SYSTEM_REMARK_CALL_WEIGHT: Weight = 2 * 1_345_000;

impl CliEncodeCall for Rococo {
	fn encode_call(call: &Call) -> anyhow::Result<Self::Call> {
		Ok(match call {
			Call::Remark { remark_payload, .. } => relay_rococo_client::runtime::Call::System(
				relay_rococo_client::runtime::SystemCall::remark(
					remark_payload.as_ref().map(|x| x.0.clone()).unwrap_or_default(),
				),
			),
			Call::BridgeSendMessage { lane, payload, fee, bridge_instance_index } =>
				match *bridge_instance_index {
					bridge::ROCOCO_TO_WOCOCO_INDEX => {
						let payload = Decode::decode(&mut &*payload.0)?;
						relay_rococo_client::runtime::Call::BridgeWococoMessages(
							relay_rococo_client::runtime::BridgeWococoMessagesCall::send_message(
								lane.0, payload, fee.0,
							),
						)
					},
					_ => anyhow::bail!(
						"Unsupported target bridge pallet with instance index: {}",
						bridge_instance_index
					),
				},
			_ => anyhow::bail!("The call is not supported"),
		})
	}

	fn get_dispatch_info(
		call: &relay_rococo_client::runtime::Call,
	) -> anyhow::Result<DispatchInfo> {
		match *call {
			relay_rococo_client::runtime::Call::System(
				relay_rococo_client::runtime::SystemCall::remark(_),
			) => Ok(DispatchInfo {
				weight: SYSTEM_REMARK_CALL_WEIGHT,
				class: DispatchClass::Normal,
				pays_fee: Pays::Yes,
			}),
			_ => anyhow::bail!("Unsupported Rococo call: {:?}", call),
		}
	}
}

impl CliChain for Rococo {
	const RUNTIME_VERSION: RuntimeVersion = bp_rococo::VERSION;

	type KeyPair = sp_core::sr25519::Pair;
	type MessagePayload = ();

	fn ss58_format() -> u16 {
		42
	}

	fn encode_message(
		_message: encode_message::MessagePayload,
	) -> anyhow::Result<Self::MessagePayload> {
		Err(anyhow!("Sending messages from Rococo is not yet supported."))
	}
}
