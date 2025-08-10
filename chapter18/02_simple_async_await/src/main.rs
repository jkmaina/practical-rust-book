// Simple async/await example showing .await usage
// Demonstrates how .await is used inside async functions to wait for futures

use futures::executor::block_on;

// Async function returns a future
async fn hello_world() -> String {
    "Hello, world!".to_string()
}

// Async function that uses .await to call another async function
// .await is only allowed inside async functions and blocks
async fn greet() -> String {
    let message = hello_world().await;  // .await waits for the future to complete
    format!("Greeting: {}", message)
}

fn main() {
    // Execute async function using block_on
    let result = block_on(greet());
    println!("{}", result);
}
