//! A collection of node-specific RPC methods.

use std::sync::Arc;

use futures::channel::mpsc;
use jsonrpsee::RpcModule;
// Substrate
use sc_client_api::{
    backend::{Backend, StorageProvider},
    client::BlockchainEvents,
    AuxStore, UsageProvider,
};
use sc_consensus_manual_seal::rpc::EngineCommand;
use sc_rpc::SubscriptionTaskExecutor;
use sc_rpc_api::DenyUnsafe;
use sc_service::TransactionPool;
use sc_transaction_pool::ChainApi;
use sp_api::{CallApiAt, ProvideRuntimeApi};
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_inherents::CreateInherentDataProviders;
use sp_runtime::traits::Block as BlockT;

use aya_runtime::{opaque::Block, AccountId, Balance, Hash, Nonce};

mod eth;
pub use self::eth::{create_eth, overrides_handle, EthDeps};

/// Full client dependencies.
pub struct FullDeps<C, P, A: ChainApi, CT, CIDP> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// Manual seal command sink
    pub command_sink: Option<mpsc::Sender<EngineCommand<Hash>>>,
    /// Ethereum-compatibility specific dependencies.
    pub eth: EthDeps<Block, C, P, A, CT, CIDP>,
}

pub struct DefaultEthConfig<C, BE>(std::marker::PhantomData<(C, BE)>);

impl<C, BE> fc_rpc::EthConfig<Block, C> for DefaultEthConfig<C, BE>
where
    C: StorageProvider<Block, BE> + Sync + Send + 'static,
    BE: Backend<Block> + 'static,
{
    type EstimateGasAdapter = ();
    type RuntimeStorageOverride =
        fc_rpc::frontier_backend_client::SystemAccountId20StorageOverride<Block, C, BE>;
}

use log::{debug, error, info, warn}; // Adjust according to what you need

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, BE, A, CT, CIDP>(
    deps: FullDeps<C, P, A, CT, CIDP>,
    subscription_task_executor: SubscriptionTaskExecutor,
    pubsub_notification_sinks: Arc<
        fc_mapping_sync::EthereumBlockNotificationSinks<
            fc_mapping_sync::EthereumBlockNotification<Block>,
        >,
    >,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>> {
    info!("Initializing RPC modules");
    pub mod cardano_follower;
    pub use self::cardano_follower::{CardanoFollowerRpc, CardanoFollowerRpcImpl};
    use chain_listener::rpc::ChainListenerRpc;

    let mut io = RpcModule::new(());

    debug!("Loading dependencies");
    let FullDeps {
        client,
        pool,
        deny_unsafe,
        command_sink,
        eth,
    } = deps;

    info!("Merging system and transaction payment RPCs");
    io.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
    io.merge(TransactionPayment::new(client.clone()).into_rpc())?;
    io.merge(ChainListener::new(client).into_rpc())?;
    if let Some(command_sink) = command_sink {
        info!("Merging manual seal");
        io.merge(ManualSeal::new(command_sink).into_rpc())?;
    }

    info!("Setting up Ethereum compatibility");
    let eth_io = create_eth::<_, _, _, _, _, _, _, DefaultEthConfig<C, BE>>(
        RpcModule::new(()),
        eth,
        subscription_task_executor,
        pubsub_notification_sinks,
    )?;
    io.merge(eth_io)?;

    info!("Registering Cardano follower RPCs");
    let cardano_follower = CardanoFollowerRpcImpl::new(deps.client.clone()); // Adjust based on actual constructor
    io.merge(cardano_follower.into_rpc())?;

    info!("RPC modules initialized successfully");
    Ok(io)
}
