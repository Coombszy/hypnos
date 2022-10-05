use clap::Parser;

// Read cli input
#[derive(Parser)]
#[command(
    about = "Hypnos IPMI Bridge - Will fetch states from Hypnos server and issue IPMI commands to target accordingly.",
    version
)]
pub struct Args {
    #[arg(short, long, value_parser, help = "Target hypnos server")]
    pub server: String,

    #[arg(
        short,
        long,
        value_parser,
        help = "MAC address to consume from hypnos server"
    )]
    pub mac_address: String,

    #[arg(
        short = 't',
        long,
        value_parser,
        help = "IP address of the IPMI interface"
    )]
    pub ipmi_ip: String,   

    #[arg(
        short = 'u',
        long,
        value_parser,
        help = "IPMI User name"
    )]
    pub ipmi_user: String,

    #[arg(
        short = 'p',
        long,
        value_parser,
        help = "IPMI User password"
    )]
    pub ipmi_password: String,

    #[arg(
        short = 'w',
        long,
        value_parser,
        default_value_t = 5,
        help = "Time to sleep between checking for new states"
    )]
    pub sleep: u64,
}
