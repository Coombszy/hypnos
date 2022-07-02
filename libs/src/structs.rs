use serde::{Deserialize, Serialize};

use crate::utils::generic_mac_address;

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
        return generic_mac_address(&self.mac_address);
    }
}

// State query from agent
#[derive(Deserialize)]
pub struct StateQuery {
    pub mac_addresses: Vec<String>,
}