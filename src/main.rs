use crate::prelude::*;

mod error;
mod prelude;
mod router;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("🚀 {name} version {version} started successfully.");
    tracing_subscriber::fmt().compact().init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
    axum::serve(listener, router::service()).await?;
    Ok(())
}
