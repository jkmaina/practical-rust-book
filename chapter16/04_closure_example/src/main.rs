// This example demonstrates a COMPILATION ERROR with thread closures
// Shows why borrowing across threads is unsafe and won't compile
// Illustrates the need for the 'move' keyword in thread closures

use std::thread;

fn main() {
    println!("=== Closure Borrowing Example (WILL NOT COMPILE) ===");
    
    // Create a vector in the main thread
    let v = vec![1, 2, 3];
    println!("Vector created in main thread: {:?}", v);
    
    // COMPILATION ERROR: This closure tries to borrow 'v' from the main thread
    // The compiler cannot guarantee that 'v' will live long enough for the spawned thread
    let handle = thread::spawn(|| {
        // ERROR: 'v' is borrowed here, but the main thread might drop it
        // before this spawned thread finishes executing
        println!("Here's a vector: {:?}", v); // COMPILE ERROR!
    });
    
    // The main thread could potentially end here, dropping 'v'
    // while the spawned thread is still trying to use it
    
    handle.join().unwrap();
    
    println!("\n=== Why This Fails ===");
    println!("1. Closure tries to borrow 'v' from main thread");
    println!("2. Spawned thread lifetime is independent of main thread");
    println!("3. Main thread could drop 'v' while spawned thread uses it");
    println!("4. This would create a dangling reference (unsafe!)");
    println!("5. Rust prevents this at compile time");
    println!("\nSolution: Use 'move' keyword to transfer ownership");
} 
