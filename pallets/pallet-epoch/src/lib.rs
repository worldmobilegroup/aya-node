#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
#[cfg_attr(feature = "std", macro_use)]
extern crate serde;
extern crate sp_std;
use alloc::string::ToString;
pub use pallet::*;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

use frame_support::weights::Weight;
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, storage::types::StorageMap};
use frame_system::{offchain::*, pallet_prelude::*};
use sp_application_crypto::AppCrypto;

use sp_runtime::codec::{Decode, Encode};

use scale_info::prelude::format;

use serde_json;
use sp_runtime::offchain::*;


use sp_runtime::offchain as rt_offchain;
use sp_runtime::{Deserialize, Serialize};
use sp_std::prelude::*;
use substrate_validator_set as validator_set;

use pallet_session;

use scale_info::TypeInfo;
use sp_runtime::app_crypto::AppPublic;

use frame_support::pallet_prelude::{BoundedVec, Get, MaxEncodedLen};
use sp_std::vec::Vec;


use sp_runtime::AccountId32;


// Define the type for the maximum length
pub struct MaxDataLength;

impl Get<u32> for MaxDataLength {
    fn get() -> u32 {
        1024 // Define your max length here
    }
}

#[derive(
    Default,
    Deserialize,
    Serialize,
    Encode,
    Decode,
    Clone,
    PartialEq,
    Eq,
    TypeInfo,
    MaxEncodedLen,
    RuntimeDebug,
)]
pub struct CustomData(pub BoundedVec<u8, MaxDataLength>);

impl CustomData {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(
    Default,
    Deserialize,
    Serialize,
    Encode,
    Decode,
    Clone,
    PartialEq,
    Eq,
    TypeInfo,
    MaxEncodedLen,
    RuntimeDebug,
)]
pub struct CustomEvent {
    pub id: u64,
    pub data: CustomData,
    pub timestamp: u64,
    pub block_height: u64,
    pub last_epoch: u64,
    pub last_blockhash: BoundedVec<u8, MaxDataLength>,
    pub last_slot: u64,
    pub new_epoch: u64,
    pub new_slot: u64,
    pub new_blockhash: BoundedVec<u8, MaxDataLength>,
    pub epoch_nonce: BoundedVec<u8, MaxDataLength>,
    pub extra_entropy: Option<BoundedVec<u8, MaxDataLength>>,
}

