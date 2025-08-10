// Basic thread creation and management in Rust
// Demonstrates spawning threads, concurrent execution, and thread synchronization
// Shows the difference between main thread and spawned thread execution

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Basic Thread Example ===");
    println!("Main thread starting...");
    
    // Create a new thread using thread::spawn()
    // The closure is moved to the new thread and executed there
    let handle = thread::spawn(|| {
        println!("Hello from the spawned thread!");
        println!("Spawned thread ID: {:?}", thread::current().id());
        
        // Sleep for 10 seconds to simulate work
        // This doesn't block the main thread
        thread::sleep(Duration::from_secs(10));
        
        println!("Spawned thread is done!");
    });
    
    // Main thread continues execution immediately after spawning
    // This runs concurrently with the spawned thread
    println!("Hello from the main thread!");
    println!("Main thread ID: {:?}", thread::current().id());
    
    // Simulate some work in the main thread
    thread::sleep(Duration::from_millis(5000));
    println!("Main thread did some work...");
    
    // Wait for the spawned thread to finish before continuing
    // join() blocks the main thread until the spawned thread completes
    // unwrap() panics if the spawned thread panicked
    println!("Main thread waiting for spawned thread to finish...");
    handle.join().unwrap();
    
    println!("Main thread is done!");
    
    println!("\n=== Key Concepts ===");
    println!("- thread::spawn() creates a new OS thread");
    println!("- Threads run concurrently (at the same time)");
    println!("- join() waits for a thread to complete");
    println!("- Main thread ending would terminate all spawned threads");
    println!("- Each thread has a unique ID");
}