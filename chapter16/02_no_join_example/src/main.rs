// This example demonstrates what happens when main thread exits without join()
// The main thread finishes first and terminates the entire program
// Spawned threads are forcibly killed when the main thread exits
// This shows why join() is important for thread synchronization

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== No Join Example - Demonstrating Thread Termination ===");
    
    // Spawn a thread that tries to print numbers 1-9
    let _handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("Spawned thread completed all iterations!"); // This may never print
    });
    
    // Main thread only prints numbers 1-4, then exits
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    println!("Main thread finished - program will terminate!");
    
    // CRITICAL: No handle.join() here!
    // When main thread exits, the entire process terminates
    // The spawned thread is forcibly killed, even if it hasn't finished
    // This is why you see spawned thread output stop abruptly
    
    // Uncomment the line below to see the difference:
    // handle.join().unwrap();
}

// Why main thread doesn't reach 5:
// 1. Main thread loop runs from 1..5 (which is 1,2,3,4 - excludes 5)
// 2. After printing "hi number 4", main thread exits
// 3. Program terminates, killing the spawned thread mid-execution
// 4. Spawned thread may be in the middle of printing when it's killed
