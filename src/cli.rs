use crate::prelude::*;
use crate::services::authentication::repositories::AccountRepository;
use clap::{Parser, ValueEnum};
use std::sync::{Arc, Mutex};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum)]
    pub mode: Option<Mode>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ClientArgs {
    #[arg(value_enum)]
    pub client_command: ClientCommand,
    #[arg(short, long)]
    pub id: String,
    #[arg(short, long)]
    pub password: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum ClientCommand {
    CreateAccount,
    ReadAccout,
    UpdateAccount,
    DeleteAccount,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum Mode {
    Client,
    Server,
}

fn handle_client_mode(repo: Arc<Mutex<dyn AccountRepository>>) -> Result<()> {
    let args = ClientArgs::parse();
    match args.client_command {
        ClientCommand::CreateAccount => {
            if let Some(password) = args.password {
                let account = repo.lock().unwrap().create(args.id, password)?;
                println!("Creating Account ... got {:?}", account);
            }
        }
        ClientCommand::ReadAccout => {
            let account = repo.lock().unwrap().read(args.id)?;
            println!("Reading Account ... got {:?}", account);
        }
        ClientCommand::UpdateAccount => {
            if let Some(password) = args.password {
                repo.lock().unwrap().update(args.id, password)?;
                println!("Updating Account done.");
            }
        }
        ClientCommand::DeleteAccount => {
            repo.lock().unwrap().delete(args.id)?;
            println!("Deleting Account done.");
        }
    }
    Ok(())
}

pub fn has_client_result(repo: Arc<Mutex<dyn AccountRepository>>) -> Result<bool> {
    let args = Args::parse();
    match args.mode {
        Some(Mode::Client) => {
            handle_client_mode(repo)?;
            return Ok(true);
        }
        Some(Mode::Server) => Ok(false),
        None => Ok(false),
    }
}
