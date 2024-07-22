use std::time::Duration;

pub async fn task_1() -> &'static str {
    println!("Task 1 Start");
    tokio::time::sleep(Duration::from_millis(500)).await;
    println!("Task 1 End");
    "Task 1 Result"
}

pub async fn task_2() -> &'static str {
    println!("Task 2 Start");
    tokio::time::sleep(Duration::from_millis(250)).await;
    println!("Task 2 End");
    "Task 2 Result"
}
