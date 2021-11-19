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

//! RialtoParachain-to-Millau messages sync entrypoint.

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

/// Description of RialtoParachain -> Millau messages bridge.
#[derive(Clone, Debug)]
pub struct RialtoParachainMessagesToMillau;

impl SubstrateMessageLane for RialtoParachainMessagesToMillau {
	const SOURCE_TO_TARGET_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None; // TODO
	const TARGET_TO_SOURCE_CONVERSION_RATE_PARAMETER_NAME: Option<&'static str> = None; // TODO

	type SourceChain = RialtoParachain;
	type TargetChain = Millau;

	type SourceTransactionSignScheme = RialtoParachain;
	type TargetTransactionSignScheme = Millau;

	type ReceiveMessagesProofCallBuilder = DirectReceiveMessagesProofCallBuilder<
		Self,
		millau_runtime::Runtime,
		millau_runtime::WithRialtoParachainMessagesInstance,
	>;
	type ReceiveMessagesDeliveryProofCallBuilder = DirectReceiveMessagesDeliveryProofCallBuilder<
		Self,
		rialto_parachain_runtime::Runtime,
		rialto_parachain_runtime::WithMillauMessagesInstance,
	>;

	type RelayStrategy = MixStrategy;
}

/// Update Millau -> RialtoParachain conversion rate, stored in RialtoParachain runtime storage.
pub(crate) async fn update_millau_to_rialto_parachain_conversion_rate(
	_client: Client<RialtoParachain>,
	_signer: <RialtoParachain as TransactionSignScheme>::AccountKeyPair,
	_updated_rate: f64,
) -> anyhow::Result<()> {
	Ok(())
}
