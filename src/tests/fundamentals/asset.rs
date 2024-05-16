use crate::fundamentals::asset::*;
use xcm::latest::prelude::*;

#[test]
fn fungibles() {
	assert_eq!(EmptyAssets::get(), Assets::new());
	assert_eq!(NativeToken::get(), AssetId(Location::new(0, [])));
	assert_eq!(DotToken::get(), AssetId(Location::new(1, [])));
	assert_eq!(AmountInNativeToken::get(), (NativeToken::get(), 100u128).into());
	assert_eq!(AmountInDot::get(), (DotToken::get(), 100u128).into());
}

#[test]
fn nonfungibles() {
	assert_eq!(
		NftLocation::get(),
		Location::new(1, [Parachain(1000), PalletInstance(5), GeneralIndex(42)])
	);
	assert_eq!(Nft::get(), (NftLocation::get(), 69u64).into());
}