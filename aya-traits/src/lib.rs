#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]
#![feature(trivial_bounds)]
pub trait OffchainBound: frame_system::Config + frame_system::offchain::SigningTypes {
    type AuthorityId: frame_system::offchain::AppCrypto<
        <Self as frame_system::offchain::SigningTypes>::Public,
        <Self as frame_system::offchain::SigningTypes>::Signature,
    >;
}