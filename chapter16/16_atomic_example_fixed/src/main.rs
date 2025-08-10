// Corrected atomic operations example using Arc for shared ownership
// Demonstrates proper sharing of atomic values across multiple threads
// Arc<AtomicT> pattern combines reference counting with lock-free operations
// Shows how to fix the previous example that had ownership issues

use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
fn main() {
    // Create an atomic integer inside an Arc
    let counter = Arc::new(AtomicI32::new(0));
    
    // Spawn threads to increment the counter
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                // Atomically increment the counter
                counter.fetch_add(1, Ordering::SeqCst);
            })
        })
        .collect();
    
    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Print the final value
    println!("Final count: {}", counter.load(Ordering::SeqCst));
}
