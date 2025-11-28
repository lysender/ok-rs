use clap::Parser;

/// ok-rs: HTTP server that responds with OK, sometimes it echos
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct AppArgs {
    #[arg(long, value_name = "HOST")]
    pub host: Option<String>,

    #[arg(short, long, value_name = "PORT")]
    pub port: u16,

    /// Log headers of incoming requests
    #[arg(long = "debug-headers")]
    pub debug_headers: bool,
}
