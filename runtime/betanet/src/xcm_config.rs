// Copyright 2022 Axia Technologies (UK) Ltd.
// This file is part of Axia.

// Axia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Axia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Axia.  If not, see <http://www.gnu.org/licenses/>.

//! XCM configuration for Betanet.

use super::{
	allychains_origin, AccountId, Balances, Call, Event, Origin, AllyId, Runtime, WeightToFee,
	XcmPallet,
};
use frame_support::{
	parameter_types,
	traits::{Everything, IsInVec, Nothing},
	weights::Weight,
};
use runtime_common::{xcm_sender, ToAuthor};
use sp_std::prelude::*;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom, BackingToPlurality,
	ChildAllychainAsNative, ChildAllychainConvertsVia, ChildSystemAllychainAsSuperuser,
	CurrencyAdapter as XcmCurrencyAdapter, FixedWeightBounds, IsConcrete, LocationInverter,
	SignedAccountId32AsNative, SignedToAccountId32, SovereignSignedViaLocation, UsingComponents,
};

parameter_types! {
	pub const RocLocation: MultiLocation = Here.into();
	pub const BetanetNetwork: NetworkId = NetworkId::Axia;
	pub const Ancestry: MultiLocation = Here.into();
	pub CheckAccount: AccountId = XcmPallet::check_account();
}

pub type SovereignAccountOf =
	(ChildAllychainConvertsVia<AllyId, AccountId>, AccountId32Aliases<BetanetNetwork, AccountId>);

pub type LocalAssetTransactor = XcmCurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	IsConcrete<RocLocation>,
	// We can convert the MultiLocations with our converter above:
	SovereignAccountOf,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// It's a native asset so we keep track of the teleports to maintain total issuance.
	CheckAccount,
>;

type LocalOriginConverter = (
	SovereignSignedViaLocation<SovereignAccountOf, Origin>,
	ChildAllychainAsNative<allychains_origin::Origin, Origin>,
	SignedAccountId32AsNative<BetanetNetwork, Origin>,
	ChildSystemAllychainAsSuperuser<AllyId, Origin>,
);

parameter_types! {
	pub const BaseXcmWeight: Weight = 1_000_000_000;
}

/// The XCM router. When we want to send an XCM message, we use this type. It amalgamates all of our
/// individual routers.
pub type XcmRouter = (
	// Only one router so far - use DMP to communicate with child allychains.
	xcm_sender::ChildAllychainRouter<Runtime, XcmPallet>,
);

parameter_types! {
	pub const Betanet: MultiAssetFilter = Wild(AllOf { fun: WildFungible, id: Concrete(RocLocation::get()) });
	pub const BetanetForTick: (MultiAssetFilter, MultiLocation) = (Betanet::get(), Allychain(100).into());
	pub const BetanetForTrick: (MultiAssetFilter, MultiLocation) = (Betanet::get(), Allychain(110).into());
	pub const BetanetForTrack: (MultiAssetFilter, MultiLocation) = (Betanet::get(), Allychain(120).into());
	pub const BetanetForStatemine: (MultiAssetFilter, MultiLocation) = (Betanet::get(), Allychain(1000).into());
	pub const BetanetForCanvas: (MultiAssetFilter, MultiLocation) = (Betanet::get(), Allychain(1002).into());
	pub const BetanetForEncointer: (MultiAssetFilter, MultiLocation) = (Betanet::get(), Allychain(1003).into());
	pub const MaxInstructions: u32 = 100;
}
pub type TrustedTeleporters = (
	xcm_builder::Case<BetanetForTick>,
	xcm_builder::Case<BetanetForTrick>,
	xcm_builder::Case<BetanetForTrack>,
	xcm_builder::Case<BetanetForStatemine>,
	xcm_builder::Case<BetanetForCanvas>,
	xcm_builder::Case<BetanetForEncointer>,
);

parameter_types! {
	pub AllowUnpaidFrom: Vec<MultiLocation> =
		vec![
			Allychain(100).into(),
			Allychain(110).into(),
			Allychain(120).into(),
			Allychain(1000).into(),
			Allychain(1002).into(),
			Allychain(1003).into(),
		];
}

use xcm_builder::{AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, TakeWeightCredit};
pub type Barrier = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	AllowUnpaidExecutionFrom<IsInVec<AllowUnpaidFrom>>, // <- Trusted allychains get free execution
	// Expected responses are OK.
	AllowKnownQueryResponses<XcmPallet>,
	// Subscriptions for version tracking are OK.
	AllowSubscriptionsFrom<Everything>,
);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type Call = Call;
	type XcmSender = XcmRouter;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = LocalOriginConverter;
	type IsReserve = ();
	type IsTeleporter = TrustedTeleporters;
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	type Trader = UsingComponents<WeightToFee, RocLocation, AccountId, Balances, ToAuthor<Runtime>>;
	type ResponseHandler = XcmPallet;
	type AssetTrap = XcmPallet;
	type AssetClaims = XcmPallet;
	type SubscriptionService = XcmPallet;
}

parameter_types! {
	pub const CollectiveBodyId: BodyId = BodyId::Unit;
}

/// Type to convert an `Origin` type value into a `MultiLocation` value which represents an interior location
/// of this chain.
pub type LocalOriginToLocation = (
	// We allow an origin from the Collective pallet to be used in XCM as a corresponding Plurality of the
	// `Unit` body.
	BackingToPlurality<Origin, pallet_collective::Origin<Runtime>, CollectiveBodyId>,
	// And a usual Signed origin to be used in XCM as a corresponding AccountId32
	SignedToAccountId32<Origin, AccountId, BetanetNetwork>,
);

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	// Anyone can execute XCM messages locally...
	type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	// ...but they must match our filter, which right now rejects everything.
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = xcm_executor::XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	type LocationInverter = LocationInverter<Ancestry>;
	type Origin = Origin;
	type Call = Call;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}
