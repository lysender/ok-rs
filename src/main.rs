use clap::Parser;

mod config;
mod error;
mod web;

use config::{AppArgs, RUST_LOG};

// Re-exports
pub use error::{Error, Result};
use web::start_server;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var(RUST_LOG).is_err() {
        std::env::set_var(RUST_LOG, "ok_rs=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = AppArgs::parse();

    let _ = start_server(args).await;
}
