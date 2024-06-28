// limits.rs
use frame_support::pallet_prelude::Get;
use frame_support::pallet_prelude::MaxEncodedLen;

use crate::types::CustomData;
use crate::types::CustomEvent;
use crate::types::EpochChangeData;

pub struct MaxDataLength;

impl MaxEncodedLen for CustomEvent {
    fn max_encoded_len() -> usize {
        u32::MAX as usize
    }
}

impl MaxEncodedLen for CustomData {
    fn max_encoded_len() -> usize {
        u32::MAX as usize
    }
}

impl MaxEncodedLen for EpochChangeData {
    fn max_encoded_len() -> usize {
        u32::MAX as usize
    }
}

impl Get<u32> for MaxDataLength {
    fn get() -> u32 {
        1024 // Define your max length here
    }
}

pub struct MaxPayloadLength;

impl Get<u32> for MaxPayloadLength {
    fn get() -> u32 {
        1024 // Define your max length here
    }
}

pub struct MaxEventsLength;

impl Get<u32> for MaxEventsLength {
    fn get() -> u32 {
        100 // Define your max length here
    }
}

pub struct MaxRemoveEventsLength;

impl Get<u32> for MaxRemoveEventsLength {
    fn get() -> u32 {
        100 // Define your max length here
    }
}
