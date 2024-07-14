use crate::{
    prelude::*,
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

impl AccountRepository for AccountFileRepository {
    fn create(&self, id: String, password: String) -> Result<Account> {
        let mut accounts: Vec<Account> = vec![];
        // Get accounts from file
        if let Ok(from_file) = read_accounts(self.path.clone()) {
            accounts = from_file;
        }
        // Do nothing if the account id already exists
        if let Some(account) = accounts.iter().find(|a| a.id == id) {
            return Ok(account.clone());
        }
        // Or create a new account, save and return it
        let account = Account { id, hash: password };
        accounts.push(account.clone());
        write_accounts(self.path.clone(), accounts)?;
        Ok(account)
    }
    fn read(&self, id: String) -> Result<Account> {
        // Read the accounts from a file
        let accounts: Vec<Account> = read_accounts(self.path.clone())?;
        // Find a specific account by id and return it
        if let Some(account) = accounts.iter().find(|a| a.id == id) {
            return Ok(account.clone());
        }
        // Or return an error if not exists
        Err(Error::Generic(format!("account with id {id} not found!")))
    }
    fn update(&self, account: Account, password: String) -> Result<Account> {
        // Read the accounts from a file
        let mut accounts: Vec<Account> = read_accounts(self.path.clone())?;
        // Update a specific account by id and return it
        let mut changed = false;
        if let Some(account) = accounts.iter_mut().find(|a| a.id == account.id) {
            account.hash = password;
            changed = true;
        }
        if changed {
            write_accounts(self.path.clone(), accounts)?;
            return Ok(account.clone());
        }
        // Or return an error if not exists
        Err(Error::Generic(format!(
            "account with id {} not found!",
            account.id
        )))
    }
    fn delete(&self, account: Account) -> Result<()> {
        // Read the accounts from a file
        let mut accounts: Vec<Account> = read_accounts(self.path.clone())?;
        // Remove a specific account by id
        accounts.retain(|a| a.id != account.id);
        write_accounts(self.path.clone(), accounts)?;
        Ok(())
    }
}

fn read_accounts(path: String) -> Result<Vec<Account>> {
    let contents = std::fs::read_to_string(path)?;
    let accounts: Vec<Account> = serde_json::from_str(contents.as_str())?;
    Ok(accounts)
}

fn write_accounts(path: String, accounts: Vec<Account>) -> Result<()> {
    let contents = serde_json::to_string(&accounts)?;
    std::fs::write(path, contents)?;
    Ok(())
}
