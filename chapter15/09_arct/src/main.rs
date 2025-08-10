// Arc<T>: Atomically Reference Counted smart pointer for thread-safe shared ownership
// Unlike Rc<T>, Arc<T> is safe to use across threads due to atomic reference counting
// Enables multiple threads to share read-only access to the same data

use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Basic Arc Usage ===");
    
    // Create a shared integer wrapped in Arc
    let shared_data = Arc::new(5);
    println!("Initial reference count: {}", Arc::strong_count(&shared_data));
    
    // Create 5 threads that each print the shared data
    let mut handles = vec![];
    
    for i in 0..5 {
        // Clone the Arc to increase the reference count (cheap operation)
        // This creates a new pointer to the same data, not a copy of the data
        let data = Arc::clone(&shared_data);
        println!("After cloning for thread {}: count = {}", i, Arc::strong_count(&shared_data));
        
        // Spawn a thread - move keyword transfers ownership of 'data' to the thread
        let handle = thread::spawn(move || {
            // Each thread can safely access the shared data
            println!("Thread {}: shared data = {}", i, data);
            
            // Simulate some work
            thread::sleep(Duration::from_millis(100));
            
            // When this closure ends, the Arc clone is dropped
            println!("Thread {} finishing", i);
        });
        
        handles.push(handle);
    }
    
    println!("All threads spawned. Reference count: {}", Arc::strong_count(&shared_data));
    
    // Wait for all threads to finish
    for (i, handle) in handles.into_iter().enumerate() {
        handle.join().unwrap();
        println!("Thread {} joined. Remaining count: {}", i, Arc::strong_count(&shared_data));
    }
    
    println!("Final reference count: {}", Arc::strong_count(&shared_data));
    
    println!("\n=== Arc vs Rc Comparison ===");
    println!("Arc<T>:");
    println!("  - Thread-safe (atomic reference counting)");
    println!("  - Slightly more overhead due to atomic operations");
    println!("  - Can be shared across threads");
    println!("Rc<T>:");
    println!("  - Single-threaded only");
    println!("  - Lower overhead (non-atomic reference counting)");
    println!("  - Cannot be sent between threads");
    
    // The shared_data Arc is dropped here when main ends
    println!("\nMain function ending - Arc will be dropped");
}