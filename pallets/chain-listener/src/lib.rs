#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
#[cfg_attr(feature = "std", macro_use)]
extern crate serde;
extern crate sp_std;
use alloc::string::ToString;
pub use pallet::*;
// #[cfg(feature = "std")]
// use serde::{Deserialize, Serialize};
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
// pub use weights::*;
use sp_application_crypto::AppCrypto;
use sp_core::H256;
use sp_runtime::codec::{Encode, Decode};
use alloc::vec::Vec;
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, storage::types::StorageMap};
use frame_support::{weights::Weight};
use frame_system::{offchain::*, pallet_prelude::*};

use scale_info::prelude::format;

use serde_json;
use sp_consensus_aura::ed25519::AuthorityId;
use sp_core::Public;
use sp_runtime::offchain::*;


use sp_core::offchain::Duration;
use sp_runtime::offchain::http::Request;
use sp_runtime::{
    offchain as rt_offchain,
    offchain::{
        storage::StorageValueRef,
        storage_lock::{BlockAndTime, StorageLock},
    },
};
use sp_std::prelude::*;
use trie_db::{Trie, TrieDB, TrieDBMut, TrieLayout};
use sp_runtime::{ Serialize, Deserialize};
use scale_info::TypeInfo;
use frame_support::unsigned::TransactionSource;






#[derive(Default, Deserialize, Serialize, Debug, Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct CustomEvent {
    pub id: u64,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub block_height: u64,
    pub previous_hash: Option<H256>,
}

impl CustomEvent {
    fn new(id: u64, data: Vec<u8>, timestamp: u64, block_height: u64, previous_hash: Option<H256>) -> Self {
        CustomEvent {
            id,
            data,
            timestamp,
            block_height,
            previous_hash,
        }
    }

    fn calculate_hash(&self) -> H256 {
        let encoded = self.encode();
        sp_io::hashing::blake2_256(&encoded).into()
    }
}


#[frame_support::pallet]
pub mod pallet {
    use super::*;
   
    
    
    

    #[pallet::config]
    pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
        type AuthorityId: Public;
        // type SubmitTransaction: frame_system::offchain::SendSignedTransaction<Self, AppCrypto, Call<Self>>;
        
        // Authority identifier for signing transactions
        // type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn event_storage)]
    pub type EventStorage<T: Config> = StorageMap<_, Blake2_128Concat, u64, CustomEvent, ValueQuery>;



    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            if let Err(e) = Self::fetch_and_process_data() {
                log::error!("Error fetching and sending data: {:?}", e);
            }

