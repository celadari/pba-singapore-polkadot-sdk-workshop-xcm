use crate::asset_hub::MessageQueue;
use frame_support::parameter_types;
use xcm::latest::prelude::*;

parameter_types! {
	pub KsmPerSecondPerByte: (AssetId, u128, u128) = (AssetId(Parent.into()), 1, 1);
	pub const MaxAssetsIntoHolding: u32 = 64;
}

// You are a parachain on Kusama, these are fixed constants for you.
parameter_types! {
	pub const KsmLocation: Location = Location::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Kusama;
	pub UniversalLocation: InteriorLocation = [GlobalConsensus(RelayNetwork::get()), Parachain(MessageQueue::parachain_id().into())].into();
}
