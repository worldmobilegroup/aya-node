#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

extern crate serde;
extern crate sp_std;
use alloc::{string::ToString, vec::Vec};
use core::primitive::str;

pub use pallet::*;


use frame_support::{
    dispatch::DispatchResult, pallet_prelude::*, storage::types::StorageMap,
    
};
use frame_system::{offchain::*, pallet_prelude::*};
use serde::{Deserialize, Serialize };

use sp_application_crypto::{AppCrypto};

use sp_runtime::{
    app_crypto::AppPublic,
    codec::{Decode, Encode},
    offchain::{self as rt_offchain},
    
};

use scale_info::TypeInfo;
use serde_json;
use substrate_validator_set as validator_set;

use pallet_session;

use frame_support::pallet_prelude::{Get};
use sp_runtime::AccountId32;
use sp_std::prelude::*;

use sp_application_crypto::sr25519;

use sp_application_crypto::ed25519;

use scale_info::prelude::string::String;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock; // Add this line to declare the mock module
pub mod weights;

#[derive(
    Default, Deserialize, Serialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo,
)]
pub struct CustomEvent {
    pub id: u64,
    pub data: CustomData,
    pub timestamp: u64,
    pub block_height: u64,
}

#[derive(
    Default, Deserialize, Serialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo,
)]
pub struct CustomData {
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: EpochChangeData,
}

#[derive(
    Default, Deserialize, Serialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo,
)]
pub struct EpochChangeData {
    pub last_epoch: u64,
    pub last_blockhash: String,
    pub last_slot: u64,
    pub new_epoch: u64,
    pub new_slot: u64,
    pub new_blockhash: String,
    pub epoch_nonce: String,
    pub extra_entropy: Option<String>,
}



