use super::tasks::*;

#[tokio::test]
async fn test_tasks() {
    let t1 = tokio::spawn(async { task_1().await });
    let t2 = tokio::spawn(async { task_2().await });
    let r1 = t1.await.unwrap();
    let r2 = t2.await.unwrap();
    assert_eq!(r1, "Task 1 Result");
    assert_eq!(r2, "Task 2 Result");
}
