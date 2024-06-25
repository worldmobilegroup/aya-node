use crate::validator_set;
use fp_account::AccountId20;
use frame_support::weights::constants::WEIGHT_REF_TIME_PER_SECOND;
use frame_support::weights::Weight;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64},
};
use frame_system::{self as system, Config as OtherSystemConfig}; // Alias the Config trait
use scale_codec::{Decode, Encode};
use sp_application_crypto::ed25519::Signature;
use sp_application_crypto::sr25519;
use sp_core::H256;
use sp_runtime::traits::ConvertInto;
use sp_runtime::Perbill;
use sp_runtime::{
    impl_opaque_keys,
    testing::Header,
    testing::UintAuthorityId,
    traits::OpaqueKeys,
    traits::{BlakeTwo256, IdentityLookup, Verify},
    BuildStorage, KeyTypeId, MultiSignature, RuntimeAppPublic,
};
use sp_state_machine::BasicExternalities;
use std::collections::BTreeMap;

type AccountId = AccountId20;
type Block = frame_system::mocking::MockBlock<Runtime>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
impl_opaque_keys! {
    pub struct MockSessionKeys {
        pub dummy: UintAuthorityId,
    }
}

impl From<UintAuthorityId> for MockSessionKeys {
    fn from(dummy: UintAuthorityId) -> Self {
        Self { dummy }
    }
}

pub const KEY_ID_A: KeyTypeId = KeyTypeId([4; 4]);
pub const KEY_ID_B: KeyTypeId = KeyTypeId([9; 4]);

#[derive(Debug, Clone, scale_codec::Encode, scale_codec::Decode, PartialEq, Eq)]
pub struct PreUpgradeMockSessionKeys {
    pub a: [u8; 32],
    pub b: [u8; 64],
}

impl OpaqueKeys for PreUpgradeMockSessionKeys {
    type KeyTypeIdProviders = ();

    fn key_ids() -> &'static [KeyTypeId] {
        &[KEY_ID_A, KEY_ID_B]
    }

    fn get_raw(&self, i: KeyTypeId) -> &[u8] {
        match i {
            i if i == KEY_ID_A => &self.a[..],
            i if i == KEY_ID_B => &self.b[..],
            _ => &[],
        }
    }
}

pub type SessionManager = ();

// impl From<pallet_session::Event<Runtime>> for RuntimeEvent {
//     fn from(event: pallet_session::Event<Runtime>) -> Self {
//         RuntimeEvent::Session(event)
//     }
// }

// #[derive(Clone, PartialEq, Eq, Debug, Encode, Decode)]
// pub enum RuntimeEvent {
//     System(frame_system::Event<Runtime>),
//     Session(pallet_session::Event<Runtime>),
//     ValidatorSet(validator_set::Event<Runtime>),
// }

// impl From<frame_system::Event<Runtime>> for RuntimeEvent {
//     fn from(event: frame_system::Event<Runtime>) -> Self {
//         RuntimeEvent::System(event)
//     }
// }

// impl From<pallet_session::Event<Runtime>> for RuntimeEvent {
//     fn from(event: pallet_session::Event<Runtime>) -> Self {
//         RuntimeEvent::Session(event)
//     }
// }

// impl From<validator_set::Event<Runtime>> for RuntimeEvent {
//     fn from(event: validator_set::Event<Runtime>) -> Self {
//         RuntimeEvent::ValidatorSet(event)
//     }
// }

pub struct DummyShouldEndSession;

impl<BlockNumber: PartialEq + From<u32>> pallet_session::ShouldEndSession<BlockNumber>
    for DummyShouldEndSession
{
    fn should_end_session(_now: BlockNumber) -> bool {
        false
    }
}

pub struct DummySessionManager;

impl pallet_session::SessionManager<AccountId> for DummySessionManager {
    fn new_session(_new_index: u32) -> Option<Vec<AccountId>> {
        None
    }
    fn end_session(_end_index: u32) {}
    fn start_session(_start_index: u32) {}
}

parameter_types! {
    pub const Period: u64 = 1;
    pub const Offset: u64 = 0;
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
}

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
        System: frame_system,
        Session: pallet_session,
    }
);

impl pallet_session::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ConvertInto;
    type ShouldEndSession = DummyShouldEndSession;
    type NextSessionRotation = ();
    type SessionManager = DummySessionManager;
    type SessionHandler = TestSessionHandler;
    type Keys = MockSessionKeys;
    type WeightInfo = ();
}

// impl pallet_session::SessionManager<AccountId> for () {
//     fn new_session(_new_index: u32) -> Option<Vec<AccountId>> {
//         None
//     }
//     fn end_session(_end_index: u32) {}
//     fn start_session(_start_index: u32) {}
// }

pub struct TestSessionHandler;
impl pallet_session::SessionHandler<AccountId> for TestSessionHandler {
    const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[UintAuthorityId::ID];
    fn on_genesis_session<T: OpaqueKeys>(_validators: &[(AccountId, T)]) {}
    fn on_new_session<T: OpaqueKeys>(
        _changed: bool,
        _validators: &[(AccountId, T)],
        _queued_validators: &[(AccountId, T)],
    ) {
    }
    fn on_disabled(_validator_index: u32) {}
}

impl SendTransactionTypes<pallet_epoch::Call<Runtime>> for Runtime {
    type OverarchingCall = RuntimeCall;
    type Extrinsic = UncheckedExtrinsic;
}

// impl From<validator_set::Event<Runtime>> for RuntimeEvent {
//     fn from(event: validator_set::Event<Runtime>) -> Self {
//         RuntimeEvent::ValidatorSet(event)
//     }
// }

// impl validator_set::Config for Runtime {
//     type RuntimeEvent = RuntimeEvent;
//     type AddRemoveOrigin = frame_system::EnsureRoot<Self::AccountId>;
//     type MinAuthorities = ConstU32<1>;
//     type WeightInfo = ();
// }

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
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
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

pub type GenesisConfig = frame_system::GenesisConfig<Runtime>;
