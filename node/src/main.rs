#![warn(missing_docs)]
#![allow(
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant
)]
#![cfg_attr(feature = "runtime-benchmarks", warn(unused_crate_dependencies))]

mod chain_spec;
mod cli;
mod client;
mod command;
mod eth;
mod service;

use anyhow::Result;
use tokio::signal;
use tracing::{error, info};
use tracing_subscriber;

mod rpc;
use crate::rpc::priority_queue_rpc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    // tracing_subscriber::fmt::init();

    // Start the RPC server using a background task
    let server_handle = tokio::spawn(async {
        if let Err(e) = priority_queue_rpc::run_server().await {
            error!("RPC server failed: {:?}", e);
        }
    });

    // Start the command logic using another background task for blocking operations
    let node_handle = tokio::task::spawn_blocking(|| {
        if let Err(e) = command::run() {
            error!("Node command failed: {:?}", e);
        } else {
            info!("Node command executed successfully.");
        }
    });

    // Wait for Ctrl-C signal to shut down
    let ctrl_c_handle = tokio::spawn(async {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl-C");
        info!("Received Ctrl-C, shutting down.");
    });

    // Use `tokio::select!` to wait for any task to complete or handle the Ctrl-C signal
    tokio::select! {
        _ = server_handle => {
            info!("Server task completed.");
        },
        _ = node_handle => {
            info!("Node task completed.");
        },
        _ = ctrl_c_handle => {
            info!("Received Ctrl-C, shutting down.");
        },
    }

    info!("System shutdown complete.");
    Ok(())
}
