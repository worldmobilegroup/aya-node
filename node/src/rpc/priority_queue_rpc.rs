use jsonrpsee::server::Server;
use jsonrpsee::RpcModule;
use jsonrpsee_types::ErrorObjectOwned;
use priority_queue::PriorityQueue;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Clone)]
struct Event {
    id: u64,
    data: String,
    timestamp: u64,
    block_height: u64,
}

impl Event {
    pub fn is_valid(&self) -> bool {
        self.timestamp != 0 && self.block_height != 0
    }

    pub fn hash_without_timestamp(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.id.hash(&mut hasher);
        self.data.hash(&mut hasher);
        self.block_height.hash(&mut hasher);
        hasher.finish()
    }
}

struct EventQueue {
    queue: PriorityQueue<Event, i32>,
    event_hashes: HashSet<u64>,
    processed_events: HashSet<u64>,
}

impl EventQueue {
    fn new() -> Self {
        EventQueue {
            queue: PriorityQueue::new(),
            event_hashes: HashSet::new(),
            processed_events: HashSet::new(),
        }
    }

    fn is_duplicate(&self, event: &Event) -> bool {
        let hash = event.hash_without_timestamp();
        let is_duplicate =
            self.event_hashes.contains(&hash) || self.processed_events.contains(&event.id);

        tracing::debug!("Checking if event is duplicate:");
        tracing::debug!("Event ID: {}, Hash without timestamp: {}", event.id, hash);
        tracing::debug!(
            "Event Hashes Contains: {}",
            self.event_hashes.contains(&hash)
        );
        tracing::debug!(
            "Processed Events Contains: {}",
            self.processed_events.contains(&event.id)
        );
        tracing::debug!("Is Duplicate: {}", is_duplicate);

        is_duplicate
    }

    fn mark_as_processed(&mut self, event_id: u64) {
        self.processed_events.insert(event_id);
    }

    fn push(&mut self, event: Event, priority: i32) -> Result<(), String> {
        if self.is_duplicate(&event) {
            return Err(format!("Duplicate event: {:?}", event));
        }
        self.queue.push(event.clone(), priority);
        self.event_hashes.insert(event.hash_without_timestamp());
        Ok(())
    }

    fn iter(&self) -> impl Iterator<Item = (&Event, &i32)> {
        self.queue.iter()
    }
}

pub async fn run_server() -> anyhow::Result<SocketAddr> {
    let server = Server::builder().build("127.0.0.1:5555").await?;
    let addr = server.local_addr()?;
    info!("RPC server running on {}", addr);

    let event_queue = Arc::new(Mutex::new(EventQueue::new()));
    let mut module = RpcModule::new(event_queue.clone());

    module.register_async_method("list_all_events", |_, event_queue| async move {
        let mut queue = event_queue.lock().await;
        let mut events = Vec::new();
        let mut duplicates = Vec::new();
        let mut to_mark_processed = Vec::new();

        // Collect all events and identify duplicates
        for (event, priority) in queue.iter() {
            if queue.processed_events.contains(&event.id) {
                if queue.is_duplicate(&event) {
                    tracing::error!("Duplicate event detected: {:?}", event);
                    return Err(ErrorObjectOwned::owned(
                        -32000, // Custom error code
                        format!("Duplicate event: {:?}", event),
                        None::<()>,
                    ));
                }
                duplicates.push((event.clone(), *priority));
            } else {
                events.push((event.clone(), *priority));
                to_mark_processed.push(event.id);
            }
        }

        // Mark new events as processed
        for event_id in to_mark_processed {
            queue.mark_as_processed(event_id);
        }

        if !events.is_empty() || !duplicates.is_empty() {
            tracing::info!(
                "Events listed successfully. New events: {}, Duplicates: {}",
                events.len(),
                duplicates.len()
            );
            Ok::<_, ErrorObjectOwned>(
                serde_json::to_string(&json!({
                    "success": true,
                    "events": events,
                    "duplicates": duplicates
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

    module.register_async_method("submit_event", |params, event_queue| async move {
        let (event, priority) = params.parse::<(Event, i32)>()?;
        let mut queue = event_queue.lock().await;

        match queue.push(event.clone(), priority) {
            Ok(_) => {
                tracing::info!("Received event: {:?} with priority: {}", event, priority);
                Ok::<_, ErrorObjectOwned>("Event submitted successfully".to_string())
            }
            Err(e) => {
                tracing::error!("Failed to submit event: {}", e);
                Ok::<_, ErrorObjectOwned>(e)
            }
        }
    })?;

    let handle = server.start(module);

    // Block until the server is stopped
    handle.stopped().await;
    println!("Server has been stopped externally.");

    Ok(addr)
}
