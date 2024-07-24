use crate::prelude::*;
use crate::{
    repositories::account_file::AccountFileRepository,
    services::authentication::repositories::AccountRepository,
};
use clap::{Parser, ValueEnum};

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

async fn handle_client_mode() -> Result<()> {
    let args = ClientArgs::parse();
    let path = "./data/accounts.json".to_string();
    let repo = AccountFileRepository::new(path.clone());
    match args.client_command {
        ClientCommand::CreateAccount => {
            if let Some(password) = args.password {
                let account = repo.create(args.id, password).await?;
                println!("Creating Account ... got {:?}", account);
            }
        }
        ClientCommand::ReadAccout => {
            let account = repo.read(args.id).await?;
            println!("Reading Account ... got {:?}", account);
        }
        ClientCommand::UpdateAccount => {
            if let Some(password) = args.password {
                repo.update(args.id, password).await?;
                println!("Updating Account done.");
            }
        }
        ClientCommand::DeleteAccount => {
            repo.delete(args.id).await?;
            println!("Deleting Account done.");
        }
    }
    Ok(())
}

pub async fn is_client_mode() -> Result<bool> {
    let args = Args::parse();
    match args.mode {
        Some(Mode::Client) => {
            handle_client_mode().await?;
            return Ok(true);
        }
        Some(Mode::Server) => Ok(false),
        None => Ok(false),
    }
}
