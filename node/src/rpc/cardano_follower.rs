use aya_runtime::{opaque::Block, RuntimeApi};
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use sp_io;
use jsonrpc_derive::rpc;
use jsonrpsee_core::server::RpcModule;

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

        // Deserialize the JSON string into an Event struct
        let parsed_event: Result<Event, _> = serde_json::from_str(&event);
        match parsed_event {
            Ok(event) => {
                println!("Parsed event successfully: {:?}", event);

                // Serialize the Event struct to store it
                let data = serde_json::to_vec(&event).expect("Serialization should work");
                let key = b"cardano_events"; // Define a key for your storage

                // Store the event data in offchain storage
                sp_io::offchain::local_storage_set(sp_runtime::offchain::StorageKind::PERSISTENT, key, &data);

                println!("Event stored successfully in offchain storage.");
                Ok(0) // Return some identifier or success code
            },
            Err(e) => {
                println!("Error parsing event: {}", e);
                Err(RpcError {
                    message: "Failed to parse event".into(),
                    code: ErrorCode::ServerError(101),
                    data: None,
                })
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub data: String,
}

/// Runtime API collection that includes all APIs needed for this RPC
pub trait RuntimeApiCollection: sp_api::ApiExt<Block> {
    fn submit_event(&self, at: &BlockId<Block>, event: Event) -> Result<u64, sp_api::ApiError>;
}

impl<Client> CardanoFollowerRpcImpl<Client>
where
    Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
    Client::Api: RuntimeApiCollection,
{
    pub fn into_rpc(self) -> RpcModule<Self> {
        let mut module = RpcModule::new(self);
        module.register_async_method("submitCardanoEvent", |_params, ctx| async move {
            // You can insert more complex logic here, now that the redundancy is resolved.
            "Hello from async method".to_string()
        }).unwrap();
        module
    }
}