impl CustomEvent {
    fn new(
        id: u64,
        data: Vec<u8>,
        timestamp: u64,
        block_height: u64,
        last_epoch: u64,
        last_blockhash: Vec<u8>,
        last_slot: u64,
        new_epoch: u64,
        new_slot: u64,
        new_blockhash: Vec<u8>,
        epoch_nonce: Vec<u8>,
        extra_entropy: Option<Vec<u8>>,
    ) -> Result<Self, &'static str> {
        Ok(CustomEvent {
            id,
            data: CustomData(
                BoundedVec::try_from(data).map_err(|_| "Data exceeds maximum length")?,
            ),
            timestamp,
            block_height,
            last_epoch,
            last_blockhash: BoundedVec::try_from(last_blockhash)
                .map_err(|_| "Last blockhash exceeds maximum length")?,
            last_slot,
            new_epoch,
            new_slot,
            new_blockhash: BoundedVec::try_from(new_blockhash)
                .map_err(|_| "New blockhash exceeds maximum length")?,
            epoch_nonce: BoundedVec::try_from(epoch_nonce)
                .map_err(|_| "Epoch nonce exceeds maximum length")?,
            extra_entropy: extra_entropy
                .map(|e| {
                    BoundedVec::try_from(e).map_err(|_| "Extra entropy exceeds maximum length")
                })
                .transpose()?,
        })
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use sp_core::ByteArray;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + CreateSignedTransaction<Call<Self>>
        + validator_set::Config
        + pallet_session::Config
    {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
        type AuthorityId: AppPublic + From<sp_core::sr25519::Public>;
        type ValidatorId: Clone
            + From<Self::AccountId>
            + Into<AccountId32>
            + From<<Self as pallet_session::Config>::ValidatorId>;
        type AccountId32Convert: From<AccountId32> + Into<Self::AccountId>;
        type Call: From<Call<Self>>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn event_storage)]
    pub type EventStorage<T: Config> =
        StorageMap<_, Blake2_128Concat, u64, CustomEvent, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            // Step 3: Message Processing
            if let Err(e) = Self::fetch_and_process_events_from_queue() {
                log::error!("Error fetching and processing events: {:?}", e);
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
    impl<T: Config> Pallet<T>
    where
        <T as pallet::Config>::ValidatorId:
            Clone + Into<AccountId32> + From<<T as pallet_session::Config>::ValidatorId>,
        <T as pallet_session::Config>::ValidatorId: Clone,
    {
        fn convert_session_validator_id_to_pallet_validator_id(
            key: <T as pallet_session::Config>::ValidatorId,
        ) -> <T as pallet::Config>::ValidatorId {
            key.into()
        }
    }

    impl<T: Config> Pallet<T>
    where
        T: frame_system::offchain::SendTransactionTypes<Call<T>>,
        
    {
        fn create_inclusion_transaction() -> Result<(), &'static str> {
            let mut events = Vec::new();
            for (event_id, event) in EventStorage::<T>::iter() {
                events.push(event);
            }
    
            let call = Call::<T>::submit_inclusion_transaction { events };
    
            // Submit the transaction
            frame_system::offchain::SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
                .map_err(|_| "Failed to submit transaction")?;
    
            Ok(())
        }
        
    }

    impl<T: Config> Pallet<T>
    where
        <T as pallet::Config>::ValidatorId:
            Clone + Into<AccountId32> + From<<T as pallet_session::Config>::ValidatorId>,
        <T as pallet_session::Config>::ValidatorId: Clone,
        T::AuthorityId: AppCrypto + From<sp_core::sr25519::Public>,
    {
        // Step 5: Message Cleanup
        fn cleanup_processed_events() {
            // Remove events from storage that have been included in the blockchain
            for (event_id, _) in EventStorage::<T>::iter() {
                EventStorage::<T>::remove(event_id);
            }
        }

        fn fetch_local_keys() -> Vec<T::AuthorityId> {
            let key_type_id = T::AuthorityId::ID;
            sp_io::crypto::sr25519_public_keys(key_type_id)
                .into_iter()
                .map(|key| T::AuthorityId::from(key))
                .collect()
        }
        // Function to convert ValidatorId to AuthorityId
        fn convert_validator_id_to_authority_id(
            key: <T as pallet::Config>::ValidatorId,
        ) -> Result<T::AuthorityId, &'static str> {
            // Convert ValidatorId to AccountId32
            let account_id32: AccountId32 = key.into();

            // Retrieve the public keys and find the matching one
            let public_key = sp_io::crypto::sr25519_public_keys(T::AuthorityId::ID)
                .into_iter()
                .find(|pk| AccountId32::from(*pk) == account_id32)
                .ok_or("Failed to find AuthorityId for the given ValidatorId")?;

            // Convert the public key to AuthorityId
            Ok(T::AuthorityId::from(public_key))
        }

        // Function to convert AuthorityId to AccountId32
        fn convert_to_account_id32(key: T::AuthorityId) -> AccountId32 {
            let public_key = key.to_raw_vec();
            AccountId32::from_slice(&public_key)
                .expect("Failed to convert AuthorityId to AccountId32")
        }

        fn is_leader() -> bool {
            // Fetch the current set of validators
            let validators = validator_set::Validators::<T>::get();
            // Get the current session index
            let current_index = pallet_session::Pallet::<T>::current_index();

            if let Some(session_leader) = validators.get(current_index as usize % validators.len())
            {
                // // Convert session's ValidatorId to pallet's ValidatorId
                let leader = Self::convert_session_validator_id_to_pallet_validator_id(
                    session_leader.clone(),
                );

                // // Convert leader to AuthorityId
                if let Ok(leader_authority_id) = Self::convert_validator_id_to_authority_id(leader)
                {
                    let local_keys = Self::fetch_local_keys();

                    for local_key in local_keys {
                        if local_key == leader_authority_id {
                            return true;
                        }
                    }
                }
            }
            false
        }

        fn fetch_all_events() -> Result<Vec<u8>, Error<T>> {
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

            // Next we fully read the response body and collect it to a vector of bytes.
            Ok(body)
        }

        fn fetch_remote_events() -> Vec<CustomEvent> {
            // Implement the logic to fetch remote events from other nodes.
            // This might involve sending HTTP requests to other nodes and parsing the responses.
            // For simplicity, let's assume we have the URLs of other nodes stored somewhere.

            // let urls = vec!["http://node1:5555", "http://node2:5555"]; // Replace with actual URLs
            let all_events = Vec::new();

            // for url in urls {
            //     let response = Self::fetch_data(&format!("{}/list_all_events", url))?;
            //     let events: Vec<CustomEvent> =
            //         serde_json::from_slice(&response).map_err(|_| <Error<T>>::HttpFetchingError)?;
            //     all_events.extend(events);
            // }

            all_events
        }

        fn finalize_transactions(validated_events: Vec<CustomEvent>) {
            // for event in validated_events {
            //     Self::store_event_in_mempool(event);
            // }
        }

        // Step 4: Message Validation
        fn validate_and_process_event(event: CustomEvent) -> Result<(), Error<T>> {
            // Validate the event data
            if event.timestamp == 0 || event.block_height == 0 {
                return Err(Error::<T>::InvalidEventData);
            }

            if event.data.0.is_empty() {
                return Err(Error::<T>::InvalidEventData);
            }

            // Process the event (e.g., store in mempool)
            Self::store_event_in_mempool(event).map_err(|_| Error::<T>::StorageOverflow)?;

            Ok(())
        }

        // Step 2: Message Storage
        fn store_event_in_mempool(event: CustomEvent) -> Result<(), &'static str> {
            EventStorage::<T>::insert(event.id, event);
            Ok(())
        }

        // Step 3: Message Processing
        fn fetch_and_process_events_from_queue() -> Result<(), Error<T>> {
            let response = Self::fetch_all_events()?;
            let events: Vec<(CustomEvent, i32)> =
                serde_json::from_slice(&response).map_err(|_| <Error<T>>::HttpFetchingError)?;

            for (event, _priority) in events {
                Self::validate_and_process_event(event)?;
            }

            Ok(())
        }

        fn get_event(event_id: u64) -> Option<CustomEvent> {
            Some(EventStorage::<T>::get(event_id))
        }

      

        fn request_event_from_peers(event_id: u64) -> Result<CustomEvent, Error<T>> {
            let url = Self::construct_url(&format!("/api/events/{}", event_id));
            let response = Self::fetch_data(&url).map_err(|_| <Error<T>>::HttpFetchingError)?;

            let event: CustomEvent =
                serde_json::from_slice(&response).map_err(|_| <Error<T>>::HttpFetchingError)?;

            Ok(event)
        }

        fn construct_url(path: &str) -> String {
            const DEFAULT_HOST: &str = "http://scrolls-1";
            const DEFAULT_PORT: &str = "4123";

            format!("{}:{}{}", DEFAULT_HOST, DEFAULT_PORT, path)
        }

        fn fetch_and_process_data() -> Result<(), &'static str> {
            // Example: Fetch data from multiple endpoints
            let assets_url = Self::construct_url("/api/info/address/stake/assets/");
            // Self::fetch_data(&assets_url)?;

            // let pools_url = Self::construct_url("/api/info/pools/1"); // example with page number
            // Self::fetch_data(&pools_url)?;

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
                }
            }
            Ok(())
        }

        fn fetch_data(url: &str) -> Result<Vec<u8>, &'static str> {
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

            Ok(body)
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
        pub fn manual_fetch(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            // Fetch and process data from the priority queue
            match Self::fetch_and_process_events_from_queue() {
                Ok(_) => {
                    Self::deposit_event(Event::DataFetchedSuccessfully);
                    Ok(())
                }
                Err(e) => {
                    log::error!("Error in manual fetch: {:?}", e);
                    Err(Error::<T>::HttpFetchingError.into())
                }
            }
        }

        #[pallet::weight(10_000)]
        pub fn submit_inclusion_transaction(
            origin: OriginFor<T>,
            events: Vec<CustomEvent>,
        ) -> DispatchResult {
            // Ensure the call is unsigned to allow offchain workers to submit
            let _who = ensure_none(origin)?;
            // Verify event sequence and order with committee

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
