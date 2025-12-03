// Enable strict linting - all clippy warnings become compilation errors
#![deny(clippy::all)]
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    tokio::select! {
        result = slow_task() => {
            println!("Slow task: {}", result);
        }
        result = fast_task() => {
            println!("Fast task: {}", result);
        }
    }
}

async fn slow_task() -> i32 {
    sleep(Duration::from_secs(3)).await;
    100
}

async fn fast_task() -> i32 {
    sleep(Duration::from_secs(1)).await;
    42
}