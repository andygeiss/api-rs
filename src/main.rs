use crate::prelude::*;
use repositories::account_file::AccountFileRepository;
use services::authentication::service::AccountServiceImpl;
use std::sync::{Arc, Mutex};

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
    tracing_subscriber::fmt().compact().init();
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("🚀 {name} version {version} started successfully.");

    let afp = "./data/accounts.json".to_string();
    let afr = Arc::new(Mutex::new(AccountFileRepository::new(afp.clone())));
    let asi = Arc::new(Mutex::new(AccountServiceImpl::new(afr.clone())));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    let state = state::SharedState::new(afr, asi);
    axum::serve(listener, router::service_with_state(state)).await?;
    Ok(())
}
