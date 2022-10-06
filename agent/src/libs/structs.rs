use clap::Parser;

// Read cli input
#[derive(Parser)]
#[command(
    about = "Hypnos Agent - Will fetch states from Hypnos server and change current system state accordingly.",
    version
)]
pub struct Args {
    #[arg(
        short,
        env = "HYPNOS_SERVER",
        long,
        value_parser,
        help = "Target hypnos server"
    )]
    pub server: String,

    #[arg(
        short,
        env = "HYPNOS_MAC_ADDRESS",
        long,
        value_parser,
        help = "MAC address to consume from hypnos server"
    )]
    pub mac_address: String,

    #[arg(
        short = 'w',
        env = "HYPNOS_SLEEP",
        long,
        value_parser,
        default_value_t = 5,
        help = "Time to sleep between checking for new states"
    )]
    pub sleep: u64,
}
