// convert_validator.rs

use sp_core::H160;
use sp_runtime::AccountId32;
use fp_account::AccountId20;

pub trait ConvertValidatorId {
    fn from_account_id20(account_id: AccountId20) -> AccountId32;
    fn into_account_id20(account_id: AccountId32) -> AccountId20;
}

pub struct ValidatorIdConverter;

impl ConvertValidatorId for ValidatorIdConverter {
    fn from_account_id20(account_id: AccountId20) -> AccountId32 {
        let mut bytes = [0u8; 32];
        bytes[0..20].copy_from_slice(&account_id.as_ref());
        AccountId32::from(bytes)
    }

    fn into_account_id20(account_id: AccountId32) -> AccountId20 {
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&<AccountId32 as AsRef<[u8]>>::as_ref(&account_id)[0..20]);
        AccountId20::from(H160::from_slice(&bytes))
    }
}
