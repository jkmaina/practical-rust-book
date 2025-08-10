// Basic async/await example demonstrating Rust's asynchronous programming
// async functions return futures that must be executed by a runtime
// block_on() runs the future to completion and returns the result

use futures::executor::block_on;

// Async function returns a future
async fn hello_world() -> String {
    "Hello, world!".to_string()
}

fn main() {
    println!("Executing async function...");
    
    // Execute the async function using futures executor
    let result = block_on(hello_world());
    println!("Result: {}", result);
}
