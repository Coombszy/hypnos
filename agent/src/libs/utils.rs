use hypnos_library::structs::SysState;

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
