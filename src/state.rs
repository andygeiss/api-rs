use crate::services::authentication::repositories::AccountRepository;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SharedState {
    pub account_repository: Arc<Mutex<dyn AccountRepository>>,
}

impl SharedState {
    pub fn new(account_repository: Arc<Mutex<dyn AccountRepository>>) -> Self {
        Self { account_repository }
    }
}
