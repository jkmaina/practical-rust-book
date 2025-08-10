// Tokio concurrent execution example demonstrating parallel async tasks
// Shows how tokio::join! runs multiple async functions concurrently
// Tasks execute simultaneously rather than sequentially
// Demonstrates the power of async/await for concurrent programming

use tokio::time::{sleep, Duration};
async fn task1() {
    println!("Task 1: Starting");
    sleep(Duration::from_secs(2)).await;
    println!("Task 1: Finished");
}
async fn task2() {
    println!("Task 2: Starting");
    sleep(Duration::from_secs(1)).await;
    println!("Task 2: Finished");
}
#[tokio::main]
async fn main() {
    println!("Main: Starting");
    
    // Run tasks concurrently
    tokio::join!(task1(), task2());
    
    println!("Main: Finished");
} 
