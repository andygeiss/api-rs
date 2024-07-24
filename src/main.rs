use crate::prelude::*;

mod cli;
mod error;
mod middleware;
mod prelude;
mod repositories;
mod router;
mod routes;
mod security;
mod services;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().compact().init();
    // Client mode
    // cargo run create-account --id foo --password bar
    if cli::is_client_mode().await? {
        return Ok(());
    };
    // Server mode
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("🚀 {name} version {version} started successfully.");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, router::service_with_state()).await?;
    Ok(())
}
