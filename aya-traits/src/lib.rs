#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]
#![feature(trivial_bounds)]
use sp_runtime::traits::{IdentifyAccount, Verify};

use fp_account::AccountId20;
use frame_support::{traits::ConstU32, BoundedVec};
use scale_codec::{Decode, Encode};
use sp_core::crypto::{AccountId32, Pair, Public, Ss58Codec};

use sp_runtime::{MultiSignature, MultiSigner};
pub mod sr25519 {
    use sp_application_crypto::sr25519;

    pub type AuthorityId = sr25519::Public;
    pub type AuthoritySignature = sr25519::Signature;
}

pub trait OffchainBound: frame_system::Config + frame_system::offchain::SigningTypes {
    type AuthorityId: frame_system::offchain::AppCrypto<
        <Self as frame_system::offchain::SigningTypes>::Public,
        <Self as frame_system::offchain::SigningTypes>::Signature,
    >;
}

#[derive(Debug)]
pub enum AccountIdType {
    AccountId32(AccountId32),
    AccountId20(AccountId20),
}


pub trait TransactionHandler {
    fn handle_transaction(
        account_id: AccountIdType,
        payload: BoundedVec<u8, ConstU32<1024>>,
    ) -> Result<BoundedVec<u8, ConstU32<1024>>, TransactionHandlerError>;
}
#[derive(Debug)]
pub enum TransactionHandlerError {
    InvalidAccountId,
    SigningError,
}



pub struct MyTransactionHandler;

impl TransactionHandler for MyTransactionHandler {
    fn handle_transaction(
        account_id: AccountIdType,
        payload: BoundedVec<u8, ConstU32<1024>>,
    ) -> Result<BoundedVec<u8, ConstU32<1024>>, TransactionHandlerError> {
        let verifier = StubSignatureVerifier;
        match account_id {
            // AccountIdType::AccountId32(id) => handle_leader_event_transaction::<Runtime>(id, payload, &verifier),
            // AccountIdType::AccountId20(id) => handle_evm_transaction(id, payload),
            AccountIdType::AccountId32(_) | AccountIdType::AccountId20(_) => todo!(),
        }
    }
}

pub trait SignatureVerifier {
    fn sign_payload(&self, payload: &[u8]) -> sr25519::AuthoritySignature;
}

pub struct StubSignatureVerifier;

impl SignatureVerifier for StubSignatureVerifier {
    fn sign_payload(&self, _payload: &[u8]) -> sr25519::AuthoritySignature {
        // Return a dummy signature
        sr25519::AuthoritySignature::from_raw([0u8; 64])
    }
}



use sp_runtime::traits::SignedExtension;
use sp_runtime::MultiAddress;
use frame_system::offchain::SendSignedTransaction;

fn handle_leader_event_transaction<T: frame_system::Config + frame_system::offchain::SigningTypes + OffchainBound>(
    account_id: AccountId32,
    payload: BoundedVec<u8, ConstU32<1024>>,
    verifier: &impl SignatureVerifier,
) -> Result<BoundedVec<u8, ConstU32<1024>>, TransactionHandlerError> {
    // // Get the signer
    // let signer = frame_system::offchain::Signer::<T, T::AuthorityId>::any_account();
    // if let Some((_, result)) = signer.send_signed_transaction(|_account| {
    //     // Use the verifier to sign the payload
    //     let signature = verifier.sign_payload(payload.as_slice());
    //     let multi_signature = MultiSignature::Sr25519(signature);

    //     sp_runtime::generic::UncheckedExtrinsic {
    //         signature: Some((MultiAddress::Id(account_id.clone()), multi_signature, ())),
    //         function: payload.encode(), // Assuming payload is the function data
    //     }
    // }) {
    //     result.map_err(|_| TransactionHandlerError::SigningError)?;
    // }

    // Return a dummy BoundedVec
    Ok(BoundedVec::default())
}








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
