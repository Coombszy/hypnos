use clap::Parser;

// Read cli input
#[derive(Parser, Debug)]
#[command(
    about = "Hypnos SSH Bridge - Will fetch states from Hypnos server and issue SSH commands to target accordingly.",
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
        short = 't',
        env = "HYPNOS_SSH_IP",
        long,
        value_parser,
        help = "IP address of the SSH target"
    )]
    pub ssh_ip: String,

    #[arg(
        short = 'u',
        env = "HYPNOS_SSH_USER",
        long,
        value_parser,
        help = "SSH User name"
    )]
    pub ssh_user: String,

    #[arg(
        short = 'p',
        env = "HYPNOS_SSH_PASSWORD",
        long,
        value_parser,
        help = "SSH User password"
    )]
    pub ssh_password: String,

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
