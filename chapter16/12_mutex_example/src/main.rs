// Mutex (Mutual Exclusion) example for thread-safe mutable data access
// Demonstrates basic mutex operations: locking, modifying, and automatic unlocking
// Shows how Rust prevents data races at compile time with mutex guards

use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    println!("=== Basic Mutex Example ===");
    
    // Create a mutex containing an integer
    // Mutex wraps data and ensures only one thread can access it at a time
    let m = Mutex::new(5);
    println!("Created mutex with initial value: 5");
    
    {
        // Acquire the lock and get a mutable reference to the data
        // lock() returns a MutexGuard that implements Deref and DerefMut
        let mut num = m.lock().unwrap();
        println!("Lock acquired, current value: {}", *num);
        
        // Modify the data through the mutex guard
        *num = 6;
        println!("Value modified to: {}", *num);
        
        // Lock is automatically released when num goes out of scope
        // This is RAII (Resource Acquisition Is Initialization)
    } // <- Lock released here automatically
    
    println!("Lock released, mutex contains: {:?}", m);
    
    println!("\n=== Multi-threaded Mutex Example ===");
    
    // Create a mutex wrapped in Arc for sharing between threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    // Spawn 10 threads that each increment the counter
    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Each thread acquires the lock, increments, and releases
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter to {}", i, *num);
            
            // Simulate some work while holding the lock
            thread::sleep(Duration::from_millis(10));
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Check final value
    println!("Final counter value: {}", *counter.lock().unwrap());
    
    println!("\n=== Mutex Key Concepts ===");
    println!("1. Mutex provides mutual exclusion - only one thread can access data");
    println!("2. lock() blocks until the mutex is available");
    println!("3. MutexGuard automatically releases lock when dropped (RAII)");
    println!("4. Prevents data races at compile time");
    println!("5. Use Arc<Mutex<T>> to share mutable data between threads");
    println!("6. Deadlock is possible if not careful with lock ordering");
}
