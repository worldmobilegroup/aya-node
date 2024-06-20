// use crate::mock::{new_test_ext, Test}; // Import the mock runtime
// use crate::pallet::{CustomEvent, EventStorage};
// use frame_support::assert_ok;
// use sp_io::TestExternalities;
// use sp_runtime::traits::Hash;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use sp_core::offchain::{
//         testing::{self, OffchainState, TestOffchainExt},
//         OffchainDbExt,
//     };
//     use crate::rt_offchain::OffchainWorkerExt;

//     // Helper function to set up the test environment
//     fn setup_ext() -> TestExternalities {
//         let (offchain, _) = testing::TestOffchainExt::new();
//         let mut ext = TestExternalities::default();
//         ext.register_extension(OffchainWorkerExt(Box::new(offchain)));
//         ext
//     }

//     #[test]
//     fn test_store_event_data() {
//         new_test_ext().execute_with(|| {
//             let event = CustomEvent::new(
//                 1,
//                 vec![1, 2, 3],
//                 1625068800,
//                 100,
//                 1,
//                 vec![4, 5, 6],
//                 1,
//                 2,
//                 3,
//                 vec![7, 8, 9],
//                 vec![10, 11, 12],
//                 None,
//             ).expect("Failed to create event");

//             assert_ok!(Pallet::<Test>::store_event_in_mempool(event.clone()));
//             let stored_event = EventStorage::<Test>::get(event.id);
//             assert_eq!(stored_event, event);
//         });
//     }

//     #[test]
//     fn test_offchain_worker_process() {
//         new_test_ext().execute_with(|| {
//             let event = CustomEvent::new(
//                 1,
//                 vec![1, 2, 3],
//                 1625068800,
//                 100,
//                 1,
//                 vec![4, 5, 6],
//                 1,
//                 2,
//                 3,
//                 vec![7, 8, 9],
//                 vec![10, 11, 12],
//                 None,
//             ).expect("Failed to create event");

//             assert_ok!(Pallet::<Test>::store_event_in_mempool(event.clone()));
//             Pallet::<Test>::offchain_worker(1);
//             assert!(EventStorage::<Test>::get(event.id).is_none());
//         });
//     }
// }
