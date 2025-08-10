// This example demonstrates proper thread synchronization using join()
// The main thread waits for the spawned thread to complete before exiting
// All output from both threads will be displayed
// Contrasts with the no_join_example where spawned thread was killed early

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Join Example - Proper Thread Synchronization ===");
    
    // Spawn a thread that prints numbers 1-9
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        println!("Spawned thread completed all iterations!");
    });
    
    // Main thread prints numbers 1-4
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    println!("Main thread finished its loop, now waiting for spawned thread...");
    
    // CRITICAL: join() blocks the main thread until spawned thread completes
    // This ensures the spawned thread finishes all its work before program exits
    // unwrap() will panic if the spawned thread panicked
    handle.join().unwrap();
    
    println!("Spawned thread has finished - main thread can now exit safely!");
    println!("Program terminating gracefully with all threads completed.");
}

// Key differences from no_join_example:
// 1. join() blocks main thread until spawned thread completes
// 2. All spawned thread output (1-9) will be displayed
// 3. Program exits only after both threads finish their work
// 4. No threads are forcibly killed - proper cleanup occurs
