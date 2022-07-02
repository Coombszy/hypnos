use serde::{Deserialize, Serialize};
use hex::FromHex;

pub struct CargoPkgInfo {
    pub name: String,
    pub version: String,
    pub authors: String,
}

// State enums
#[derive(Deserialize, Serialize, Clone)]
pub enum TargetState {
    Off,
    On,
}

// System State,
#[derive(Deserialize, Serialize, Clone)]
pub struct SysState {
    pub mac_address: String,
    pub target_state: TargetState,
}

impl SysState {
    // Convert mac address string to a hex array
    pub fn get_mac_address(&self) -> [u8; 6] {
        // If contains ":", remove
        if self.mac_address.contains(":") {
            return <[u8; 6]>::from_hex(&self.mac_address.replace(":", ""))
                .expect("Decoding failed");
        }

        return <[u8; 6]>::from_hex(&self.mac_address).expect("Decoding failed");
    }
}