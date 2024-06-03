use fp_account::AccountId20;
use frame_system::Config as SysConfig;
use sp_core::H160;
use sp_runtime::codec::{Decode, Encode};
use sp_runtime::AccountId32;

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct AccountId32Wrapper(pub AccountId32);

// Conversion trait for AccountId32Wrapper
pub trait FromAccountId32Wrapper<T: SysConfig> {
    fn from_wrapper(wrapper: AccountId32Wrapper) -> T::AccountId;
}

pub trait IntoAccountId32Wrapper<T: SysConfig> {
    fn into_wrapper(self) -> AccountId32Wrapper;
}

impl From<AccountId32> for AccountId32Wrapper {
    fn from(account_id32: AccountId32) -> Self {
        AccountId32Wrapper(account_id32)
    }
}

impl From<AccountId32Wrapper> for AccountId32 {
    fn from(wrapper: AccountId32Wrapper) -> Self {
        wrapper.0
    }
}

impl<T: SysConfig<AccountId = AccountId32>> FromAccountId32Wrapper<T> for AccountId32 {
    fn from_wrapper(wrapper: AccountId32Wrapper) -> Self {
        wrapper.0
    }
}

impl<T: SysConfig<AccountId = AccountId32>> IntoAccountId32Wrapper<T> for AccountId32 {
    fn into_wrapper(self) -> AccountId32Wrapper {
        AccountId32Wrapper(self)
    }
}

// Add implementations to convert between AccountId20 and AccountId32
pub trait FromAccountId20 {
    fn from_account_id20(account_id: AccountId20) -> Self;
}

pub trait IntoAccountId20 {
    fn into_account_id20(self) -> AccountId20;
}

impl FromAccountId20 for AccountId32 {
    fn from_account_id20(account_id: AccountId20) -> Self {
        let mut bytes = [0u8; 32];
        bytes[0..20].copy_from_slice(&account_id.as_ref());
        AccountId32::from(bytes)
    }
}

impl IntoAccountId20 for AccountId32 {
    fn into_account_id20(self) -> AccountId20 {
        let mut bytes = [0u8; 20];
        bytes.copy_from_slice(&<AccountId32 as AsRef<[u8]>>::as_ref(&self)[0..20]);
        AccountId20::from(H160::from_slice(&bytes))
    }
}

impl From<AccountId32Wrapper> for AccountId20 {
    fn from(wrapper: AccountId32Wrapper) -> Self {
        AccountId32::from(wrapper).into_account_id20()
    }
}

impl From<AccountId20> for AccountId32Wrapper {
    fn from(account_id20: AccountId20) -> Self {
        AccountId32Wrapper(AccountId32::from_account_id20(account_id20))
    }
}
