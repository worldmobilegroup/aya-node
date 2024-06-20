// benchmarking.rs
use super::*;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    manual_fetch {
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller))

    submit_empty_transaction {
        let nonce: u64 = 0;
    }: _(RawOrigin::None, nonce)

    submit_encoded_payload {
        let payload: Vec<u8> = vec![];
    }: _(RawOrigin::None, payload)

    submit_signed_transaction {
        let transaction: Vec<u8> = vec![];
    }: _(RawOrigin::Signed(caller), transaction)
}

// cargo run --release --features runtime-benchmarks -- benchmark pallet \
//   --pallet pallet_epoch \
//   --extrinsic '*' \
//   --steps 50 \
//   --repeat 20 \
//   --raw \
//   --output ./weights.rs