#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "512"]
#![feature(trivial_bounds)]

use fp_account::AccountId20;
use frame_support::{traits::ConstU32, BoundedVec};
use scale_codec::{Decode, Encode};
use sp_core::crypto::{AccountId32, Pair, Public, Ss58Codec};
use sp_runtime::traits::{IdentifyAccount, Verify};
use sp_runtime::{MultiSignature, MultiSigner};

pub mod sr25519 {
    pub mod app_sr25519 {
        use sp_application_crypto::{app_crypto, key_types::AURA, sr25519};
        app_crypto!(sr25519, AURA);
    }

    pub type AuthorityId = app_sr25519::Public;
    pub type AuthoritySignature = app_sr25519::Signature;
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

#[derive(Debug)]
pub enum TransactionHandlerError {
    InvalidAccountId,
    SigningError,
}

pub trait TransactionHandler {
    fn handle_transaction(
        account_id: AccountIdType,
        payload: BoundedVec<u8, ConstU32<1024>>,
    ) -> Result<BoundedVec<u8, ConstU32<1024>>, TransactionHandlerError>;
}

pub struct MyTransactionHandler;

impl TransactionHandler for MyTransactionHandler {
    fn handle_transaction(
        account_id: AccountIdType,
        payload: BoundedVec<u8, ConstU32<1024>>,
    ) -> Result<BoundedVec<u8, ConstU32<1024>>, TransactionHandlerError> {
        let verifier = StubSignatureVerifier;
        match account_id {
            AccountIdType::AccountId32(id) => handle_substrate_transaction::<YourRuntime>(id, payload, &verifier),
            AccountIdType::AccountId20(id) => handle_evm_transaction(id, payload),
        }
    }
}

pub trait SignatureVerifier {
    fn sign_payload(&self, payload: &[u8]) -> sr25519::Signature;
}

pub struct StubSignatureVerifier;

impl SignatureVerifier for StubSignatureVerifier {
    fn sign_payload(&self, _payload: &[u8]) -> sr25519::Signature {
        // Return a dummy signature
        sr25519::Signature::from_raw([0u8; 64])
    }
}

use sp_runtime::traits::SignedExtension;
use sp_runtime::MultiAddress;
use frame_system::offchain::SendSignedTransaction;

fn handle_substrate_transaction<T: frame_system::Config + frame_system::offchain::SigningTypes + OffchainBound>(
    account_id: AccountId32,
    payload: BoundedVec<u8, ConstU32<1024>>,
    verifier: &impl SignatureVerifier,
) -> Result<BoundedVec<u8, ConstU32<1024>>, TransactionHandlerError> {
    // Get the signer
    let signer = frame_system::offchain::Signer::<T, T::AuthorityId>::any_account();
    if let Some((_, result)) = signer.send_signed_transaction(|_account| {
        // Use the verifier to sign the payload
        let signature = verifier.sign_payload(payload.as_slice());
        let multi_signature = MultiSignature::Sr25519(signature);

        sp_runtime::generic::UncheckedExtrinsic {
            signature: Some((MultiAddress::Id(account_id.clone()), multi_signature, ())),
            function: payload.encode(), // Assuming payload is the function data
        }
    }) {
        result.map_err(|_| TransactionHandlerError::SigningError)?;
    }

    // Return a dummy BoundedVec
    Ok(BoundedVec::default())
}

fn handle_evm_transaction(
    account_id: AccountId20,
    payload: BoundedVec<u8, ConstU32<1024>>,
) -> Result<BoundedVec<u8, ConstU32<1024>>, TransactionHandlerError> {
    Ok(payload)
}

fn handle_leader_event_transaction<T: frame_system::Config + frame_system::offchain::SigningTypes + OffchainBound>(
    account_id: AccountId32,
    payload: BoundedVec<u8, ConstU32<1024>>,
    verifier: &impl SignatureVerifier,
) -> Result<BoundedVec<u8, ConstU32<1024>>, TransactionHandlerError> {
    // Get the signer
    let signer = frame_system::offchain::Signer::<T, T::AuthorityId>::any_account();
    if let Some((_, result)) = signer.send_signed_transaction(|_account| {
        // Use the verifier to sign the payload
        let signature = verifier.sign_payload(payload.as_slice());
        let multi_signature = MultiSignature::Sr25519(signature);

        sp_runtime::generic::UncheckedExtrinsic {
            signature: Some((MultiAddress::Id(account_id.clone()), multi_signature, ())),
            function: payload.encode(), // Assuming payload is the function data
        }
    }) {
        result.map_err(|_| TransactionHandlerError::SigningError)?;
    }

    // Return a dummy BoundedVec
    Ok(BoundedVec::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    struct Runtime;

    impl frame_system::Config for Runtime {
        type BaseCallFilter = ();
        type BlockWeights = ();
        type BlockLength = ();
        type DbWeight = ();
        type Origin = ();
        type RuntimeCall = ();
        type RuntimeEvent = ();
        type RuntimeBlockNumber = ();
        type Hash = ();
        type Hashing = ();
        type AccountId = ();
        type Lookup = ();
        type Index = ();
        type BlockNumber = ();
        type Header = ();
        type RuntimeVersion = ();
        type PalletInfo = ();
        type AccountData = ();
        type OnNewAccount = ();
        type OnKilledAccount = ();
        type SystemWeightInfo = ();
        type SS58Prefix = ();
        type OnSetCode = ();
        type MaxConsumers = ();
    }

    impl frame_system::offchain::SigningTypes for Runtime {
        type Public = <MultiSignature as Verify>::Signer;
        type Signature = MultiSignature;
    }

    impl OffchainBound for Runtime {
        type AuthorityId = sr25519::AuthorityId;
    }

    #[test]
    fn test_handle_substrate_transaction() {
        let account_id =
            AccountId32::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
                .unwrap();
        let payload = BoundedVec::try_from(vec![1, 2, 3, 4]).unwrap();
        let verifier = StubSignatureVerifier;
        let result = handle_leader_event_transaction::<Runtime>(account_id, payload.clone(), &verifier);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_evm_transaction() {
        let account_id = AccountId20::from(hex!("d43593c715fdd31c61141abd04a99fd6822c8558"));
        let payload = BoundedVec::try_from(vec![1, 2, 3, 4]).unwrap();
        let result = handle_evm_transaction(account_id, payload.clone());
        assert!(result.is_ok());
    }
}
