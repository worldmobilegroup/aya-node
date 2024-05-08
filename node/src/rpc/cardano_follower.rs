use aya_runtime::{opaque::Block, RuntimeApi};
use jsonrpc_core::ErrorCode;
use jsonrpc_core::{Error as RpcError, ErrorCode as JsonRpcCoreErrorCode, Result as RpcResult};
use jsonrpc_core_client::RpcChannel;
use jsonrpc_derive::rpc;
use jsonrpsee::types::error::ErrorCode as JsonRpcSeeErrorCode;
use jsonrpsee::RpcModule;
use serde::{Deserialize, Serialize}; // Make sure serde's derive macros are available
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend; // Provides the `info()` method
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

/// RPC interface for receiving Cardano follower notifications.
#[rpc]
pub trait CardanoFollowerRpc {
    /// Define RPC method for submitting Cardano events.
    #[rpc(name = "submitCardanoEvent")]
    fn submit_cardano_event(&self, event: String) -> RpcResult<u64>;
}

/// Implementation of the CardanoFollowerRpc trait.
pub struct CardanoFollowerRpcImpl<Client> {
    /// Public dependency on Substrate client.
    pub client: Arc<Client>,
}
impl<Client> CardanoFollowerRpc for CardanoFollowerRpcImpl<Client>
where
    Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    Client::Api: RuntimeApiCollection,
{
    fn submit_cardano_event(&self, event: String) -> RpcResult<u64> {
        // Log the incoming event for debugging purposes
        println!("Received event: {}", event);

        // Attempt to deserialize the JSON string into an Event struct
        let parsed_event: Event = serde_json::from_str(&event).map_err(|e| {
            // Log parsing errors
            println!("Error parsing event: {}", e);
            RpcError {
                message: "Failed to parse event".into(),
                code: ErrorCode::ServerError(101),
                data: None,
            }
        })?;

        // Submitting a transaction to the runtime with the parsed event
        //TODO: Is this really what we ant to do here?
        //Probably will save to offchain storage
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);
        let result = api.submit_event(&at, parsed_event).map_err(|e| {
            // Log errors from the runtime API
            println!("Runtime API failed: {}", e);
            RpcError {
                message: "Runtime API failed".into(),
                code: ErrorCode::ServerError(102),
                data: Some(format!("{:?}", e).into()),
            }
        })?;

        Ok(result)
    }
}

impl<Client> CardanoFollowerRpcImpl<Client>
where
    Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    Client::Api: RuntimeApiCollection,
{
    pub fn into_rpc(self) -> RpcModule<Self> {
        let mut module = RpcModule::new(self);
        module
            .register_async_method("submitCardanoEvent", |params, this| {
                async move {
                    let event: String = params
                        .parse()
                        .map_err(|e| RpcError {
                            code: ErrorCode::ParseError,
                            message: "Invalid params".into(),
                            data: Some(format!("{:?}", e).into()),
                        })
                        .expect("Parameter parsing should not fail");
                    //Todo: proper error propagation
                    // this.submit_cardano_event(event).map_err(|e| RpcError {
                    //     code: ErrorCode::ServerError(JsonRpcCoreErrorCode::InternalError.into()),
                    //     message: e.message,
                    //     data: e.data,
                    // })
                }
            })
            .expect("Method registration should not fail"); // Changed to expect with a message for clarity
        module
    }
}

#[derive(Deserialize, Debug)]
pub struct Event {
    pub data: String,
}

/// Runtime API collection that includes all APIs needed for this RPC
pub trait RuntimeApiCollection: sp_api::ApiExt<Block> {
    fn submit_event(&self, at: &BlockId<Block>, event: Event) -> Result<u64, sp_api::ApiError>;
}
