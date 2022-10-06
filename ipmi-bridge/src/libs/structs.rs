use clap::Parser;

// Read cli input
#[derive(Parser)]
#[command(
    about = "Hypnos IPMI Bridge - Will fetch states from Hypnos server and issue IPMI commands to target accordingly.",
    version
)]
pub struct Args {
    #[arg(short, env = "HYPNOS_SERVER", long, value_parser, help = "Target hypnos server")]
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
        short = 't',
        env = "HYPNOS_IPMI_IP",
        long,
        value_parser,
        help = "IP address of the IPMI interface"
    )]
    pub ipmi_ip: String,

    #[arg(short = 'u', env = "HYPNOS_IPMI_USER", long, value_parser, help = "IPMI User name")]
    pub ipmi_user: String,

    #[arg(short = 'p', env = "HYPNOS_IPMI_PASSWORD", long, value_parser, help = "IPMI User password")]
    pub ipmi_password: String,

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
