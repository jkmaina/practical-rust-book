// Demonstrates the 'move' keyword with thread closures
// Shows how ownership is transferred from main thread to spawned thread
// Explains why 'move' is necessary for thread safety with captured variables

use std::thread;

fn main() {
    println!("=== Move Closure Example ===");
    
    // Create a vector in the main thread
    let v = vec![1, 2, 3];
    println!("Vector created in main thread: {:?}", v);
    
    // The 'move' keyword forces the closure to take ownership of captured variables
    // Without 'move', the closure would try to borrow 'v', which is unsafe across threads
    let handle = thread::spawn(move || {
        // 'v' is now owned by this closure and moved to the spawned thread
        println!("Vector accessed in spawned thread: {:?}", v);
        
        // The spawned thread can safely use 'v' because it owns it
        println!("Vector length in spawned thread: {}", v.len());
    });
    
    // IMPORTANT: We can't use 'v' here anymore because ownership was moved
    // Uncommenting the line below would cause a compile error:
    // println!("Trying to use v in main thread: {:?}", v); // ERROR!
    
    println!("Main thread continuing without access to 'v'...");
    
    // Wait for the spawned thread to complete
    handle.join().unwrap();
    
    println!("\n=== Why 'move' is Required ===");
    println!("1. Threads have independent lifetimes");
    println!("2. Main thread might end before spawned thread");
    println!("3. Borrowing across threads is unsafe (data might be dropped)");
    println!("4. 'move' transfers ownership, ensuring data lives as long as the thread");
    println!("5. Rust's ownership system prevents data races at compile time");
}
