use crate::{
    prelude::*,
    security,
    services::authentication::{entities::Account, repositories::AccountRepository},
};

pub struct AccountFileRepository {
    path: String,
}

impl AccountFileRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[async_trait]
impl AccountRepository for AccountFileRepository {
    async fn create(&self, id: String, password: String) -> Result<Account> {
        let mut accounts: Vec<Account> = vec![];
        // Get accounts from file
        if let Ok(from_file) = read_accounts(self.path.clone()).await {
            accounts = from_file;
        }
        // Do nothing if the account id already exists
        if let Some(account) = accounts.iter().find(|a| a.id == id) {
            return Ok(account.clone());
        }
        // Or create a new account, save and return it
        let hash = security::password::create_hash(password);
        let account = Account { id, hash };
        accounts.push(account.clone());
        write_accounts(self.path.clone(), accounts).await?;
        Ok(account)
    }
    async fn read(&self, id: String) -> Result<Account> {
        // Read the accounts from a file
        let accounts: Vec<Account> = read_accounts(self.path.clone()).await?;
        // Find a specific account by id and return it
        if let Some(account) = accounts.iter().find(|a| a.id == id) {
            return Ok(account.clone());
        }
        // Or return an error if not exists
        Err(Error::Generic(format!("account with id {id} not found!")))
    }
    async fn update(&self, id: String, password: String) -> Result<()> {
        // Read the accounts from a file
        let mut accounts: Vec<Account> = read_accounts(self.path.clone()).await?;
        // Update a specific account by id and return it
        if let Some(account) = accounts.iter_mut().find(|a| a.id == id) {
            let hash = security::password::create_hash(password);
            account.hash = hash;
        }
        write_accounts(self.path.clone(), accounts).await?;
        return Ok(());
    }
    async fn delete(&self, id: String) -> Result<()> {
        // Read the accounts from a file
        let mut accounts: Vec<Account> = read_accounts(self.path.clone()).await?;
        // Remove a specific account by id
        accounts.retain(|a| a.id != id);
        write_accounts(self.path.clone(), accounts).await?;
        Ok(())
    }
}

async fn read_accounts(path: String) -> Result<Vec<Account>> {
    let contents = std::fs::read_to_string(path)?;
    let accounts: Vec<Account> = serde_json::from_str(contents.as_str())?;
    Ok(accounts)
}

async fn write_accounts(path: String, accounts: Vec<Account>) -> Result<()> {
    let contents = serde_json::to_string(&accounts)?;
    std::fs::write(path, contents)?;
    Ok(())
}
