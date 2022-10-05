use clap::Parser;
use hypnos_library::structs::{SysState, TargetState};
use hypnos_library::utils::{fetch_state, is_alive, query_state};
use std::{process::exit, thread, time::Duration};

mod libs;
use libs::structs::Args;
use libs::utils::{ipmi_soft_stop, ipmi_start};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // validate arguments have been supplied correctly correctly
    let args = Args::parse();

    startup();

    // Create path to server query endpoint and health endpoint (Letting the compiler take the wheel on this one ;) )
    let query_ep = format!("{}/query_state/{}", args.server, args.mac_address);
    let fetch_ep = format!("{}/fetch_state/{}", args.server, args.mac_address);
    let health_ep = format!("{}/health", args.server);

    print!("Starting bridge...");
    if is_alive(&health_ep).await {
        println!("Running!");
    } else {
        exit(1)
    }

    loop {
        match query_state(&query_ep).await {
            // Connection to server was successful
            Ok(r) => match r {
                // State to ingest was found!
                Some(state) => {
                    if conditional_states(&state) {
                        handle_state(&fetch_state(&fetch_ep).await.unwrap().unwrap(), &args);
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
        TargetState::IpmiOff => true,
        TargetState::IpmiOn => true,
        _ => false,
    }
}

// Handle State changes, All new states must be implemented here in the future
fn handle_state(state: &SysState, args: &Args) {
    match &state.target_state {
        TargetState::IpmiOff => ipmi_soft_stop(&args.ipmi_user, &args.ipmi_password, &args.ipmi_ip),
        TargetState::IpmiOn => ipmi_start(&args.ipmi_user, &args.ipmi_password, &args.ipmi_ip),
        _ => println!(
            "How did this happen! {:?} State should never enter the Queue!",
            &state.target_state
        ),
    }
}

fn startup() {
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
