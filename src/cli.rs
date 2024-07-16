use crate::prelude::*;
use crate::services::authentication::repositories::AccountRepository;
use clap::{Parser, ValueEnum};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum ClientCommand {
    CreateAccount,
    ReadAccout,
    UpdateAccount,
    DeleteAccount,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(value_enum)]
    pub client_command: Option<ClientCommand>,
    #[arg(short, long)]
    pub id: String,
    #[arg(short, long)]
    pub password: Option<String>,
}

pub fn handle_client_mode(repo: Arc<Mutex<dyn AccountRepository>>) -> Result<()> {
    let args = Args::parse();
    if let Some(cmd) = args.client_command {
        match cmd {
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
    }
    Ok(())
}
