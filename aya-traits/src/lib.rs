#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]
#![feature(trivial_bounds)]

use fp_account::AccountId20;
use frame_support::{traits::ConstU32, BoundedVec};
use frame_system::offchain::SendSignedTransaction;
use scale_codec::{Decode, Encode};
use sp_core::crypto::{AccountId32, Pair, Public, Ss58Codec};
use sp_runtime::traits::SignedExtension;
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::MultiAddress;
use sp_runtime::{MultiSignature, MultiSigner};

pub mod sr25519 {
    use sp_application_crypto::sr25519;

    pub type AuthorityId = sr25519::Public;
    pub type AuthoritySignature = sr25519::Signature;
    pub type AuthorityPair = sr25519::Pair;
    pub type AuthorityMultiSignature = sp_runtime::MultiSignature;
    pub type AuthorityMultiSigner = sp_runtime::MultiSigner;
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
            // AccountIdType::AccountId20(_) => Err(TransactionHandlerError::InvalidAccountId),
            AccountIdType::AccountId32(_) => Ok(payload), // Dummy handling for compilation
            AccountIdType::AccountId20(_) => Err(TransactionHandlerError::InvalidAccountId),
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

fn handle_leader_event_transaction<
    T: frame_system::Config
        + frame_system::offchain::SigningTypes
        + OffchainBound
        + frame_system::offchain::CreateSignedTransaction<<T as frame_system::Config>::RuntimeCall>,
>(
    account_id: AccountId32,
    payload: BoundedVec<u8, ConstU32<1024>>,
    verifier: &impl SignatureVerifier,
) -> Result<BoundedVec<u8, ConstU32<1024>>, TransactionHandlerError> {
    let signer = frame_system::offchain::Signer::<T, T::AuthorityId>::any_account();

    // if let Some((_, result)) = signer.send_signed_transaction(|account| {
    //     let signature = verifier.sign_payload(payload.as_slice());
    //     let multi_signature = MultiSignature::Sr25519(signature);

    //     // Create the call to the pallet function
    //     let call = <T as frame_system::Config>::RuntimeCall::PalletName::submit_inclusion_transaction {
    //         events: vec![/* Populate your events here */],
    //     };

    //     call
    // }) {
    //     result.map_err(|_| TransactionHandlerError::SigningError)?;
    // }

    Ok(BoundedVec::default())
}

use frame_system::offchain::AppCrypto;
use frame_system::offchain::SendTransactionTypes;
use frame_system::offchain::SigningTypes;
pub trait CreateSignedTransaction<LocalCall>:
    SendTransactionTypes<LocalCall> + SigningTypes
{
    fn create_transaction<C: AppCrypto<Self::Public, Self::Signature>>(
        call: LocalCall,
        public: Self::Public,
        account: Self::AccountId,
        nonce: Self::Nonce,
    ) -> Option<(LocalCall, sr25519::AuthoritySignature)>;
}
// impl<T: frame_system::Config> SendSignedTransaction<T::RuntimeCall> for MyTransactionHandler {
//     type Result = Option<Result<(), ()>>;

//     fn send_signed_transaction(&self, f: impl Fn(&Account<T>) -> T::RuntimeCall) -> Self::Result {
//         let signer = frame_system::offchain::Signer::<T, T::AuthorityId>::any_account();
//         signer.send_signed_transaction(|account| f(account))
//     }

//     fn send_single_signed_transaction(
//         &self,
//         account: &Account<T>,
//         call: T::RuntimeCall,
//     ) -> Option<Result<(), ()>> {
//         let mut account_data = crate::Account::<T>::get(&account.id);
//         log::debug!(
//             target: "runtime::offchain",
//             "Creating signed transaction from account: {:?} (nonce: {:?})",
//             account.id,
//             account_data.nonce,
//         );
//         let (call, signature) = T::create_transaction::<sr25519::AuthorityId>(
//             call.into(),
//             account.public.clone(),
//             account.id.clone(),
//             account_data.nonce,
//         )?;
//         let res = SubmitTransaction::<T, T::RuntimeCall>::submit_transaction(call, Some(signature));

//         if res.is_ok() {
//             account_data.nonce += 1.into();
//             crate::Account::<T>::insert(&account.id, account_data);
//         }

//         Some(res)
//     }
// }

pub struct Account<T: frame_system::Config> {
    pub id: T::AccountId,
    pub public: sr25519::AuthorityMultiSigner,
}
