use jsonrpsee::server::Server;
use jsonrpsee::RpcModule;
use jsonrpsee_types::ErrorObjectOwned;
use priority_queue::PriorityQueue;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;
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
    let pq = Arc::new(Mutex::new(PriorityQueue::<Event, i32>::new()));
    let event_ids = Arc::new(Mutex::new(HashSet::<u64>::new()));
    let context = Arc::new((pq, event_ids));

    let server = Server::builder().build("127.0.0.1:5555").await?;
    let addr = server.local_addr()?;
    info!("RPC server running on {}", addr);

    let mut module = RpcModule::new(context);

    module.register_async_method("submit_event", |params, ctx| async move {
        let (pq, event_ids) = &**ctx;
        let (event, priority) = params.parse::<(Event, i32)>()?;

        let mut event_ids = event_ids.lock().await;
        if event_ids.contains(&event.id) {
            error!("Event with id {} already exists", event.id);
            return Ok::<_, ErrorObjectOwned>("Event with the same id already exists".to_string());
        }

        let mut pq = pq.lock().await;
        event_ids.insert(event.id);
        println!("Received event: {:?} with priority: {}", event, priority);
        pq.push(event, priority);
        Ok::<_, ErrorObjectOwned>("Event submitted successfully".to_string())
    })?;

    module.register_async_method("list_all_events", |_, ctx| async move {
        let (pq, _) = &**ctx;
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

    module.register_async_method("clear_all_events", |_, ctx| async move {
        let (pq, event_ids) = &**ctx;
        let mut pq = pq.lock().await;
        let mut event_ids = event_ids.lock().await;
        pq.clear();
        event_ids.clear();
        tracing::info!("response: {:?}", "Events cleared successfully");
        Ok::<_, ErrorObjectOwned>("All events cleared".to_string())
    })?;

    module.register_async_method("pop", |_, ctx| async move {
        let (pq, event_ids) = &**ctx;
        let mut pq = pq.lock().await;
        let mut event_ids = event_ids.lock().await;
        if let Some((event, _priority)) = pq.pop() {
            event_ids.remove(&event.id);
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

    module.register_async_method("get_event_count", |_, ctx| async move {
        let (pq, _) = &**ctx;
        let pq = pq.lock().await;
        let response = json!({
            "jsonrpc": "2.0",
            "result": pq.len(),
            "id": 1
        })
        .to_string();
        Ok::<_, ErrorObjectOwned>(response)
    })?;

    module.register_async_method("get_event_by_id", |params, ctx| async move {
        let (pq, _) = &**ctx;
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

    module.register_async_method("update_event_priority", |params, ctx| async move {
        let (pq, _) = &**ctx;
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

    module.register_async_method("get_events_by_timestamp", |params, ctx| async move {
        let (pq, _) = &**ctx;
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

    module.register_async_method("request_event", |params, ctx| async move {
        let (pq, _) = &**ctx;
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

    module.register_async_method("remove_event", |params, ctx| async move {
        let (pq, event_ids) = &**ctx;
        let event_id: u64 = params.one()?;
        let mut pq = pq.lock().await;
        let mut event_ids = event_ids.lock().await;
        let mut found = false;

        let mut temp_queue = PriorityQueue::new();
        while let Some((event, priority)) = pq.pop() {
            if event.id == event_id {
                found = true;
                break;
            } else {
                temp_queue.push(event, priority);
            }
        }

        // Restore the remaining events
        while let Some((event, priority)) = temp_queue.pop() {
            pq.push(event, priority);
        }

        if found {
            event_ids.remove(&event_id);
            Ok::<_, ErrorObjectOwned>("Event removed successfully".to_string())
        } else {
            Ok::<_, ErrorObjectOwned>("Event not found".to_string())
        }
    })?;

    let handle = server.start(module);

    // Block until the server is stopped
    handle.stopped().await;
    println!("Server has been stopped externally.");

    Ok(addr)
}
