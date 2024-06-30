use frame_support::construct_runtime;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};
use pallet_epoch;
use substrate_validator_set;
use sp_core::sr25519;
use sp_core::H256;
use frame_system as system;
use frame_support::parameter_types;
use sp_runtime::traits::OpaqueKeys;
use sp_runtime::testing::UintAuthorityId;
use sp_runtime::impl_opaque_keys;
use std::collections::BTreeMap;
use pallet_session::SessionManager;
use sp_runtime::RuntimeAppPublic;

type Block = frame_system::mocking::MockBlock<Test>;

// Define the missing types
pub struct TestShouldEndSession;
impl pallet_session::ShouldEndSession<u64> for TestShouldEndSession {
    fn should_end_session(now: u64) -> bool {
        false
    }
}

pub struct TestSessionHandler;
impl pallet_session::SessionHandler<u64> for TestSessionHandler {
    const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[UintAuthorityId::ID];

    fn on_genesis_session<T: OpaqueKeys>(_validators: &[(u64, T)]) {}
    fn on_new_session<T: OpaqueKeys>(_changed: bool, _validators: &[(u64, T)], _queued_validators: &[(u64, T)]) {}
    fn on_disabled(_validator_index: u32) {}
    fn on_before_session_ending() {}
}

// Implement the Config trait for pallet_session
impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = u64;
    type ValidatorIdOf = substrate_validator_set::ValidatorOf<Self>;
    type ShouldEndSession = TestShouldEndSession;
    type NextSessionRotation = ();
    type SessionManager = substrate_validator_set::Pallet<Self>;
    type SessionHandler = TestSessionHandler;
    type Keys = MockSessionKeys;
    type WeightInfo = ();
}

// Implement the Config trait for substrate_validator_set
impl substrate_validator_set::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AddRemoveOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type MinAuthorities = frame_support::traits::ConstU32<1>;
    type WeightInfo = ();
}

// Configure a mock runtime to test the pallet.
construct_runtime!(
    pub enum Test {
        System: frame_system,
        ValidatorSet: substrate_validator_set,
        Session: pallet_session,
        // Epoch: pallet_epoch::{Pallet, Call, Storage, Event<T>},
    }
);

impl_opaque_keys! {
    pub struct MockSessionKeys {
        pub dummy: UintAuthorityId,
    }
}

pub struct MockAccountId32Convert;

impl MockAccountId32Convert {
    fn into_account_id(_account: sp_runtime::AccountId32) -> u64 {
        0 // For testing purposes, always return 0
    }

    fn into_account_id32(_account: u64) -> sp_runtime::AccountId32 {
        sp_runtime::AccountId32::new([0; 32]) // For testing purposes, return a zero-filled AccountId32
    }
}

parameter_types! {
    pub static Validators: Vec<u64> = vec![1, 2, 3];
    pub static NextValidators: Vec<u64> = vec![1, 2, 3];
    pub static Authorities: Vec<UintAuthorityId> =
        vec![UintAuthorityId(1), UintAuthorityId(2), UintAuthorityId(3)];
    pub static ForceSessionEnd: bool = false;
    pub static SessionLength: u64 = 2;
    pub static SessionChanged: bool = false;
    pub static TestSessionChanged: bool = false;
    pub static Disabled: bool = false;
    pub static BeforeSessionEndCalled: bool = false;
    pub static ValidatorAccounts: BTreeMap<u64, u64> = BTreeMap::new();
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = frame_support::traits::ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = frame_support::traits::ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
    
    // New associated types
    type RuntimeTask = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
