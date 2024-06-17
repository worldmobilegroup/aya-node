use crate as pallet_epoch;
use frame_support::{parameter_types, traits::ConstU64};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    testing::Header,
    Perbill, Percent,
};
use frame_system::mocking::{MockBlock, MockUncheckedExtrinsic};
use frame_support::traits::PalletInfo;
use sp_runtime::BuildStorage;
use frame_support::traits::PalletInfoAccess;
use frame_system;
use pallet_epoch as epoch;
use substrate_validator_set;
// Define configuration parameters
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: u64 = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::one();
    pub const EpochDuration: u64 = 5;
    pub const MinimumPeriod: u64 = 5;
    pub const MinAuthorities: u32 = 2;
}


type Block = frame_system::mocking::MockBlock<Test>;

// Declare the `Test` runtime
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Epoch: pallet_epoch,
        ValidatorSet: substrate_validator_set,
    }
);

// // Declare the `Test` runtime
// frame_support::construct_runtime!(
//     pub enum Test //where
//         // Block = Block,
//         // NodeBlock = Block,
//         // UncheckedExtrinsic = UncheckedExtrinsic,
//     {
//         System: frame_system,
//         // Epoch: pallet_epoch,
//         // ValidatorSet: substrate_validator_set,
//     }
// );

// // Define types
type UncheckedExtrinsic<Test> = MockUncheckedExtrinsic<Test>;
type Block<Test> = MockBlock<Test>;

// impl frame_system::Config for Test {
//     type BaseCallFilter = frame_support::traits::Everything;
//     type BlockWeights = ();
//     type BlockLength = ();
//     type DbWeight = ();
//     type RuntimeOrigin = Self::RuntimeOrigin;
//     type RuntimeCall = Self::RuntimeCall;
//     type AccountId = u64;
//     type Lookup = IdentityLookup<Self::AccountId>;
//     // type Header = Header;
//     type RuntimeEvent = RuntimeEvent;
//     type BlockHashCount = BlockHashCount;
//     type Version = ();
//     type PalletInfo = PalletInfo;
//     type AccountData = ();
//     type OnNewAccount = ();
//     type OnKilledAccount = ();
//     type SystemWeightInfo = ();
//     type SS58Prefix = ();
//     type OnSetCode = ();
//     type MaxConsumers = frame_support::traits::ConstU32<16>;

//     // Add the missing types
//     type RuntimeTask = ();
//     type Nonce = u64;
//     type Hash = H256;
//     type Hashing = BlakeTwo256;
//     type Block = Block<Self>;
//     type SingleBlockMigrations = ();
//     type MultiBlockMigrator = ();
//     type PreInherents = ();
//     type PostInherents = ();
//     type PostTransactions = ();
// }

// // Implement `substrate_validator_set::Config` for `Test`
// impl substrate_validator_set::Config for Test {
//     type RuntimeEvent = RuntimeEvent;
//     type AddRemoveOrigin = frame_system::EnsureRoot<u64>;
//     type MinAuthorities = MinAuthorities;
//     type WeightInfo = ();
// }

// // Implement `pallet_epoch::Config` for `Test`
// impl pallet_epoch::Config for Test {
//     type EpochDuration = EpochDuration;
//     type RuntimeEvent = RuntimeEvent;
//     type ValidatorId = u64;
//     type AuthorityId = u64;
//     type WeightInfo = ();
// }

// // Define the `new_test_ext` function
// pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
//     let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
//     t.into()
// }
