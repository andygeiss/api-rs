use super::{entities::Account, repositories::AccountRepository};
use std::sync::{Arc, Mutex};

pub trait AccountService: Send + Sync {
    fn is_password_correct(&self, account: Account, password: String) -> bool;
}

pub struct AccountServiceImpl {
    account_repository: Arc<Mutex<dyn AccountRepository>>,
}

impl AccountServiceImpl {
    pub fn new(account_repository: Arc<Mutex<dyn AccountRepository>>) -> Self {
        Self { account_repository }
    }
}

impl AccountService for AccountServiceImpl {
    fn is_password_correct(&self, account: Account, password: String) -> bool {
        true
    }
}
