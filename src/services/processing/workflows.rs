use super::tasks::*;
use crate::{prelude::*, services::authentication::repositories::AccountRepository};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};

#[async_trait]
pub trait Workflow {
    async fn process(&self) -> Result<String>;
}

pub struct WorkflowImpl {
    account_repository: Arc<Mutex<dyn AccountRepository>>,
}

impl WorkflowImpl {
    pub fn new(account_repository: Arc<Mutex<dyn AccountRepository>>) -> Self {
        Self { account_repository }
    }
}

#[async_trait]
impl Workflow for WorkflowImpl {
    async fn process(&self) -> Result<String> {
        let id = "foo".to_string();
        let account = self.account_repository.lock().unwrap().read(id)?;
        let task_1 = tokio::spawn(async { task_1().await });
        let task_2 = tokio::spawn(async { task_2().await });
        let result_1 = task_1.await?;
        let result_2 = task_2.await?;
        let result = format!("{} {} from {:?}", result_1, result_2, account.id);
        Ok(result)
    }
}
