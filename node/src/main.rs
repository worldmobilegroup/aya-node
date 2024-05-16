
//! Substrate Node Template CLI library.

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

use log::{ info, error, warn, debug, trace };
use env_logger;
use tokio;


use tokio::signal;
use tracing_subscriber;

use anyhow::{self, Result};


mod rpc;  
use crate::rpc::priority_queue_rpc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
   
    // Start the RPC server using a background task
    let server_handle = tokio::spawn(async {
        match priority_queue_rpc::run_server().await {
            Ok(_) => info!("RPC server stopped."),
            Err(e) => error!("RPC server failed: {:?}", e),
        }
    });

    // Start the command logic using another background task for blocking operations
    let node_handle = tokio::task::spawn_blocking(|| {
        match command::run() {
            Ok(_) => info!("Node command executed successfully."),
            Err(e) => error!("Node command failed: {:?}", e),
        }
    });

    // Optional: Wait for both tasks to complete or handle control signals
    tokio::select! {
        _ = server_handle => info!("Server task completed."),
        _ = node_handle => info!("Node task completed."),
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl-C, shutting down.");
        }
    }

    info!("System shutdown complete.");
    Ok(())
}

