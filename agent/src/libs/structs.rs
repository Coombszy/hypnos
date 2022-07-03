use clap::Parser;

// Read cli input
#[derive(Parser)]
#[clap(about, version)]
pub struct Args {
    // Target hypnos server
    #[clap(short, long, value_parser)]
    pub server: String,
}
