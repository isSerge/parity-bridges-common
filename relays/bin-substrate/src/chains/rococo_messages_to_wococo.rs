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

//! Rococo-to-Wococo messages sync entrypoint.

use frame_support::weights::Weight;
use messages_relay::relay_strategy::MixStrategy;
use relay_rococo_client::Rococo;
use relay_wococo_client::Wococo;
use substrate_relay_helper::messages_lane::SubstrateMessageLane;

/// Description of Rococo -> Wococo messages bridge.
#[derive(Clone, Debug)]
pub struct RococoMessagesToWococo;
substrate_relay_helper::generate_mocked_receive_message_proof_call_builder!(
	RococoMessagesToWococo,
	RococoMessagesToWococoReceiveMessagesProofCallBuilder,
	relay_wococo_client::runtime::Call::BridgeRococoMessages,
	relay_wococo_client::runtime::BridgeRococoMessagesCall::receive_messages_proof
);
substrate_relay_helper::generate_mocked_receive_message_delivery_proof_call_builder!(
	RococoMessagesToWococo,
	RococoMessagesToWococoReceiveMessagesDeliveryProofCallBuilder,
	relay_rococo_client::runtime::Call::BridgeWococoMessages,
	relay_rococo_client::runtime::BridgeWococoMessagesCall::receive_messages_delivery_proof
);

impl SubstrateMessageLane for RococoMessagesToWococo {
	const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;
	const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None;

	type SourceChain = Rococo;
	type TargetChain = Wococo;

	type SourceTransactionSignScheme = Rococo;
	type TargetTransactionSignScheme = Wococo;

	type ReceiveMessagesProofCallBuilder = RococoMessagesToWococoReceiveMessagesProofCallBuilder;
	type ReceiveMessagesDeliveryProofCallBuilder =
		RococoMessagesToWococoReceiveMessagesDeliveryProofCallBuilder;

	type RelayStrategy = MixStrategy;
}