#[frame_support::pallet]
pub mod pallet {
    use super::*;
  

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
        type UnsignedPriority: Get<u64>;
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
        fn offchain_worker(_block_number: BlockNumberFor<T>) {
            if Self::is_leader() {
                if let Err(e) = Self::fetch_and_process_events_from_queue() {
                    log::error!("Error processing events: {:?}", e);
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
        fn fetch_and_process_events_from_queue() -> Result<(), Error<T>> {
            log::info!("Fetching all events from the queue");

            // Fetch all events as a JSON response
            let response = Self::process_real_event()?;

            // Deserialize the JSON response into the expected structure
            let events: Vec<Vec<CustomEvent>> = serde_json::from_slice(&response).map_err(|e| {
                log::error!("Failed to deserialize events: {:?}", e);
                <Error<T>>::JsonSerializationError
            })?;

            // Process each event
            for event_group in events.iter() {
                for event in event_group.iter() {
                    log::info!("Processing event: {:?}", event);
                    Self::validate_and_process_event(event.clone())?;
                    // Encode the event payload
                    let payload = event.encode();

                    // Submit the encoded payload as an unsigned transaction
                    log::info!(
                        "Submitting unsigned transaction with payload: {:?}",
                        payload
                    );

                    // Create and submit the call
                    let call = Call::<T>::submit_encoded_payload {
                        payload: payload.clone(),
                    };

                    match frame_system::offchain::SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()) {
                Ok(_) => log::info!("Transaction submitted successfully"),
                Err(e) => log::error!("Error submitting unsigned transaction: {:?}", e),
            }
                }
            }

            // Return Ok(()) at the end of the function
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
        fn process_real_event() -> Result<Vec<u8>, Error<T>> {
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

            // Initiate an external HTTP POST request
            let request =
                rt_offchain::http::Request::post(HTTP_REMOTE_REQUEST, vec![&json_payload])
                    .add_header("User-Agent", HTTP_HEADER_USER_AGENT)
                    .add_header(HTTP_HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
                    .deadline(
                        sp_io::offchain::timestamp()
                            .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD)),
                    )
                    .send()
                    .map_err(|_| <Error<T>>::HttpFetchingError)?;

            let response = request
                .try_wait(
                    sp_io::offchain::timestamp()
                        .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD)),
                )
                .map_err(|_| <Error<T>>::HttpFetchingError)?
                .map_err(|_| <Error<T>>::HttpFetchingError)?;

            if response.code != 200 {
                log::error!("Non-200 response code: {}", response.code);
                return Err(<Error<T>>::HttpFetchingError);
            }
            let response_body = response.body().collect::<Vec<u8>>();
            let json_string = String::from_utf8(response_body).map_err(|e| {
                log::error!("Failed to parse response body as UTF-8: {:?}", e);
                <Error<T>>::InvalidUtf8
            })?;

            log::info!("HTTP Response Body: {}", json_string);

            // Parse the outer JSON-RPC response
            let rpc_response: serde_json::Value =
                serde_json::from_str(&json_string).map_err(|e| {
                    log::error!("Failed to parse JSON-RPC response: {:?}", e);
                    <Error<T>>::InvalidResponseFormat
                })?;

            log::info!("RPC Response: {:?}", rpc_response);

            // Extract the "result" field, which is a stringified JSON
            let result_str = rpc_response["result"].as_str().ok_or_else(|| {
                log::error!("Failed to extract result string");
                <Error<T>>::InvalidResponseFormat
            })?;

            log::info!("Result string: {}", result_str);

            // Parse the inner JSON (the actual event data)
            let inner_response: serde_json::Value =
                serde_json::from_str(result_str).map_err(|e| {
                    log::error!("Failed to parse inner JSON response: {:?}", e);
                    <Error<T>>::InvalidResponseFormat
                })?;

            log::info!("Inner response: {:?}", inner_response);

            // Extract the events array
            let events = inner_response["events"].as_array().ok_or_else(|| {
                log::error!("Failed to extract events array");
                <Error<T>>::InvalidResponseFormat
            })?;

            log::info!("Events: {:?}", events);

            // Process each event
            let mut processed_events = Vec::new();
            for (i, event_group) in events.iter().enumerate() {
                let mut processed_group = Vec::new();
                for (j, event) in event_group
                    .as_array()
                    .ok_or_else(|| {
                        log::error!("Event group {} is not an array", i);
                        <Error<T>>::InvalidResponseFormat
                    })?
                    .iter()
                    .enumerate()
                {
                    log::info!("Processing event {}.{}: {:?}", i, j, event);

                    // Check if the event is a JSON object
                    if let Some(event_obj) = event.as_object() {
                        let id = event_obj["id"].as_u64().ok_or_else(|| {
                            log::error!("Failed to extract id from event {}.{}", i, j);
                            <Error<T>>::InvalidResponseFormat
                        })?;
                        let timestamp = event_obj["timestamp"].as_u64().ok_or_else(|| {
                            log::error!("Failed to extract timestamp from event {}.{}", i, j);
                            <Error<T>>::InvalidResponseFormat
                        })?;
                        let block_height = event_obj["block_height"].as_u64().ok_or_else(|| {
                            log::error!("Failed to extract block_height from event {}.{}", i, j);
                            <Error<T>>::InvalidResponseFormat
                        })?;

                        // Parse the data field, which is a stringified JSON
                        let data_str = event_obj["data"].as_str().ok_or_else(|| {
                            log::error!("Failed to extract data string from event {}.{}", i, j);
                            <Error<T>>::InvalidResponseFormat
                        })?;

                        log::info!("Data string for event {}.{}: {}", i, j, data_str);

                        let data: CustomData = serde_json::from_str(data_str).map_err(|e| {
                            log::error!(
                                "Failed to parse event data for event {}.{}: {:?}",
                                i,
                                j,
                                e
                            );
                            <Error<T>>::JsonSerializationError
                        })?;

                        let custom_event = CustomEvent {
                            id,
                            data,
                            timestamp,
                            block_height,
                        };

                        log::info!("Processed event {}.{}: {:?}", i, j, custom_event);

                        processed_group.push(custom_event);
                    } else {
                        log::warn!("Skipping non-object event {}.{}: {:?}", i, j, event);
                    }
                }
                processed_events.push(processed_group);
            }

            // Serialize the processed events back to JSON
            let events_json = serde_json::to_string(&processed_events).map_err(|e| {
                log::error!("Failed to serialize processed events: {:?}", e);
                <Error<T>>::JsonSerializationError
            })?;

            log::info!("Processed events JSON: {}", events_json);

            Ok(events_json.into_bytes())
        }
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
        // Step 4: Message Validation
        fn validate_and_process_event(event: CustomEvent) -> Result<(), Error<T>> {
            // Validate the event data
            if event.timestamp == 0 || event.block_height == 0 {
                return Err(Error::<T>::InvalidEventData);
            }

            // Check if the data is empty (you might want to adjust this condition based on your needs)
            if event.data.event_type.is_empty() {
                return Err(Error::<T>::InvalidEventData);
            }

            // Process the event (e.g., store in mempool)
            Self::store_event_in_mempool(event.clone()).map_err(|_| Error::<T>::StorageOverflow)?;


            Ok(())
        }

