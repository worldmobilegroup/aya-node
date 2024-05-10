
use jsonrpc_core::{Error as RpcError, ErrorCode, Result as RpcResult};
use serde::{Deserialize, Serialize};
use aya_runtime::Block;
use sp_blockchain::HeaderBackend;

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
#[derive(Default)]
pub struct CardanoFollowerRpcImpl;

impl CardanoFollowerRpc for CardanoFollowerRpcImpl
{
 

    fn submit_cardano_event(&self, event: String) -> RpcResult<u64> {
        println!("Received event: {}", event);
        log::info!("submit_cardano_event called with data: {}", event);
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



impl CardanoFollowerRpcImpl {
    pub fn into_rpc(self) -> RpcModule<Self> {
        let mut module = RpcModule::new(self);
        module.register_async_method("submitCardanoEvent", |params, ctx| async move {
            let event_param: String = params.parse().unwrap_or_else(|_| "Default event data".to_string());
            log::info!("Received event via RPC: {}", event_param);
            // Assume further processing here
            Ok("Event processed successfully".to_string())
        }).unwrap();
        module
    }
}
