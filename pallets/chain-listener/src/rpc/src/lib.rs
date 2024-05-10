pub use chain_listener_runtime_api::ChainListenerApi as ChainListenerRuntimeApi;
use jsonrpsee::types::error::{ErrorCode, ErrorObjectOwned};
use jsonrpsee::{core::RpcResult, proc_macros::rpc};

use jsonrpsee_types::error::ErrorObject;

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc(client, server)]
pub trait ChainListenerApi<BlockHash> {
    #[method(name = "chain_listener_getValue")]
    fn get_value(&self, at: Option<BlockHash>) -> RpcResult<u32>;
}


pub struct ChainListener<C, Block> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<Block>,
}

impl<C, Block> ChainListener<C, Block> {
  
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}
const RUNTIME_ERROR: i32 = 1;

impl<C, Block> ChainListenerApiServer<<Block as BlockT>::Hash> for ChainListener<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: ChainListenerRuntimeApi<Block>,
{
	fn get_value(&self, at: Option<<Block as BlockT>::Hash>) -> RpcResult<u32> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||self.client.info().best_hash));

		api.get_value(&at).map_err(runtime_error_into_rpc_err)
	}
}



/// Converts a runtime trap into an RPC error suitable for JSON RPC.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> jsonrpsee::types::error::ErrorObjectOwned {
    jsonrpsee::types::error::ErrorObjectOwned::owned(
        RUNTIME_ERROR,  // Example error code
        "Runtime error",
        Some(format!("{:?}", err))
    )
}

#[derive(Debug)]
pub enum RpcError {
    RuntimeError,
    // Other error variants can be added here
}

impl From<RpcError> for ErrorObjectOwned {
    fn from(error: RpcError) -> Self {
        match error {
            RpcError::RuntimeError => {
                ErrorObjectOwned::owned(1, "Runtime error", Some(format!("{:?}", error)))
            } // Handle other errors accordingly
        }
    }
}
