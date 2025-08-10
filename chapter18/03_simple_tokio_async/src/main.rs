// Tokio async runtime example demonstrating async main and time operations
// Tokio runtime is the most popular async runtime for Rust
// #[tokio::main] macro sets up the async runtime automatically
// Shows non-blocking sleep operation using tokio::time::sleep

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Start");
    
    sleep(Duration::from_secs(1)).await;
    
    println!("End");
} 
