#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// use sp_runtime::traits::{IdentifyAccount, Verify};
// use fp_account::AccountId20;
// use frame_support::{traits::ConstU32, BoundedVec};
// use scale_codec::{Decode, Encode};
// use sp_core::crypto::{AccountId32, Pair, Public, Ss58Codec};
// use sp_runtime::{MultiSignature, MultiSigner};
// use frame_system::offchain::SendSignedTransaction;
// use sp_runtime::traits::SignedExtension;
// use sp_runtime::MultiAddress;

// #[derive(
//     Default,
//     Deserialize,
//     Serialize,
//     Encode,
//     Decode,
//     Clone,
//     PartialEq,
//     Eq,
//     TypeInfo,
//     MaxEncodedLen,
//     RuntimeDebug,
// )]
// pub struct CustomEvent {
//     pub id: u64,
//     pub data: BoundedVec<u8, ConstU32<1024>>,
//     pub timestamp: u64,
//     pub block_height: u64,
//     pub last_epoch: u64,
//     pub last_blockhash: BoundedVec<u8, ConstU32<1024>>,
//     pub last_slot: u64,
//     pub new_epoch: u64,
//     pub new_slot: u64,
//     pub new_blockhash: BoundedVec<u8, ConstU32<1024>>,
//     pub epoch_nonce: BoundedVec<u8, ConstU32<1024>>,
//     pub extra_entropy: Option<BoundedVec<u8, ConstU32<1024>>>,
// }

// impl CustomEvent {
//     pub fn new(
//         id: u64,
//         data: Vec<u8>,
//         timestamp: u64,
//         block_height: u64,
//         last_epoch: u64,
//         last_blockhash: Vec<u8>,
//         last_slot: u64,
//         new_epoch: u64,
//         new_slot: u64,
//         new_blockhash: Vec<u8>,
//         epoch_nonce: Vec<u8>,
//         extra_entropy: Option<Vec<u8>>,
//     ) -> Result<Self, &'static str> {
//         Ok(CustomEvent {
//             id,
//             data: BoundedVec::try_from(data).map_err(|_| "Data exceeds maximum length")?,
//             timestamp,
//             block_height,
//             last_epoch,
//             last_blockhash: BoundedVec::try_from(last_blockhash)
//                 .map_err(|_| "Last blockhash exceeds maximum length")?,
//             last_slot,
//             new_epoch,
//             new_slot,
//             new_blockhash: BoundedVec::try_from(new_blockhash)
//                 .map_err(|_| "New blockhash exceeds maximum length")?,
//             epoch_nonce: BoundedVec::try_from(epoch_nonce)
//                 .map_err(|_| "Epoch nonce exceeds maximum length")?,
//             extra_entropy: extra_entropy
//                 .map(|e| BoundedVec::try_from(e).map_err(|_| "Extra entropy exceeds maximum length"))
//                 .transpose()?,
//         })
//     }
// }
