#![cfg_attr(not(feature = "std"), no_std)]

use sp_api::decl_runtime_apis;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
// Here we declare the runtime API. It is implemented it the `impl` block in
// runtime file (the `runtime-api/src/lib.rs`)
sp_api::decl_runtime_apis! {
    pub trait ChainListenerApi {
        fn get_value() -> u32;
    }
}
