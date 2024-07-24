use super::entities::Account;
use crate::prelude::*;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn create(&self, id: String, password: String) -> Result<Account>;
    async fn read(&self, id: String) -> Result<Account>;
    async fn update(&self, id: String, password: String) -> Result<()>;
    async fn delete(&self, id: String) -> Result<()>;
}
