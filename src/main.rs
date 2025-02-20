use clap::Parser;

mod config;
mod error;
mod web;

use config::AppArgs;

// Re-exports
pub use error::{Error, Result};
use web::start_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = AppArgs::parse();

    let _ = start_server(args).await;
}
