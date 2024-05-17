use jsonrpsee::server::Server;
use jsonrpsee::RpcModule;
use jsonrpsee_types::ErrorObjectOwned;
use priority_queue::PriorityQueue;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
struct Event {
    data: String,
    timestamp: u64,
    block_height: u64,
}

pub async fn run_server() -> anyhow::Result<SocketAddr> {
    let server = Server::builder().build("127.0.0.1:5555").await?;
    let addr = server.local_addr()?;
    info!("RPC server running on {}", addr);

    let pq = Arc::new(Mutex::new(PriorityQueue::<Event, i32>::new()));
    let mut module = RpcModule::new(pq);

    module.register_async_method("submit_event", |params, pq| async move {
        let (event, priority) = params.parse::<(Event, i32)>()?;
        let mut pq = pq.lock().await;
        println!("Received event: {:?} with priority: {}", event, priority);
        let response = pq.push(event, priority);
        Ok::<_, ErrorObjectOwned>("Event submitted successfully".to_string())
    })?;
    module.register_async_method("list_all_events", |_, pq| async move {
        let mut pq = pq.lock().await;
        let events = pq.iter().map(|(e, p)| (e.clone(), *p)).collect::<Vec<_>>();
        if !events.is_empty() {
            tracing::info!("response: {:?}", "Events listed successfully");
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": true,
                    "events": events
                }))
                .unwrap(),
            )
        } else {
            Ok(serde_json::to_string(&json!({
                "success": false,
                "message": "No events found"
            }))
            .unwrap())
        }
    })?;

    let handle = server.start(module);

    // Block until the server is stopped
    handle.stopped().await;
    println!("Server has been stopped externally.");

    Ok(addr)
}

// pub async fn run_server() -> anyhow::Result<SocketAddr> {
//     let server = Server::builder().build("127.0.0.1:5555").await?;
//     let addr = server.local_addr()?;
//     info!("RPC server running on {}", addr);

//     // Explicitly specifying the item and priority types for PriorityQueue
//     let pq = Arc::new(Mutex::new(PriorityQueue::<Event, i32>::new()));

//     let mut module = RpcModule::new(pq); // Make sure that pq is the correct type expected here
//     module.register_async_method("submit_event", |params, pq| async move {
//         let (event, priority) = params.parse::<(Event, i32)>()?;
//         let mut pq = pq.lock().await;
//         println!("Received event: {:?} with priority: {}", event, priority);
//         let response = pq.push(event, priority);
//         Ok::<_, ErrorObjectOwned>("Event submitted successfully".to_string())
//     })?;

//     module.register_async_method("pop_highest_priority_event", |params, pq| async move {
//         let mut pq = pq.lock().await;
//         if let Some((event, _priority)) = pq.pop() {
//             tracing::info!("response: {:?}", "Event popped successfully");
//             Ok::<_, ErrorObjectOwned>(
//                 serde_json::to_string(&json!({
//                     "success": true,
//                     "event": event
//                 }))
//                 .unwrap(),
//             )
//         } else {
//             Ok(serde_json::to_string(&json!({
//                 "success": false,
//                 "message": "No events to pop"
//             }))
//             .unwrap())
//         }
//     })?;

//     module.register_async_method("list_all_events", |_, pq| async move {
//         let mut pq = pq.lock().await;
//         let events = pq.iter().map(|(e, p)| (e.clone(), *p)).collect::<Vec<_>>();
//         if !events.is_empty() {
//             tracing::info!("response: {:?}", "Events listed successfully");
//             Ok::<_, ErrorObjectOwned>(
//                 serde_json::to_string(&json!({
//                     "success": true,
//                     "events": events
//                 }))
//                 .unwrap(),
//             )
//         } else {
//             Ok(
//                 serde_json::to_string(&json!({
//                     "success": false,
//                     "message": "No events found"
//                 }))
//                 .unwrap(),
//             )
//         }
//     })?;

//     module.register_async_method("clear_all_events", |_, pq| async move {
//         let mut pq = pq.lock().await;
//         pq.clear();

//         tracing::info!("response: {:?}", "Events cleared successfully");
//         Ok::<_, ErrorObjectOwned>("All events cleared".to_string())
//     })?;

//     module.register_async_method("get_event_count", |_, pq| async move {
//         let mut pq = pq.lock().await;
//         Ok::<_, ErrorObjectOwned>(pq.len())
//     })?;

//     let addr = server.local_addr()?;
//     let handle = server.start(module);

//     tokio::spawn(async move {
//         handle.stopped().await;
//         println!("Server has been stopped externally.");
//     });

//     Ok(addr)
// }
