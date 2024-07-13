use crate::services::authentication::{repositories::AccountRepository, service::AccountService};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SharedState {
    pub account_repository: Arc<Mutex<dyn AccountRepository>>,
    pub account_service: Arc<Mutex<dyn AccountService>>,
}

impl SharedState {
    pub fn new(
        account_repository: Arc<Mutex<dyn AccountRepository>>,
        account_service: Arc<Mutex<dyn AccountService>>,
    ) -> Self {
        Self {
            account_repository,
            account_service,
        }
    }
}
