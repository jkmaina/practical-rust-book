// Barrier synchronization example for coordinating multiple threads
// Barrier blocks threads until a specified number reach the synchronization point
// Useful for parallel algorithms where threads must complete phases together
// Demonstrates thread coordination and synchronization patterns

use std::sync::{Arc, Barrier};
use std::thread;
fn main() {
    // Create a barrier that waits for 3 threads
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("Thread {} is waiting at the barrier", i);
            barrier.wait();
            println!("Thread {} has passed the barrier", i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
