// FuturesUnordered example demonstrating concurrent task execution
// Processes multiple futures concurrently and yields results as they complete
// Results arrive in completion order, not submission order
// Useful for handling multiple async operations with different completion times

use futures::stream::{FuturesUnordered, StreamExt};
use tokio::time::{sleep, Duration};
#[tokio::main]
async fn main() {
    let mut tasks = FuturesUnordered::new();
    
    // Add some tasks
    for i in 1..=5 {
        tasks.push(async move {
            let delay = Duration::from_secs(6 - i);
            sleep(delay).await;
            i
        });
    }
    
    // Process tasks as they complete
    while let Some(result) = tasks.next().await {
        println!("Task completed with result: {}", result);
    }
}
