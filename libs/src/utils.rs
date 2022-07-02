use crate::structs::CargoPkgInfo;
use hex::FromHex;

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

pub fn generic_mac_address(mac_address: &String) -> [u8; 6] {
    // If contains ":", remove
    if mac_address.contains(":") {
        return <[u8; 6]>::from_hex(mac_address.replace(":", ""))
            .expect("Decoding failed");
    }

    return <[u8; 6]>::from_hex(mac_address).expect("Decoding failed");
}