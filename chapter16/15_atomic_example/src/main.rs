// Atomic operations example for lock-free concurrent programming
// Atomics provide thread-safe operations without the overhead of mutexes
// Useful for simple operations like counters, flags, and basic synchronization
// Demonstrates SeqCst (Sequential Consistency) ordering for strongest guarantees

use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;
fn main() {
    // Create an atomic integer
    let counter = AtomicI32::new(0);
    
    // Spawn threads to increment the counter
    let handles: Vec<_> = (0..10)
        .map(|_| {
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
