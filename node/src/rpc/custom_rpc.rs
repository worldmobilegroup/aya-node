use jsonrpsee::server::{RpcServiceBuilder, Server};
use jsonrpsee::RpcModule;
use jsonrpsee_types::ErrorObjectOwned;
use priority_queue::PriorityQueue;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
struct Event {
    data: String,
    timestamp: u64,
    block_height: u64,
}

pub async fn run_server() -> anyhow::Result<()> {
   

    log::info!("Server setup complete, starting now...");

    let rpc_middleware = RpcServiceBuilder::new().rpc_logger(1024);
    let server = Server::builder()
        .set_rpc_middleware(rpc_middleware)
        .build("0.0.0.0:5555")
        .await?;

    log::info!("Server is running on {}", server.local_addr()?);

    let pq = Arc::new(Mutex::new(PriorityQueue::new()));
    let mut module = RpcModule::new(pq.clone());
    initialize_rpc_methods(&mut module)?;

    let server_handle = server.start(module);
    log::info!("RPC Server started successfully.");

    // If `stopped()` is available:
    tokio::select! {
        _ = server_handle.clone().stopped() => {
            log::info!("Server has stopped normally.");
        },
        _ = tokio::signal::ctrl_c() => {
            log::info!("Received Ctrl+C, initiating shutdown.");
            server_handle.stop(); // Assuming `stop()` is a method to stop the server.
        }
    }

    Ok(())
}


fn initialize_rpc_methods(
    module: &mut RpcModule<Arc<Mutex<PriorityQueue<Event, i32>>>>,
) -> Result<(), anyhow::Error> {
    module.register_method("say_hello", |_, _| "lo")?;

    module.register_async_method("submit_event", |params, pq| async move {
        let (event, priority) = params.parse::<(Event, i32)>()?;
        let mut pq = pq.lock().await;
        let response = pq.push(event, priority);
        tracing::info!("response: {:?}", "Event submitted successfully");
        Ok::<_, ErrorObjectOwned>("Event submitted successfully".to_string())
    })?;
    module.register_async_method("pop_highest_priority_event", |params, pq| async move {
        let mut pq = pq.lock().await;
        if let Some((event, _priority)) = pq.pop() {
            tracing::info!("response: {:?}", "Event popped successfully");
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": true,
                    "event": event
                }))
                .unwrap(),
            ) // Ensure the JSON serialization doesn't fail; handle errors as needed
        } else {
            Ok(serde_json::to_string(&json!({
                "success": false,
                "message": "No events to pop"
            }))
            .unwrap()) // Similarly, ensure this doesn't fail
        }
        
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
                .unwrap(), // Serialize the response into JSON string
            )
        } else {
            Ok(
                serde_json::to_string(&json!({
                    "success": false,
                    "message": "No events found"
                }))
                .unwrap(), // Handle the case when no events are found
            )
        }
    })?;

    
    module.register_async_method("clear_all_events", |_, pq| async move {
        let mut pq = pq.lock().await;
        pq.clear();
    
        tracing::info!("response: {:?}", "Events cleared successfully");
        Ok::<_, ErrorObjectOwned>("All events cleared".to_string())
    })?;

    module.register_async_method("get_event_count", |_, pq| async move {
        let mut pq = pq.lock().await;
        Ok::<_, ErrorObjectOwned>(pq.len())
    })?;
    
    
    // module.register_method("get_event_priority", |params, _| {
    //     let event = params.parse::<Event>()?;
    //     let mut pq = pq.lock().await;
    //     if let Some(priority) = pq.get_priority(&event) {
    //         Ok::<_, ErrorObjectOwned>(priority)
    //     } else {
    //         Err(ErrorObjectOwned::custom(0, "Event not found".to_string()))
    //     }
    // })?;
    // module.register_method("set_event_priority", |params, _| {
    //     let (event, priority) = params.parse::<(Event, i32)>()?;
    //     let mut pq = pq.lock().await;
    //     if pq.set_priority(&event, priority) {
    //         Ok::<_, ErrorObjectOwned>("Priority updated successfully".to_string())
    //     } else {
    //         Err(ErrorObjectOwned::custom(0, "Event not found".to_string()))
    //     }
    // })?;
    // module.register_method("remove_event", |params, _| {
    //     let event = params.parse::<Event>()?;
    //     let mut pq = pq.lock().await;
    //     if pq.remove(&event) {
    //         Ok::<_, ErrorObjectOwned>("Event removed successfully".to_string())
    //     } else {
    //         Err(ErrorObjectOwned::custom(0, "Event not found".to_string()))
    //     }
    // })?;
    // module.register_method("remove_event_by_priority", |params, _| {
    //     let priority = params.parse::<i32>()?;
    //     let mut pq = pq.lock().await;
    //     if pq.remove_by_priority(&priority) {
    //         Ok::<_, ErrorObjectOwned>("Events removed successfully".to_string())
    //     } else {
    //         Err(ErrorObjectOwned::custom(0, "No events found".to_string()))
    //     }
    // })?;
    // module.register_method("remove_all_events_by_priority", |params, _| {
    //     let priority = params.parse::<i32>()?;
    //     let mut pq = pq.lock().await;
    //     if pq.remove_all_by_priority(&priority) {
    //         Ok::<_, ErrorObjectOwned>("Events removed successfully".to_string())
    //     } else {
    //         Err(ErrorObjectOwned::custom(0, "No events found".to_string()))
    //     }
    // })?;
    // module.register_async_method("get_highest_priority_event", |_, _| async move {
    //     let mut pq = pq.lock().await;
    //     if let Some((event, _priority)) = pq.peek() {
    //         Ok::<_, ErrorObjectOwned>(json!({
    //             "success": true,
    //             "event": event.clone()
    //         }))
    //     } else {
    //         Ok(json!({
    //             "success": false,
    //             "message": "No events found"
    //         }))
    //     }
    // })?;

    Ok(())
}
