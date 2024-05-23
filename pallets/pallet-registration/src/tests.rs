use crate::{mock::*, Error, Event, Something};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
    new_test_ext().execute_with(|| {
        // Go past genesis block so events get deposited
        System::set_block_number(1);
        // Dispatch a signed extrinsic.
        assert_ok!(TemplateModule::do_something(RuntimeOrigin::signed(1), 42));
        // Read pallet storage and assert an expected result.
        assert_eq!(Something::<Test>::get(), Some(42));
        // Assert that the correct event was deposited
        System::assert_last_event(
            Event::SomethingStored {
                something: 42,
                who: 1,
            }
            .into(),
        );
    });
}

#[test]
fn correct_error_for_none_value() {
    new_test_ext().execute_with(|| {
        // Ensure the expected error is thrown when no value is present.
        assert_noop!(
            TemplateModule::cause_error(RuntimeOrigin::signed(1)),
            Error::<Test>::NoneValue
        );
    });
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use sp_core::offchain::{
//         testing::{self, OffchainState, OffchainWorkerExt, TestOffchainExt},
//         OffchainDbExt,
//     };
//     use sp_io::TestExternalities;

//     fn setup_ext() -> TestExternalities {
//         let (offchain, _) = testing::TestOffchainExt::new();
//         let mut ext = TestExternalities::default();
//         ext.register_extension(OffchainWorkerExt(offchain));
//         ext
//     }

//     #[test]
//     fn test_store_event_data() {
//         let mut ext = setup_ext();
//         ext.execute_with(|| {
//             // Assuming submit_cardano_event is a function callable here that triggers offchain storage logic
//             let event = Event { data: "Test event data".to_string() };
//             let event_str = serde_json::to_string(&event).unwrap();
//             submit_cardano_event(event_str).unwrap();

//             let storage_key = sp_io::hashing::blake2_128(b"cardano_events");

//             assert!(
//                 sp_io::offchain::local_storage_get(
//                     sp_runtime::offchain::StorageKind::PERSISTENT,
//                     &storage_key
//                 ).is_some(),
//                 "Event data should be stored"
//             );
//         });
//     }
// }
