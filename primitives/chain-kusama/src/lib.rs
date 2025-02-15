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

#![cfg_attr(not(feature = "std"), no_std)]
// RuntimeApi generated functions
#![allow(clippy::too_many_arguments)]
// Runtime-generated DecodeLimit::decode_all_with_depth_limit
#![allow(clippy::unnecessary_mut_passed)]

use bp_messages::{LaneId, MessageDetails, MessageNonce, UnrewardedRelayersState};
use frame_support::weights::{
	WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
};
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub use bp_polkadot_core::*;

/// Kusama Chain
pub type Kusama = PolkadotLike;

// NOTE: This needs to be kept up to date with the Kusama runtime found in the Polkadot repo.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: sp_version::create_runtime_str!("kusama"),
	impl_name: sp_version::create_runtime_str!("parity-kusama"),
	authoring_version: 2,
	spec_version: 9140,
	impl_version: 0,
	apis: sp_version::create_apis_vec![[]],
	transaction_version: 8,
	state_version: 0,
};

// NOTE: This needs to be kept up to date with the Kusama runtime found in the Polkadot repo.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		const CENTS: Balance = 1_000_000_000_000 / 30_000;
		// in Kusama, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
		let p = CENTS;
		let q = 10 * Balance::from(ExtrinsicBaseWeight::get());
		smallvec::smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

// We use this to get the account on Kusama (target) which is derived from Polkadot's (source)
// account.
pub fn derive_account_from_polkadot_id(id: bp_runtime::SourceAccount<AccountId>) -> AccountId {
	let encoded_id = bp_runtime::derive_account_id(bp_runtime::POLKADOT_CHAIN_ID, id);
	AccountIdConverter::convert(encoded_id)
}

/// Per-byte fee for Kusama transactions.
pub const TRANSACTION_BYTE_FEE: Balance = 10 * 1_000_000_000_000 / 30_000 / 1_000;

/// Existential deposit on Kusama.
pub const EXISTENTIAL_DEPOSIT: Balance = 1_000_000_000_000 / 30_000;

/// The target length of a session (how often authorities change) on Kusama measured in of number of
/// blocks.
///
/// Note that since this is a target sessions may change before/after this time depending on network
/// conditions.
pub const SESSION_LENGTH: BlockNumber = time_units::HOURS;

/// Name of the With-Kusama GRANDPA pallet instance that is deployed at bridged chains.
pub const WITH_KUSAMA_GRANDPA_PALLET_NAME: &str = "BridgeKusamaGrandpa";
/// Name of the With-Kusama messages pallet instance that is deployed at bridged chains.
pub const WITH_KUSAMA_MESSAGES_PALLET_NAME: &str = "BridgeKusamaMessages";

/// Name of the DOT->KSM conversion rate stored in the Kusama runtime.
pub const POLKADOT_TO_KUSAMA_CONVERSION_RATE_PARAMETER_NAME: &str =
	"PolkadotToKusamaConversionRate";

/// Name of the `KusamaFinalityApi::best_finalized` runtime method.
pub const BEST_FINALIZED_KUSAMA_HEADER_METHOD: &str = "KusamaFinalityApi_best_finalized";

/// Name of the `ToKusamaOutboundLaneApi::estimate_message_delivery_and_dispatch_fee` runtime
/// method.
pub const TO_KUSAMA_ESTIMATE_MESSAGE_FEE_METHOD: &str =
	"ToKusamaOutboundLaneApi_estimate_message_delivery_and_dispatch_fee";
/// Name of the `ToKusamaOutboundLaneApi::message_details` runtime method.
pub const TO_KUSAMA_MESSAGE_DETAILS_METHOD: &str = "ToKusamaOutboundLaneApi_message_details";

/// Name of the `FromKusamaInboundLaneApi::unrewarded_relayers_state` runtime method.
pub const FROM_KUSAMA_UNREWARDED_RELAYERS_STATE: &str =
	"FromKusamaInboundLaneApi_unrewarded_relayers_state";

sp_api::decl_runtime_apis! {
	/// API for querying information about the finalized Kusama headers.
	///
	/// This API is implemented by runtimes that are bridging with the Kusama chain, not the
	/// Kusama runtime itself.
	pub trait KusamaFinalityApi {
		/// Returns number and hash of the best finalized header known to the bridge module.
		fn best_finalized() -> (BlockNumber, Hash);
	}

	/// Outbound message lane API for messages that are sent to Kusama chain.
	///
	/// This API is implemented by runtimes that are sending messages to Kusama chain, not the
	/// Kusama runtime itself.
	pub trait ToKusamaOutboundLaneApi<OutboundMessageFee: Parameter, OutboundPayload: Parameter> {
		/// Estimate message delivery and dispatch fee that needs to be paid by the sender on
		/// this chain.
		///
		/// Returns `None` if message is too expensive to be sent to Kusama from this chain.
		///
		/// Please keep in mind that this method returns the lowest message fee required for message
		/// to be accepted to the lane. It may be good idea to pay a bit over this price to account
		/// future exchange rate changes and guarantee that relayer would deliver your message
		/// to the target chain.
		fn estimate_message_delivery_and_dispatch_fee(
			lane_id: LaneId,
			payload: OutboundPayload,
		) -> Option<OutboundMessageFee>;
		/// Returns dispatch weight, encoded payload size and delivery+dispatch fee of all
		/// messages in given inclusive range.
		///
		/// If some (or all) messages are missing from the storage, they'll also will
		/// be missing from the resulting vector. The vector is ordered by the nonce.
		fn message_details(
			lane: LaneId,
			begin: MessageNonce,
			end: MessageNonce,
		) -> Vec<MessageDetails<OutboundMessageFee>>;
	}

	/// Inbound message lane API for messages sent by Kusama chain.
	///
	/// This API is implemented by runtimes that are receiving messages from Kusama chain, not the
	/// Kusama runtime itself.
	pub trait FromKusamaInboundLaneApi {
		/// State of the unrewarded relayers set at given lane.
		fn unrewarded_relayers_state(lane: LaneId) -> UnrewardedRelayersState;
	}
}
