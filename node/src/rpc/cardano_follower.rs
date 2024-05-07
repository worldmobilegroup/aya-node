use jsonrpc_derive::rpc;
use jsonrpsee::RpcModule;
use jsonrpc_core::{Error as RpcError, Result as RpcResult, ErrorCode};
use serde::{Deserialize, Serialize}; // Make sure serde's derive macros are available
use std::sync::Arc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend; // Provides the `info()` method
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use jsonrpc_core_client::RpcChannel;
// Assuming you are working with a specific block type from your runtime
use aya_runtime::{opaque::Block, RuntimeApi};

// Make sure all necessary crates are correctly imported

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
        println!("Received event: {}", event);

        // Parse JSON and extract necessary info
        let parsed_event: Event = serde_json::from_str(&event).map_err(|e| RpcError {
            message: "Failed to parse event".into(),
            code: ErrorCode::ServerError(101),
            data: None,
        })?;

        // Submitting a transaction to the runtime
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);
        let result = api.submit_event(&at, parsed_event).map_err(|e| RpcError {
            message: "Runtime API failed".into(),
            code: ErrorCode::ServerError(102),
            data: Some(format!("{:?}", e).into()),
        })?;

        Ok(result)
    }
}

impl<Client> CardanoFollowerRpcImpl<Client>
where
    // Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    // Client::Api: RuntimeApiCollection,
{
    pub fn into_rpc(self) -> RpcModule<(Self)> {
        let mut module = RpcModule::new(self);
        // module.register_async_method("submitCardanoEvent", |params, this| {
        //     async move {
        //         let event: String = params.parse()?;
        //         this.submit_cardano_event(event)
        //     }
        // }).unwrap();
        module
    }
}

#[derive(Deserialize, Debug)]
pub struct Event {
    // Example fields
    pub data: String,
}

/// Runtime API collection that includes all APIs needed for this RPC
pub trait RuntimeApiCollection: sp_api::ApiExt<Block> {
    fn submit_event(&self, at: &BlockId<Block>, event: Event) -> Result<u64, sp_api::ApiError>;
}
