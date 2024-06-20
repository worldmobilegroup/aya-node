use crate as pallet_epoch;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

use frame_support::construct_runtime;
use sp_io::TestExternalities;

type UncheckedExtrinsic<Runtime> = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block<Runtime> = frame_system::mocking::MockBlock<Runtime>;
type RuntimeOrigin<Test> = frame_system::pallet::Origin<Test>;
type RuntimeCall<Test> = frame_system::pallet::Call<Test>;
type RuntimeEvent<Test> = frame_system::pallet::Event<Test>;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

// pub fn new_test_ext() -> sp_io::TestExternalities {
//     let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
//     t.into()
// }
