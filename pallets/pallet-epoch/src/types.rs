use alloc::string::String;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_io::hashing::blake2_256;
use sp_runtime::codec::{Decode, Encode};
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Hash;
use sp_std::prelude::*;

#[derive(
    Default, Deserialize, Serialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo,
)]
pub struct CustomEvent {
    pub id: u64,
    pub data: CustomData,
    pub timestamp: u64,
    pub block_height: u64,
}

impl CustomEvent {
    pub fn is_valid(&self) -> bool {
        self.timestamp != 0 && self.block_height != 0
    }

    pub fn hash_without_timestamp(&self) -> [u8; 32] {
        let mut encoded_data = self.id.encode();
        encoded_data.extend(self.data.encode());
        encoded_data.extend(self.block_height.encode());
        BlakeTwo256::hash_of(&encoded_data).into()
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut encoded_data = self.data.event_type.encode();
        encoded_data.extend(self.data.encode());
        blake2_256(&encoded_data)
    }
}

#[derive(
    Default, Deserialize, Serialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo,
)]
pub struct CustomData {
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: EpochChangeData,
}

impl CustomData {
    pub fn hash(&self) -> [u8; 32] {
        let mut encoded_data = self.event_type.encode();
        encoded_data.extend(self.data.encode());
        blake2_256(&encoded_data)
    }
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

impl EpochChangeData {
    pub fn hash(&self) -> [u8; 32] {
        let mut encoded_data = self.last_epoch.encode();
        encoded_data.extend(self.last_blockhash.encode());
        encoded_data.extend(self.last_slot.encode());
        encoded_data.extend(self.new_epoch.encode());
        encoded_data.extend(self.new_slot.encode());
        encoded_data.extend(self.new_blockhash.encode());
        encoded_data.extend(self.epoch_nonce.encode());
        if let Some(extra_entropy) = &self.extra_entropy {
            encoded_data.extend(extra_entropy.encode());
        }
        blake2_256(&encoded_data)
    }
}
