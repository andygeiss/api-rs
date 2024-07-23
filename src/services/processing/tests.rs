use super::workflows::{Workflow, WorkflowImpl};
use crate::{
    prelude::*, repositories::account_file::AccountFileRepository,
    services::authentication::repositories::AccountRepository,
};

#[tokio::test]
async fn test_tasks() {
    let path = "./testdata/test_tasks_accounts.json".to_string();
    let repo = thread_safe(AccountFileRepository::new(path));
    repo.lock()
        .unwrap()
        .create("foo".to_string(), "bar".to_string())
        .unwrap();
    let workflow = thread_safe(WorkflowImpl::new(repo));
    let result = workflow.lock().unwrap().process().await;
    assert_eq!(result.is_ok(), true);
    assert_eq!(result.unwrap(), "Hello World from \"foo\"".to_string());
}
