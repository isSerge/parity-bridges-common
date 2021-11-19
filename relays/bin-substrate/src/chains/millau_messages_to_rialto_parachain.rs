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

//! Millau-to-RialtoParachain messages sync entrypoint.

use codec::Encode;
use sp_core::{Bytes, Pair};

use messages_relay::relay_strategy::MixStrategy;
use relay_millau_client::Millau;
use relay_rialto_parachain_client::RialtoParachain;
use relay_substrate_client::{Client, TransactionSignScheme, UnsignedTransaction};
use substrate_relay_helper::messages_lane::{
	DirectReceiveMessagesDeliveryProofCallBuilder, DirectReceiveMessagesProofCallBuilder,
	SubstrateMessageLane,
};

/// Description of Millau -> RialtoParachain messages bridge.
#[derive(Clone, Debug)]
pub struct MillauMessagesToRialtoParachain;

impl SubstrateMessageLane for MillauMessagesToRialtoParachain {
	const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None; // TODO
	const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None; // TODO

	type SourceChain = Millau;
	type TargetChain = RialtoParachain;

	type SourceTransactionSignScheme = Millau;
	type TargetTransactionSignScheme = RialtoParachain;

	type ReceiveMessagesProofCallBuilder = DirectReceiveMessagesProofCallBuilder<
		Self,
		rialto_parachain_runtime::Runtime,
		rialto_parachain_runtime::WithMillauMessagesInstance,
	>;
	type ReceiveMessagesDeliveryProofCallBuilder = DirectReceiveMessagesDeliveryProofCallBuilder<
		Self,
		millau_runtime::Runtime,
		millau_runtime::WithRialtoParachainMessagesInstance,
	>;

	type RelayStrategy = MixStrategy;
}

/// Update RialtoParachain -> Millau conversion rate, stored in Millau runtime storage.
pub(crate) async fn update_rialto_parachain_to_millau_conversion_rate(
	_client: Client<Millau>,
	_signer: <Millau as TransactionSignScheme>::AccountKeyPair,
	_updated_rate: f64,
) -> anyhow::Result<()> {
	Ok(())
}
