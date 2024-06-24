use frame_support::{parameter_types, traits::{ConstU32, ConstU64}};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup, Verify},
    testing::Header,
    BuildStorage, MultiSignature,
};
use sp_application_crypto::sr25519;
use frame_support::weights::Weight;
use frame_system::{self as system, Config as OtherSystemConfig};  // Alias the Config trait
use sp_runtime::Perbill;
use frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND;

type UncheckedExtrinsic<Test> = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block<Test> = frame_system::mocking::MockBlock<Test>;



const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
    pub const BlockHashCount: u32 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
        ::with_sensible_defaults(
            Weight::from_parts(2 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX), // 2 seconds of compute
            NORMAL_DISPATCH_RATIO,
        );
    pub const SS58Prefix: u8 = 42;
}


frame_support::construct_runtime!(
    pub enum Runtime {
        System: frame_system::{Pallet, Call, Event<T>, Config<T>} = 0,
    }
);

impl frame_system::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type BaseCallFilter = ();
    type BlockWeights = BlockWeights;
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeTask = ();
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block<Self>;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}
pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_config_builds() {
        new_test_ext().execute_with(|| {
            assert!(System::block_number() == 0);
        });
    }
}

