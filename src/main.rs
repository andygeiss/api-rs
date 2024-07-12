use crate::prelude::*;

mod error;
mod middleware;
mod prelude;
mod repositories;
mod router;
mod routes;
mod security;
mod services;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("🚀 {name} version {version} started successfully.");
    tracing_subscriber::fmt().compact().init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    let state = state::SharedState::new();
    axum::serve(listener, router::service_with_state(state)).await?;
    Ok(())
}
