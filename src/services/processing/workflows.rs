use super::tasks::*;
use crate::prelude::*;
use async_trait::async_trait;

#[async_trait]
pub trait Workflow {
    async fn process(&self) -> Result<String>;
}

pub struct WorkflowImpl {}

impl WorkflowImpl {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Workflow for WorkflowImpl {
    async fn process(&self) -> Result<String> {
        let task_1 = tokio::spawn(async { task_1().await });
        let task_2 = tokio::spawn(async { task_2().await });
        let result_1 = task_1.await?;
        let result_2 = task_2.await?;
        let result = format!("{} {}", result_1, result_2);
        Ok(result)
    }
}
