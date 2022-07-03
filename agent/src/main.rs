use clap::Parser;
use std::env::args;
use system_shutdown::shutdown;

mod libs;
use libs::structs::Args;

fn main() {
    let args = Args::parse();

    println!("{}", args.server);

    // match shutdown() {
    //     Ok(_) => println!("Shutting down!"),
    //     Err(error) => eprintln!("Failed to shutdown: {}", error),
    // }
}
