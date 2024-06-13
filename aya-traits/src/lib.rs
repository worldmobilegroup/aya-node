#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]
#![feature(trivial_bounds)]
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_core::crypto::AccountId32;
use fp_account::AccountId20;
use sp_runtime::MultiSigner;
use frame_support::BoundedVec;

pub trait OffchainBound: frame_system::Config + frame_system::offchain::SigningTypes {
    type AuthorityId: frame_system::offchain::AppCrypto<
        <Self as frame_system::offchain::SigningTypes>::Public,
        <Self as frame_system::offchain::SigningTypes>::Signature,
    >;
}
// pub trait TransactionHandler {
//     fn handle_transaction(
//         account_id: AccountIdType,
//         payload: Vec<u8>
//     ) -> Result<Vec<u8>, TransactionHandlerError>;
// }


// pub enum TransactionHandlerError {
//     InvalidAccountId,
//     SigningError,
// }

// pub trait TransactionHandler {
//     fn handle_transaction(account_id: AccountIdType, payload: Vec<u8>) -> Result<Vec<u8>, TransactionHandlerError>;
// }

#[derive(Debug)]
pub enum AccountIdType {
    AccountId32(AccountId32),
    AccountId20(AccountId20),
}

pub struct MyTransactionHandler;

// impl TransactionHandler for MyTransactionHandler {
//     fn handle_transaction(account_id: AccountIdType, payload: Vec<u8>) -> Result<Vec<u8>, TransactionHandlerError> {
//         match account_id {
//             AccountIdType::AccountId32(id) => handle_substrate_transaction(id, payload),
//             AccountIdType::AccountId20(id) => handle_evm_transaction(id, payload),
//         }
//     }
// }

// fn handle_substrate_transaction(account_id: AccountId32, payload: Vec<u8>) -> Result<Vec<u8>, TransactionHandlerError> {
//     // Implement the logic to create and sign a Substrate transaction
//     Ok(payload)
// }

// fn handle_evm_transaction(account_id: AccountId20, payload: Vec<u8>) -> Result<Vec<u8>, TransactionHandlerError> {
//     // Implement the logic to create and sign an EVM transaction
//     Ok(payload)
// }
