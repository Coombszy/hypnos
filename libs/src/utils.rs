use crate::structs::{CargoPkgInfo, SysState};
use hex::{FromHex, FromHexError};

// Draws start screen containing app version and ascii
pub fn draw_start_screen(package_info: &CargoPkgInfo) {
    let ascii_name = r#"     _   _                             
    | | | |_   _ _ __  _ __   ___  ___ 
    | |_| | | | | '_ \| '_ \ / _ \/ __|
    |  _  | |_| | |_) | | | | (_) \__ \
    |_| |_|\__, | .__/|_| |_|\___/|___/
           |___/|_|                    "#;

    let offset = 47 - package_info.name.len();
    println!("{} v{}", &ascii_name, package_info.version);
    println!("{}{}", " ".repeat(offset), package_info.name);
    println!("\n   Created by {}", package_info.authors);
    println!("==================================================")
}

pub fn generic_mac_address(mac_address: &String) -> Result<[u8; 6], FromHexError> {
    // If contains ":", remove
    if mac_address.contains(':') {
        return <[u8; 6]>::from_hex(mac_address.replace(':', ""));
    }

    <[u8; 6]>::from_hex(mac_address)
}

// Query state from remote server
pub async fn query_state(server: &String) -> Result<Option<SysState>, Box<dyn std::error::Error>> {
    let response = reqwest::get(server).await;

    match response {
        Ok(r) => {
            if r.status() == reqwest::StatusCode::OK {
                // This *should* always parse
                let state: SysState = r.json().await.unwrap();
                return Ok(Some(state));
            }
            Ok(None)
        }
        Err(e) => {
            println!("Failed to reach server! \n {}", e);
            Err(Box::new(e))
        }
    }
}

// Fetches state from remote server
pub async fn fetch_state(server: &String) -> Result<Option<SysState>, Box<dyn std::error::Error>> {
    let response = reqwest::get(server).await;

    match response {
        Ok(r) => {
            if r.status() == reqwest::StatusCode::OK {
                // This *should* always parse
                let state: SysState = r.json().await.unwrap();
                return Ok(Some(state));
            }
            Ok(None)
        }
        Err(e) => {
            println!("Failed to reach server! \n {}", e);
            Err(Box::new(e))
        }
    }
}

// Validate that the remote server is alive
// The `println!`s need a \n at the start as the caller of this function
// is using a `print!`
pub async fn is_alive(server: &String) -> bool {
    let response = reqwest::get(server).await;

    match response {
        Ok(r) => {
            if r.status() == reqwest::StatusCode::OK {
                // This *should* always parse
                return true;
            }
            println!("\nServer did not respond healthily ({})", r.status());
            false
        }
        Err(e) => {
            println!("\nFailed to reach server! \n {}", e);
            false
        }
    }
}
