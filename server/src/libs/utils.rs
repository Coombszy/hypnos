use std::{fs, process::exit};

use crate::libs::structs::TOMLData;

// Loads TOMLData struct from filename
pub fn load_config_toml(filename: String) -> TOMLData {
    // Load in raw string from config toml
    let toml_raw = match fs::read_to_string(&filename) {
        Ok(c) => c,
        // Failed to read file
        Err(e) => {
            println!("Could not read TOML file '{}'", &filename);
            println!("Error: {}", e);
            exit(1);
        }
    };
    // Convert to TOML struct
    let config_data: TOMLData = match toml::from_str(&toml_raw) {
        Ok(d) => d,
        // Failed to parse from String to TOMLData Struct
        Err(e) => {
            println!("Unable to load data from {}", &filename);
            println!("Error: {}", e);
            exit(1);
        }
    };
    config_data
}

pub fn send_magic_packet(mac_address: &[u8; 6]) -> Result<(), std::io::Error> {
    let magic_packet = wake_on_lan::MagicPacket::new(mac_address);

    magic_packet.send()
}
