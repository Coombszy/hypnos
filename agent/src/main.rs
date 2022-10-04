use clap::Parser;
use hypnos_library::structs::{SysState, TargetState};
use std::{time::Duration, thread, process::exit};
use system_shutdown::shutdown;

mod libs;
use libs::structs::Args;
use libs::utils::{fetch_state, is_alive};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    startup();

    // Create path to server query endpoint and health endpoint (Letting the compiler take the wheel on this one ;) )
    let query_ep = format!("{}/query_state/{}", Args::parse().server, Args::parse().mac_address);
    let health_ep = format!("{}/health", Args::parse().server);

    print!("Starting agent...");
    if is_alive(&health_ep).await {
        println!("Running!");
    }
    else {
        exit(1)
    }

    loop {
        match fetch_state(&query_ep).await {
            // Connection to server was successful
            Ok(r) => match r {
                // State to ingest was found!
                Some(state) => {
                    println!("State = {:?}", state);
                    handle_state(&state);
                }
                // No state to ingest
                None => (),
            }
            // Failed to connect to server
            Err(_) => {
                let sleep = 60;
                println!("Sleeping for {sleep} seconds before retrying...");
                thread::sleep(Duration::from_secs(sleep));
            },
        }
        // Pause before next request
        thread::sleep(Duration::from_secs(Args::parse().sleep));
    }
}

// Handle State changes, All new states must be implemented here in the future
fn handle_state(state: &SysState) {
    match &state.target_state {
        TargetState::Off => shutdown().unwrap(),
        TargetState::On => println!("How did this happen! On State should never enter the Queue!")
    }
}

fn startup() {
    // Permissions check and attempt to escalate
    if sudo::check() != sudo::RunningAs::Root {
        println!("Hypnos agent needs to be running as root to control the machine power state. Attempting to escalate...");
        let escalate_result = sudo::escalate_if_needed();
        match escalate_result {
            Ok(_) => (),
            Err(_) => {
                println!("Failed to escalate");
                exit(1);
            }
        }
    }

    println!("{} ({}) By {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), env!("CARGO_PKG_AUTHORS"));
    
    // Validate that server it targeting a valid host
    if !(Args::parse().server.contains("http://") || Args::parse().server.contains("https://")) {
        println!("target server must contain `http` or `https`.");
        exit(1);
    }
    
}
