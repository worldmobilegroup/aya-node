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
    id: u64, // Add an ID field to the Event struct
    data: String,
    timestamp: u64,
    block_height: u64,
}

#[derive(Serialize, Deserialize)]
struct UpdatePriorityParams {
    id: u64,
    new_priority: i32,
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

    module.register_async_method("clear_all_events", |_, pq| async move {
        let mut pq = pq.lock().await;
        pq.clear();

        tracing::info!("response: {:?}", "Events cleared successfully");
        Ok::<_, ErrorObjectOwned>("All events cleared".to_string())
    })?;

    module.register_async_method("pop", |params, pq| async move {
        let mut pq = pq.lock().await;
        if let Some((event, _priority)) = pq.pop() {
            tracing::info!("response: {:?}", "Event popped successfully");
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": true,
                    "event": event
                }))
                .unwrap(),
            )
        } else {
            Ok(serde_json::to_string(&json!({
                "success": false,
                "message": "No events to pop"
            }))
            .unwrap())
        }
    })?;

    module.register_async_method("get_event_count", |_, pq| async move {
        let pq = pq.lock().await;
        let response = json!({
            "jsonrpc": "2.0",
            "result": pq.len(),
            "id": 1
        })
        .to_string();
        Ok::<_, ErrorObjectOwned>(response)
    })?;

    module.register_async_method("get_event_by_id", |params, pq| async move {
        let id: u64 = params.one()?;
        let pq = pq.lock().await;
        let event = pq.iter().find(|(e, _)| e.id == id).map(|(e, _)| e.clone());
        if let Some(event) = event {
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": true,
                    "event": event
                }))
                .unwrap(),
            )
        } else {
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": false,
                    "message": "Event not found"
                }))
                .unwrap(),
            )
        }
    })?;
    //TOFIX
    module.register_async_method("update_event_priority", |params, pq| async move {
        let UpdatePriorityParams { id, new_priority } = params.parse()?;
        let mut pq = pq.lock().await;
        let mut event_opt = None;
        let mut events = Vec::new();
        while let Some((e, p)) = pq.pop() {
            if e.id == id {
                event_opt = Some((e, new_priority));
            } else {
                events.push((e, p));
            }
        }
        for (e, p) in events {
            pq.push(e, p);
        }
        if let Some((event, priority)) = event_opt {
            pq.push(event, priority);
            Ok::<_, ErrorObjectOwned>("Event priority updated successfully".to_string())
        } else {
            Ok::<_, ErrorObjectOwned>("Event not found".to_string())
        }
    })?;

    module.register_async_method("get_events_by_timestamp", |params, pq| async move {
        let (start, end): (u64, u64) = params.parse()?;
        let pq = pq.lock().await;
        let events = pq
            .iter()
            .filter(|(e, _)| e.timestamp >= start && e.timestamp <= end)
            .map(|(e, p)| (e.clone(), *p))
            .collect::<Vec<_>>();
        if !events.is_empty() {
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": true,
                    "events": events
                }))
                .unwrap(),
            )
        } else {
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": false,
                    "message": "No events found in the given timestamp range"
                }))
                .unwrap(),
            )
        }
    })?;
    module.register_async_method("request_event", |params, pq| async move {
        let event_id: u64 = params.one()?;
        let pq = pq.lock().await;
        let event = pq
            .iter()
            .find(|(e, _)| e.id == event_id)
            .map(|(e, _)| e.clone());
        if let Some(event) = event {
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": true,
                    "event": event
                }))
                .unwrap(),
            )
        } else {
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": false,
                    "message": "Event not found"
                }))
                .unwrap(),
            )
        }
    })?;

    let handle = server.start(module);

    // Block until the server is stopped
    handle.stopped().await;
    println!("Server has been stopped externally.");

    Ok(addr)
}
