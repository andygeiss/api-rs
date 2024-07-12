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
        let account = Account {
            id: "".to_string(),
            hash: "".to_string(),
        };
        Ok(account)
    }
    fn read(&self, id: String) -> Result<Account> {
        let account = Account {
            id: "".to_string(),
            hash: "".to_string(),
        };
        Ok(account)
    }
    fn update(&self, account: Account, password: String) -> Result<Account> {
        let account = Account {
            id: account.id,
            hash: "".to_string(),
        };
        Ok(account)
    }
    fn delete(&self, account: crate::services::authentication::entities::Account) -> Result<()> {
        Ok(())
    }
}
