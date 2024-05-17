#![cfg_attr(not(feature = "std"), no_std)]
#[cfg_attr(feature = "std", macro_use)]
extern crate serde;
extern crate sp_std;
extern crate alloc;
use alloc::string::ToString;
pub use pallet::*;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*, weights::Weight};
    use frame_system::{offchain::*, pallet_prelude::*};

    use scale_info::prelude::format;
    use serde::{Deserialize, Serialize};
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
    use alloc::string::ToString; 
    #[pallet::config]
    pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
        type AuthorityId: Public;
        // Authority identifier for signing transactions
        // type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            if let Err(e) = Self::fetch_and_process_data() {
                log::error!("Error fetching and sending data: {:?}", e);
            }

            if let Err(e) = Self::fetch_all_events() {
                log::error!("Error fetching all events: {:?}", e);
            }

            // const STORAGE_KEY_ASSETS: &[u8] = b"my-pallet::assets";
            // const STORAGE_KEY_POOLS: &[u8] = b"my-pallet::pools";

            // let _ = Self::fetch_data(Self::construct_url("/api/info/address/stake/assets/"), STORAGE_KEY_ASSETS);
            // let _ = Self::fetch_data(Self::construct_url("/api/info/pools/1"), STORAGE_KEY_POOLS);

            // Optionally process data immediately or at a different interval/trigger
            // let _ = Self::process_stored_data(STORAGE_KEY_ASSETS);
            // let _ = Self::process_stored_data(STORAGE_KEY_POOLS);
        }
    }

    impl<T: Config> Pallet<T> {
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

        fn fetch_data(url: &str, storage_key: &[u8]) -> Result<(), &'static str> {
            let request = http::Request::get(url);
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(8000));
            let pending = request
                .add_header("User-Agent", "SubstrateOffchainWorker")
                .deadline(deadline)
                .send()
                .map_err(|_| "Failed to send request")?;

            let response = pending
                .try_wait(deadline)
                .map_err(|_| "Timeout while waiting for response")?
                .map_err(|_| "Failed to receive response")?;

            if response.code != 200 {
                log::error!("Unexpected status code: {}", response.code);
                return Err("Non-200 status code returned from API");
            }

            let data = response.body().collect::<Vec<u8>>();
            sp_io::offchain::local_storage_set(
                sp_runtime::offchain::StorageKind::PERSISTENT,
                storage_key,
                &data,
            );

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
            // Perform fetch operations immediately
            // let _ = Self::fetch_data(Self::construct_url("/api/info/address/stake/assets/"), STORAGE_KEY_ASSETS);
            // let _ = Self::fetch_data(Self::construct_url("/api/info/pools/1"), STORAGE_KEY_POOLS);

            Ok(())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
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

// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
//     fn offchain_worker(block_number: BlockNumberFor<T>) {
//         // Process events every block or at some interval
//         if let Err(e) = Self::process_cardano_events() {
//             log::error!("Error processing events: {:?}", e);
//         }
//     }
// }

// impl<T: Config> Pallet<T> {
//     fn process_cardano_events() -> Result<(), &'static str> {
//         let key = b"cardano_events";
//         if let Some(data) = sp_io::offchain::local_storage_get(sp_runtime::offchain::StorageKind::PERSISTENT, key) {
//             let events: Vec<Event> = serde_json::from_slice(&data).map_err(|_| "Failed to parse stored data")?;

//             for event in events {
//                 log::info!("Processing stored Cardano event: {:?}", event);
//                 // Here you can add further processing, like submitting on-chain transactions
//             }
//         }
//         Ok(())
//     }
// }
// Consistency and Ordering

//     Timestamps and Sequence Numbers:
//         Assign timestamps or sequence numbers to each event as it's captured. This can help in maintaining the order when events are processed or compared across different nodes.
//         Ensure that clocks are synchronized across nodes if using timestamps, or use a logical clock (like Lamport timestamps) to order events without relying on synchronized real-time clocks.

//     Hash Chains:
//         Each event could include the hash of the previous event. This creates a chain that inherently orders the events and adds an additional layer of integrity checking.

//     Merkle Trees:
//         Implement Merkle trees in your offchain storage to efficiently prove the existence and integrity of the events in your queue. This is particularly useful when you need to compare queues across nodes and quickly identify discrepancies.
