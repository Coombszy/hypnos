use clap::Parser;
use hypnos_library::structs::{SysState, TargetState};
use hypnos_library::utils::{fetch_state, is_alive, query_state};
use std::io::{self, Write};
use std::{process::exit, thread, time::Duration};
use system_shutdown::shutdown;

mod libs;
use libs::structs::Args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // validate arguments have been supplied correctly correctly
    let args = Args::parse();

    startup();

    // Create path to server query endpoint and health endpoint (Letting the compiler take the wheel on this one ;) )
    let query_ep = format!("{}/query_state/{}", args.server, args.mac_address);
    let fetch_ep = format!("{}/fetch_state/{}", args.server, args.mac_address);
    let health_ep = format!("{}/health", args.server);

    // Handle start up check and 3 retries
    print!("Starting agent...");
    if is_alive(&health_ep).await {
        println!("Running!");
    } else {
        let mut failed = true;
        for _ in 0..3 {
            print!("Retrying in 30 seconds...");
            io::stdout().flush().unwrap(); // Ensure that the stdout is written before sleeping the thread
            thread::sleep(Duration::from_secs(30));
            if is_alive(&health_ep).await {
                println!("Success... Running!");
                failed = false;
                break;
            }
        }
        // If still failed, shutdown
        if failed {
            exit(1)
        }
    }

    loop {
        match query_state(&query_ep).await {
            // Connection to server was successful
            Ok(r) => match r {
                // State to ingest was found!
                Some(state) => {
                    // If not in the list of states that should be ingested, do nothing
                    if conditional_states(&state) {
                        handle_state(&fetch_state(&fetch_ep).await.unwrap().unwrap());
                    }
                }
                // No state to ingest
                None => (),
            },
            // Failed to connect to server
            Err(_) => {
                let sleep = 60;
                println!("Sleeping for {sleep} seconds before retrying...");
                thread::sleep(Duration::from_secs(sleep));
            }
        }
        // Pause before next request
        thread::sleep(Duration::from_secs(args.sleep));
    }
}

// Whether or not this app should handle this state
fn conditional_states(state: &SysState) -> bool {
    match &state.target_state {
        TargetState::AgentOff => true,
        _ => false,
    }
}

// Handle State changes, All new states must be implemented here in the future
fn handle_state(state: &SysState) {
    match &state.target_state {
        TargetState::AgentOff => shutdown().unwrap(),
        _ => println!(
            "How did this happen! {:?} State should never enter the Queue!",
            &state.target_state
        ),
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

    println!(
        "{} ({}) By {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS")
    );

    // Validate that server it targeting a valid host
    if !(Args::parse().server.contains("http://") || Args::parse().server.contains("https://")) {
        println!("target server must contain `http` or `https`.");
        exit(1);
    }
}
