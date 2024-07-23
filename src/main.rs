use crate::prelude::*;
use repositories::account_file::AccountFileRepository;
use services::processing::workflows::{Workflow, WorkflowImpl};

mod cli;
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
    let account_path = "./data/accounts.json".to_string();
    let account_repo = thread_safe(AccountFileRepository::new(account_path.clone()));
    // Client mode
    // cargo run create-account --id foo --password bar
    if cli::has_client_result(account_repo.clone())? {
        return Ok(());
    };
    // Async processing
    let workflow = WorkflowImpl::new();
    let result = workflow.process().await?;
    println!("📦 '{result}' after async processing.");
    // Server mode
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    println!("🚀 {name} version {version} started successfully.");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    let state = state::SharedState::new(account_repo);
    axum::serve(listener, router::service_with_state(state)).await?;
    Ok(())
}
