// RwLock (Read-Write Lock) example for optimized concurrent data access
// Allows multiple concurrent readers OR one exclusive writer (but not both)
// More efficient than Mutex when reads are more frequent than writes
// Demonstrates the reader-writer pattern for shared data structures

use std::sync::{Arc, RwLock};
use std::thread;
fn main() {
    // Create a RwLock inside an Arc
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    
    // Clone the Arc for the reader threads
    let reader_data = Arc::clone(&data);
    
    // Spawn reader threads
    let reader_handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&reader_data);
            thread::spawn(move || {
                let read_guard = data.read().unwrap();
                println!("Reader {}: {:?}", i, *read_guard);
            })
        })
        .collect();
    
    // Spawn a writer thread
    let writer_handle = thread::spawn(move || {
        let mut write_guard = data.write().unwrap();
        write_guard.push(4);
        println!("Writer: {:?}", *write_guard);
    });
    
    // Wait for all threads to finish
    for handle in reader_handles {
        handle.join().unwrap();
    }
    writer_handle.join().unwrap();
}
