#![cfg_attr(not(feature = "std"), no_std)]
#[cfg_attr(feature = "std", macro_use)]
extern crate serde;
extern crate sp_std;

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
    use sp_consensus_aura::ed25519::AuthorityId;
    use sp_core::Public;
    use sp_runtime::offchain::*;
    use sp_runtime::offchain::{http, Duration};
    use sp_std::prelude::*;
    use serde_json;

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
        }
    }

    impl<T: Config> Pallet<T> {
        fn construct_url(path: &str) -> String {
            const DEFAULT_HOST: &str = "http://scrolls-1";
            const DEFAULT_PORT: &str = "4123";

            format!("{}:{}{}", DEFAULT_HOST, DEFAULT_PORT, path)
        }

        fn fetch_and_process_data() -> Result<(), &'static str> {
            // Example: Fetch data from multiple endpoints
            let assets_url = Self::construct_url("/api/info/address/stake/assets/");
            Self::fetch_data(&assets_url)?;

            let pools_url = Self::construct_url("/api/info/pools/1"); // example with page number
            Self::fetch_data(&pools_url)?;

            Ok(())
        }

        fn fetch_data(url: &str) -> Result<(), &'static str> {
            let request = http::Request::get(url);
             // Get the current time and compute the deadline
            let current_time = sp_io::offchain::timestamp();
            let deadline = current_time.add(Duration::from_millis(8_000));
            // Add headers and set timeout
            let pending = request
                .add_header("User-Agent", "SubstrateOffchainWorker")
                .deadline(deadline)
                .send()
                .map_err(|_| "Failed to send request")?;

            // Handling the response
            let response = pending
                .try_wait(deadline)
                .map_err(|_| "Timeout while waiting for response")?
                .map_err(|_| "Failed to receive response")?;

            if response.code != 200 {
                log::error!("Unexpected status code: {}", response.code);
                return Err("Non-200 status code returned from API");
            }

            // Log the successful fetch
            log::info!("Successfully fetched data from: {}", url);
            Self::process_response(response.body().collect::<Vec<u8>>())?;

            Ok(())
        }

        fn fetch_address_stake_assets() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/info/address/stake/assets/");
            let data = Self::fetch_data(&url)?;
            // process_address_stake_assets(data)
            Ok(())
        }

        fn fetch_addresses_assets() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/info/addresses/assets/");
            let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_pools(page: u32) -> Result<(), &'static str> {
            let url = Self::construct_url(&format!("/api/info/pools/{}", page));
            let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_token_nft_status() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/info/tokens/isNft/");
            let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_epoch_stake_amount(stake_addr: &str, epoch: u32) -> Result<(), &'static str> {
            let url = Self::construct_url(&format!("/api/info/epoch/stake/amount/{}/{}", stake_addr, epoch));
            let data = Self::fetch_data(&url)?;
                // save to local storage queue
            Ok(())
        }

        fn fetch_reward_amount(stake_addr: &str) -> Result<(), &'static str> {
            let url = Self::construct_url(&format!("/api/info/reward/amount/{}", stake_addr));
            let data = Self::fetch_data(&url)?;
            // Process data as needed
            Ok(())
        }

        fn fetch_epoch_changes(from_epoch: u32, to_epoch: u32) -> Result<(), &'static str> {
            let url = Self::construct_url(&format!("/api/aya/epoch/change/from/{}/{}", from_epoch, to_epoch));
            let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_latest_epoch_change() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/aya/epoch/change/latest");
            let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }

        fn fetch_current_epoch() -> Result<(), &'static str> {
            let url = Self::construct_url("/api/aya/epoch/current/");
            let data = Self::fetch_data(&url)?;
            // save to local storage queue
            Ok(())
        }
    }
    use scale_info::prelude::string::String;
    use sp_runtime::Deserialize;
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
        pub fn trigger_fetch(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            match Self::fetch_and_process_data() {
                Ok(_) => {
                    Self::deposit_event(Event::DataFetchedSuccessfully);
                    Ok(())
                },
                Err(_e) => {
                    Err(Error::<T>::HttpFetchingError.into())
                }
            }
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

