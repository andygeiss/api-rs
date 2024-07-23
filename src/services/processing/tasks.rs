use std::time::Duration;

pub async fn task_1() -> &'static str {
    tokio::time::sleep(Duration::from_millis(500)).await;
    "Hello"
}

pub async fn task_2() -> &'static str {
    tokio::time::sleep(Duration::from_millis(250)).await;
    "World"
}
