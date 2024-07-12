use crate::prelude::*;

use super::entities::Account;

pub trait AccountRepository: Send + Sync {
    fn create(&self, id: String, password: String) -> Result<Account>;
    fn read(&self, id: String) -> Result<Account>;
    fn update(&self, account: Account, password: String) -> Result<Account>;
    fn delete(&self, account: Account) -> Result<()>;
}
