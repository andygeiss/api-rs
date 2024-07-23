use super::workflows::{Workflow, WorkflowImpl};
use crate::prelude::*;

#[tokio::test]
async fn test_tasks() {
    let workflow = thread_safe(WorkflowImpl::new());
    let result = workflow.lock().unwrap().process().await;
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), "Hello World".to_string());
}
