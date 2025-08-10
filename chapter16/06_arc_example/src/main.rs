// Demonstrates Arc (Atomically Reference Counted) for thread-safe shared ownership
// Shows how to share data between threads without transferring ownership
// Arc enables multiple threads to safely access the same data concurrently

use std::sync::Arc;
use std::thread;

fn main() {
    println!("=== Arc Example - Shared Data Across Threads ===");
    
    // Create an Arc containing the value 5
    // Arc provides thread-safe reference counting
    let arc = Arc::new(5);
    println!("Original Arc created with value: {}", arc);
    println!("Initial reference count: {}", Arc::strong_count(&arc));
    
    // Clone the Arc to create another reference to the same data
    // This is a cheap operation - only the reference count is incremented
    // The actual data (5) is NOT copied
    let arc_clone = Arc::clone(&arc);
    println!("After cloning: reference count = {}", Arc::strong_count(&arc));
    
    // Move the cloned Arc to a new thread
    // The spawned thread now has its own reference to the shared data
    let handle = thread::spawn(move || {
        // Both the main thread and this spawned thread can access the same data
        println!("Arc value in spawned thread: {}", arc_clone);
        println!("Reference count from spawned thread: {}", Arc::strong_count(&arc_clone));
        
        // The arc_clone will be dropped when this thread ends
    });
    
    // Main thread can still access the original Arc
    println!("Arc value in main thread: {}", arc);
    
    // Wait for the spawned thread to complete
    handle.join().unwrap();
    
    // After the spawned thread ends, reference count decreases
    println!("Final reference count: {}", Arc::strong_count(&arc));
    
    println!("\n=== Key Arc Concepts ===");
    println!("1. Arc enables shared ownership across threads");
    println!("2. Arc::clone() is cheap - only increments reference count");
    println!("3. Data is freed when the last Arc reference is dropped");
    println!("4. Arc provides thread-safe reference counting (atomic operations)");
    println!("5. Arc is read-only - use Arc<Mutex<T>> for mutable shared data");
}
