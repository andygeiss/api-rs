use super::entities::Account;
use crate::prelude::*;

#[async_trait]
#[automock]
pub trait AccountRepository: Send + Sync {
    fn create(&self, id: String, password: String) -> Result<Account>;
    fn read(&self, id: String) -> Result<Account>;
    fn update(&self, id: String, password: String) -> Result<()>;
    fn delete(&self, id: String) -> Result<()>;
}
