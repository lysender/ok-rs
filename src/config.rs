use clap::Parser;

pub const RUST_LOG: &str = "RUST_LOG";

/// ok-rs: HTTP server that responds with OK, sometimes it echos
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    #[arg(short, long, value_name = "PORT")]
    pub port: u16,
}
