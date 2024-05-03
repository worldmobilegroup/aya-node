#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
#[cfg(test)]
mod tests;


#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, weights::Weight};
    use frame_system::{pallet_prelude::*, offchain::*};
    use sp_runtime::offchain::*;
    use sp_consensus_aura::ed25519::AuthorityId;
   
    use sp_core::Public;

	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		type AuthorityId: Public;
        
    }

    #[pallet::pallet]
	
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
       
        fn offchain_worker(block_number: BlockNumberFor<T>) {
            // off-chain worker logic
            log::info!("Hello from offchain worker: {:?}", block_number);
            log::info!("Hello from offchain worker: {:?}", block_number);

            // Simulate the chain follower code
            let config = Config {
                cdp_host: "localhost".to_string(),
                cdp_port: 8080,
                queue_host: "localhost".to_string(),
                queue_port: 9000,
                public_key: Some("public_key".to_string()),
                private_key: Some("private_key".to_string()),
                channel: "channel".to_string(),
                cursor_path: Some("cursor".to_string()),
                dbsync_path: Some("dbsync".to_string()),
            };

            let bootstrapper = config.bootstrapper(&Default::default(), &Default::default());
            let mut cursor = bootstrapper.build_cursor();

            // Simulate pushing data to the queue
            let client = reqwest::blocking::Client::new();
            let payload = serde_json::json!({ "message": "Hello from Substrate" });
            let response = client
                .post(&format!("http://{}/", config.queue_host))
                .header("Content-Type", "application/json")
                .json(&payload)
                .send();

            match response {
                Ok(resp) => log::info!("Send Message to Queue; Response: {:?}", resp),
                Err(e) => log::error!("Failed to send request: {:?}", e),
            }

            // Simulate writing the cursor
            let point = crosscut::PointArg::new(block_number, 0);
            let cursor_str = point.to_string();
            std::fs::write(config.cursor_path(), &cursor_str).expect("couldn't write cursor");

            log::debug!(
                "new cursor {} saved to file {}",
                &cursor_str,
                config.cursor_path(),
            );


        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // Add your extrinsics here
    }

    #[pallet::error]
    pub enum Error<T> {
        // Add your error variants here
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // Add your event variants here
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