        // Step 2: Message Storage
        fn store_event_in_mempool(event: CustomEvent) -> Result<(), &'static str> {
            EventStorage::<T>::insert(event.id, event);
            Ok(())
        }

        fn remove_event_from_priority_queue(event_id: u64) -> Result<(), Error<T>> {
            // Call the HTTP method to remove the event from the priority queue
            let remove_event_payload = serde_json::json!({
                "jsonrpc": "2.0",
                "method": "remove_event",
                "params": [event_id],
                "id": 1
            })
            .to_string()
            .into_bytes();
            let remove_event_payload_ref: Vec<&[u8]> = vec![&remove_event_payload];

            const HTTP_REMOTE_REQUEST: &str = "http://127.0.0.1:5555";
            const HTTP_HEADER_USER_AGENT: &str = "SubstrateOffchainWorker";
            const HTTP_HEADER_CONTENT_TYPE: &str = "Content-Type";
            const CONTENT_TYPE_JSON: &str = "application/json";
            const FETCH_TIMEOUT_PERIOD: u64 = 5000; // in milliseconds

            let request =
                rt_offchain::http::Request::post(HTTP_REMOTE_REQUEST, remove_event_payload_ref)
                    .add_header("User-Agent", HTTP_HEADER_USER_AGENT)
                    .add_header(HTTP_HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
                    .deadline(
                        sp_io::offchain::timestamp()
                            .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD)),
                    )
                    .send()
                    .map_err(|_| <Error<T>>::HttpFetchingError)?;

            let response = request
                .try_wait(
                    sp_io::offchain::timestamp()
                        .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD)),
                )
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

            Ok(())
        }

        fn get_event(event_id: u64) -> Option<CustomEvent> {
            Some(EventStorage::<T>::get(event_id))
        }
    }

    impl<T: Config> Pallet<T> {
        fn process_decoded_call(call: Call<T>) -> DispatchResult {
            match call {
                Call::process_epoch_event { nonce, payload: _ } => {
                    log::info!("Processing decoded call with nonce: {}", nonce);

                    Ok(())
                }
                _ => Err(Error::<T>::InvalidCall.into()),
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn manual_fetch(origin: OriginFor<T>) -> DispatchResult {
            ensure_signed(origin)?;

            // // Fetch and process data from the priority queue
            // match Self::fetch_and_process_events_from_queue() {
            //     Ok(_) => {
            //         Self::deposit_event(Event::DataFetchedSuccessfully);
            //         Ok(())
            //     }
            //     Err(e) => {
            //         log::error!("Error in manual fetch: {:?}", e);
            //         Err(Error::<T>::HttpFetchingError.into())
            //     }
            // }
            Ok(())
        }
        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn process_epoch_event(
            origin: OriginFor<T>,
            _nonce: u64,
            payload: Vec<u8>,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            log::info!("Received payload: {:?}", payload);

            // Decode the payload using SCALE codec
            let decoded_payload = CustomEvent::decode(&mut &payload[..]).map_err(|e| {
                log::error!("Failed to decode payload: {:?}", e);
                Error::<T>::InvalidPayload
            })?;

            log::info!("Decoded payload: {:?}", decoded_payload);

            // Process the payload as needed
            Ok(())
        }
        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn submit_encoded_payload(origin: OriginFor<T>, payload: Vec<u8>) -> DispatchResult {
            log::info!("submit_encoded_payload called with payload: {:?}", payload);

            let _who = ensure_none(origin)?;

            // Decode the payload
            let call: Call<T> = Decode::decode(&mut &payload[..]).map_err(|e| {
                log::error!("Failed to decode payload: {:?}", e);
                Error::<T>::InvalidPayload
            })?;

            // Process the decoded call
            log::info!("Processing decoded call: {:?}", call);
            Self::process_decoded_call(call)
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        InvalidEventData,
        EventNotFound,
        HttpFetchingError,
        InvalidPayload,
        InvalidCall,
        InvalidResponseFormat,
        InvalidUtf8,
        TransactionSubmissionError,
        NoEventsInQueue,
        JsonSerializationError,
    }

    #[pallet::event]
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