            if let Err(e) = Self::fetch_all_events() {
                log::error!("Error fetching all events: {:?}", e);
            }
             // Check if the validator is the leader
        if Self::is_leader() {
            // Create and submit an inclusion transaction
            if let Err(e) = Self::create_inclusion_transaction() {
                log::error!("Error creating inclusion transaction: {:?}", e);
            }
        }

        }
        
    }

    impl<T: Config> Pallet<T> {
        fn cleanup_processed_events() {
            // Remove events from storage that have been included in the blockchain
            for event_id in EventStorage::<T>::iter_keys() {
                EventStorage::<T>::remove(event_id);
            }
        }

        fn is_leader() -> bool {
            // Implement your leader election logic here
            // For simplicity, we assume the current validator is the leader
            true
        }

        fn fetch_all_events() -> Result<Vec<CustomEvent>, Error<T>> {
            const HTTP_REMOTE_REQUEST: &str = "http://127.0.0.1:5555";
            const HTTP_HEADER_USER_AGENT: &str = "SubstrateOffchainWorker";
            const HTTP_HEADER_CONTENT_TYPE: &str = "Content-Type";
            const CONTENT_TYPE_JSON: &str = "application/json";
            const FETCH_TIMEOUT_PERIOD: u64 = 3000; // in milliseconds
    
            // Create the JSON-RPC request payload
            let json_payload = serde_json::json!({
                "jsonrpc": "2.0",
                "method": "list_all_events",
                "params": [],
                "id": 1
            })
            .to_string()
            .into_bytes();
            let json_payload_ref: Vec<&[u8]> = vec![&json_payload];
    
            // Initiate an external HTTP POST request. This is using high-level wrappers from `sp_runtime`.
            let request = rt_offchain::http::Request::post(HTTP_REMOTE_REQUEST, json_payload_ref);
    
            // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
            let timeout = sp_io::offchain::timestamp()
                .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));
    
            // Set the request headers
            let pending = request
                .add_header("User-Agent", HTTP_HEADER_USER_AGENT)
                .add_header(HTTP_HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
                .deadline(timeout) // Setting the timeout time
                .send() // Sending the request out by the host
                .map_err(|_| <Error<T>>::HttpFetchingError)?;
    
            let response = pending
                .try_wait(timeout)
                .map_err(|_| <Error<T>>::HttpFetchingError)?
                .map_err(|_| <Error<T>>::HttpFetchingError)?;
    
            log::info!("Response code: {}", response.code);
    
            let body = response.body().collect::<Vec<u8>>();
            match String::from_utf8(body.clone()) {
                Ok(json_string) => {
                    log::info!("Response body: {}", json_string);
                }
                Err(e) => {
                    log::error!("Failed to parse response body as UTF-8: {:?}", e);
                    log::info!("Response body bytes: {:?}", body);
                }
            }
    
            if response.code != 200 {
                return Err(<Error<T>>::HttpFetchingError);
            }
    
            // Deserialize the response body to Vec<CustomEvent>
            let events: Vec<CustomEvent> = serde_json::from_slice(&body)
                .map_err(|_| <Error<T>>::HttpFetchingError)?;
    
            Ok(events)
        }

        fn validate_and_process_event(event: CustomEvent) -> Result<(), Error<T>> {
            // // Validate the event data
            if event.timestamp == 0 || event.block_height == 0 {
                return Err(Error::<T>::InvalidEventData);
            }

            // Additional validation logic...

            // Process the event (e.g., store in mempool)
            Self::store_event_in_mempool(event);

            Ok(())
        }

        fn store_event_in_mempool(event: CustomEvent) {
            // let previous_event = EventStorage::<T>::get(event.id - 1);
            // let previous_hash = previous_event.map(|e| e.calculate_hash());
        
            // let new_event = CustomEvent::new(event.id, event.data, event.timestamp, event.block_height, previous_hash);
            EventStorage::<T>::insert(event.id, event);
        }
        fn fetch_and_process_events_from_queue() -> Result<(), Error<T>> {
            // Fetch events from the queue using the RPC call
            let response = Self::fetch_all_events()?;
            let events: Vec<(CustomEvent, i32)> = serde_json::from_slice(&response)
                .map_err(|_| <Error<T>>::HttpFetchingError)?;
    
            for (event, _priority) in events {
                Self::validate_and_process_event(event)?;
            }
    
            Ok(())
        }
        fn get_event(event_id: u64) -> Option<CustomEvent> {
            Some(EventStorage::<T>::get(event_id))
        }
        fn hash_event(event: &CustomEvent) -> H256 {
            let encoded = event.encode();
            sp_io::hashing::blake2_256(&encoded).into()
        }
        fn create_inclusion_transaction() -> Result<(), &'static str> {
            let mut events = Vec::new();
            for event_id in EventStorage::<T>::iter_keys() {
                if let event = EventStorage::<T>::get(event_id) {
                    events.push(event);
                }
            }
    
            let call = Call::<T>::submit_inclusion_transaction { events };
    
            // // Submit the transaction
            // T::SubmitTransaction::submit_unsigned_transaction(call.into())
            //     .map_err(|_| "Failed to submit transaction")?;
    
            Ok(())
        }
        fn synchronize_events_with_peers() -> Result<(), Error<T>> {
            let event_ids: Vec<u64> = EventStorage::<T>::iter_keys().collect();
        
            for event_id in event_ids {
                if let Some(event) = Self::get_event(event_id) {
                    if !Self::validate_and_process_event(event.clone()).is_ok() {
                        // Request the event from other workers
                        let missing_event = Self::request_event_from_peers(event_id)?;
                        Self::validate_and_process_event(missing_event)?;
                    }
                }
            }
        
            Ok(())
        }
    
        fn request_event_from_peers(event_id: u64) -> Result<CustomEvent, Error<T>> {
            // Implement RPC call to request the event from other workers
            // Deserialize the response and return the event
            unimplemented!()
        }
        // fn verify_inclusion_tx(tx: Transaction) -> Result<(), Error<T>> {
        //     // Verify the events included in the transaction
        //     for event in tx.events {
        //         if !Self::is_event_in_mempool(event) {
        //             // Make a callback to request the event
        //             Self::request_event_from_follower(event.id)?;
        //         }
        //     }

        //     Ok(())
        // }

        // fn is_event_in_mempool(event: Event) -> bool {
        //     // Check if the event is in the local mempool
        //     Mempool::<T>::get().contains(&event)
        // }

        // fn request_event_from_follower(event_id: u64) -> Result<Event, Error<T>> {
        //     // Make an RPC call to request the event from the Chain-Follower
        //     let event = // RPC call logic...
        //     Ok(event)
        // }

        fn construct_url(path: &str) -> String {
            const DEFAULT_HOST: &str = "http://scrolls-1";
            const DEFAULT_PORT: &str = "4123";

            format!("{}:{}{}", DEFAULT_HOST, DEFAULT_PORT, path)
        }

        fn fetch_and_process_data() -> Result<(), &'static str> {
            if let Ok(events) = Self::fetch_all_events() {
                for event in events {
                    if let Err(e) = Self::validate_and_process_event(event) {
                        log::error!("Failed to process event: {:?}", e);
                    }
                }
            } else {
                log::error!("Failed to fetch events");
            }
            Ok(())
        }

        fn process_stored_data(storage_key: &[u8]) -> Result<(), &'static str> {
            if let Some(data) = sp_io::offchain::local_storage_get(
                sp_runtime::offchain::StorageKind::PERSISTENT,
                storage_key,
            ) {
                let assets: Vec<Asset> =
                    serde_json::from_slice(&data).map_err(|_| "Failed to parse JSON data")?;
    
                for asset in assets {
                    log::info!("Asset ID: {}, Quantity: {}", asset.asset_id, asset.quantity);
                    // Store the asset data in pallet storage (if necessary)
                    // e.g., AssetStorage::<T>::insert(asset.asset_id, asset);
                }
            }
            Ok(())
        }
      
        
        fn fetch_data(url: &str) -> Result<(), &'static str> {
            const FETCH_TIMEOUT_PERIOD: u64 = 3000; // in milliseconds
            let request = rt_offchain::http::Request::get(url);

            let timeout = sp_io::offchain::timestamp()
                .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));

            let pending = request
                .deadline(timeout)
                .send()
                .map_err(|_| "Failed to send request")?;

            let response = pending
                .try_wait(timeout)
                .map_err(|_| "Timeout while waiting for response")?
                .map_err(|_| "Failed to receive response")?;

            if response.code != 200 {
                log::error!("Unexpected status code: {}", response.code);
                return Err("Non-200 status code returned from API");
            }

            let body = response.body().collect::<Vec<u8>>();
            log::info!("Response body: {:?}", body);

            Ok(())
        }

        fn fetch_address_stake_assets() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/info/address/stake/assets/");
            // let data = Self::fetch_data(&url)?;
            // process_address_stake_assets(data)
            Ok(())
        }

        fn fetch_addresses_assets() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/info/addresses/assets/");
            // let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_pools(page: u32) -> Result<(), &'static str> {
            let url = Self::construct_url(&format!("/api/info/pools/{}", page));
            // let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_token_nft_status() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/info/tokens/isNft/");
            // let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_epoch_stake_amount(stake_addr: &str, epoch: u32) -> Result<(), &'static str> {
            let url = Self::construct_url(&format!(
                "/api/info/epoch/stake/amount/{}/{}",
                stake_addr, epoch
            ));
            // let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_reward_amount(stake_addr: &str) -> Result<(), &'static str> {
            let url = Self::construct_url(&format!("/api/info/reward/amount/{}", stake_addr));
            // let data = Self::fetch_data(&url)?;
            // Process data as needed
            Ok(())
        }

        fn fetch_epoch_changes(from_epoch: u32, to_epoch: u32) -> Result<(), &'static str> {
            let url = Self::construct_url(&format!(
                "/api/aya/epoch/change/from/{}/{}",
                from_epoch, to_epoch
            ));
            // let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_latest_epoch_change() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/aya/epoch/change/latest");
            // let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_current_epoch() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/aya/epoch/current/");
            // let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }
        fn verify_event_sequence(events: &[CustomEvent]) -> Result<(), &'static str> {
            // Implement logic to verify the sequence and order of events
            // This function should compare events with the local mempool or state
            for i in 1..events.len() {
                if events[i].previous_hash != Some(events[i - 1].calculate_hash()) {
                    return Err("Event sequence is invalid");
                }
            }
            Ok(())
        }
    }
    use scale_info::prelude::string::String;

    #[derive(Deserialize, Debug)]
    struct Asset {
        // Define the expected fields
        asset_id: String,
        quantity: u64,
    }

    impl<T: Config> Pallet<T> {
        fn process_response(data: Vec<u8>) -> Result<(), &'static str> {
            if let Ok(assets) = serde_json::from_slice::<Vec<Asset>>(&data) {
                // Process each asset
                for asset in assets {
                    log::info!("Asset ID: {}, Quantity: {}", asset.asset_id, asset.quantity);
                }
            } else {
                log::error!("Failed to parse JSON data");
                return Err("Failed to parse JSON");
            }

            Ok(())
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        // pub fn trigger_fetch(origin: OriginFor<T>) -> DispatchResult {
        //     let _who = ensure_signed(origin)?;

        //     match Self::fetch_and_process_data() {
        //         Ok(_) => {
        //             Self::deposit_event(Event::DataFetchedSuccessfully);
        //             Ok(())
        //         },
        //         Err(_e) => {
        //             Err(Error::<T>::HttpFetchingError.into())
        //         }
        //     }
        // }
        pub fn manual_fetch(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;
            const STORAGE_KEY_ASSETS: &[u8] = b"my-pallet::assets";
            const STORAGE_KEY_POOLS: &[u8] = b"my-pallet::pools";

            let assets_url = Self::construct_url("/api/info/address/stake/assets/");
            Self::fetch_data(&assets_url).map_err(|_| Error::<T>::HttpFetchingError)?;

            let pools_url = Self::construct_url("/api/info/pools/1");
            Self::fetch_data(&pools_url).map_err(|_| Error::<T>::HttpFetchingError)?;


            Ok(())
        }
        #[pallet::weight(10_000)]
        pub fn submit_inclusion_transaction(origin: OriginFor<T>, events: Vec<CustomEvent>) -> DispatchResult {
            // Ensure the call is unsigned to allow offchain workers to submit
            let _who = ensure_none(origin)?;
             // Verify event sequence and order with committee
             Self::verify_event_sequence(&events)?;
            // Logic to handle the inclusion of events in the transaction
            // Validate events, ensure proper ordering, etc.
            
            Ok(())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        InvalidEventData,
        EventNotFound,
        HttpFetchingError,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DataFetchedSuccessfully,
    }

    #[pallet::type_value]
    pub fn DefaultForRuntimeEvent() -> () {
        ()
    }

    pub trait WeightInfo {
        fn some_extrinsic() -> Weight {
            Weight::zero()
        }
    }

    impl WeightInfo for () {
        fn some_extrinsic() -> Weight {
            Weight::zero()
        }
    }
}

// Implement Off-Chain Worker for Event Fetching and Processing

// - Added CustomEvent struct and methods for creating and hashing events.
// - Implemented pallet structure with Config trait and StorageMap for event storage.
// - Added off-chain worker logic to fetch and process events from an external source.
// - Implemented leader election logic and inclusion transaction creation.
// - Enhanced HTTP request handling and error logging.
// - Added placeholder functions for future integration with priority queue and KV store.
