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

type UncheckedExtrinsic<Test> = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block<Test> = frame_system::mocking::MockBlock<Test>;
// type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;

parameter_types! {
    pub const BlockHashCount: u32 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::simple_max(Weight::from_parts(1024, 0));
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




// parameter_types! {
//     pub const BlockHashCount: u32 = 250;
//     pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::simple_max(Weight::from_parts(1024, 0));
//     pub const SS58Prefix: u8 = 42;
// }

// impl frame_system::Config for Test {
//     type BaseCallFilter = ();
//     type BlockWeights = ();
//     type BlockLength = ();
//     type Origin = Origin;
//     type Index = u64;
//     type Call = RuntimeCall;
//     type Hash = H256;
//     type Hashing = BlakeTwo256;
//     type AccountId = u64;
//     type Lookup = IdentityLookup<Self::AccountId>;
//     type Header = Header;
//     type Event = RuntimeEvent;
//     type BlockNumber = u64;
//     type Version = ();
//     type PalletInfo = PalletInfo;
//     // type AccountData = pallet_balances::AccountData<u64>;
//     type OnNewAccount = ();
//     type OnKilledAccount = ();
//     type DbWeight = ();
//     type BlockHashCount = BlockHashCount;
//     type BlockHashMapping = ();
//     type BlockNumberFor = ();
//     type ExtrinsicBaseWeight = ();
//     type MaximumBlockWeight = BlockWeights;
//     type MaximumBlockLength = ();
//     type AvailableBlockRatio = ();
//     type Version = ();
//     type ModuleToIndex = ();
//     // type AccountData = pallet_balances::AccountData<u64>;
//     type OnNewAccount = ();
//     type OnKilledAccount = ();
//     type SystemWeightInfo = ();
// }

// impl substrate_validator_set::Config for Test {
//     type RuntimeEvent = RuntimeEvent;
//     type AddRemoveOrigin = frame_system::EnsureRoot<Self::AccountId>;
//     type MinAuthorities = ConstU32<1>;
//     type WeightInfo = ();
// }

// impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
// where
//     RuntimeCall: From<LocalCall>,
// {
//     fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
//         call: LocalCall,
//         _public: Self::Public,
//         _account: AccountId,
//         nonce: u64,
//     ) -> Option<(LocalCall, <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
//         Some((call, (nonce, ())))
//     }
// }

// impl frame_system::offchain::SigningTypes for Test {
//     type Public = <Signature as Verify>::Signer;
//     type Signature = Signature;
// }

// impl pallet_epoch::Config for Test {
//     type RuntimeEvent = RuntimeEvent;
//     type WeightInfo = ();
//     type AuthorityId = sr25519::Public;
//     type ValidatorId = u64;
//     type AccountId32Convert = sp_runtime::traits::Identity;
//     type Call = RuntimeCall;
//     type UnsignedPriority = ConstU64<1>;
// }

// pub fn new_test_ext() -> sp_io::TestExternalities {
//     frame_system::GenesisConfig::default().build_storage().unwrap().into()
// }