use jsonrpc_derive::rpc;
use jsonrpc_core::{Error as RpcError, Result as RpcResult};
use std::sync::Arc;
use sc_service::Client;
use sp_runtime::generic::BlockId;
use sp_runtime::traits::Block as BlockT;
use substrate_frame_rpc_system::helpers::{self, Call};

// Assuming you are working with a specific block type from your runtime
use aya_runtime::{opaque::Block, RuntimeApi};

/// RPC interface for receiving Cardano follower notifications.
#[rpc]
pub trait CardanoFollowerRpc {
    #[rpc(name = "submitCardanoEvent")]
    fn submit_cardano_event(&self, event: String) -> RpcResult<u64>;
}

/// Implementation of the CardanoFollowerRpc trait.
pub struct CardanoFollowerRpcImpl<Client> {
    /// Dependency on Substrate client.
    client: Arc<Client>,
}

impl<Client> CardanoFollowerRpc for CardanoFollowerRpcImpl<Client>
where
    Client: sp_api::ProvideRuntimeApi<Block> + Send + Sync + 'static,
    Client::Api: RuntimeApiCollection,
{
    fn submit_cardano_event(&self, event: String) -> RpcResult<u64> {
        println!("Received event: {}", event);

        // Here you might want to parse the event and act on it
        // Example: Parse JSON and extract necessary info
        let parsed_event = serde_json::from_str::<Event>(&event).map_err(|e| {
            RpcError {
                message: "Failed to parse event".into(),
                code: jsonrpc_core::ErrorCode::ServerError(101),
                data: None,
            }
        })?;

        // Example action: submitting a transaction to the runtime
        let api = self.client.runtime_api();
        let at = BlockId::hash(self.client.info().best_hash);
        let result = api.submit_event(&at, parsed_event).map_err(|e| RpcError {
            message: "Runtime API failed".into(),
            code: jsonrpc_core::ErrorCode::ServerError(102),
            data: Some(format!("{:?}", e).into()),
        })?;

        // Respond with some relevant information or confirmation
        Ok(result)
    }
}

/// Runtime API collection that includes all APIs needed for this RPC
pub trait RuntimeApiCollection: sp_api::ApiExt<Block> {
    // Define necessary API calls here, e.g.,
    fn submit_event(&self, at: &BlockId<Block>, event: Event) -> Result<u64, sp_api::ApiError>;
}

// Additional structs and helpers as needed
#[derive(serde::Deserialize, Debug)]
struct Event {
    // Define event structure
}
