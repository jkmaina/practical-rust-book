use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    let resource_a = Arc::new(Mutex::new(0));
    let resource_b = Arc::new(Mutex::new(0));
    
    let resource_a_clone = Arc::clone(&resource_a);
    let resource_b_clone = Arc::clone(&resource_b);
    
    // Thread 1: Lock A, then lock B
    let thread_1 = thread::spawn(move || {
        let _a = resource_a_clone.lock().unwrap();
        println!("Thread 1: Locked resource A");
        
        // Sleep to increase the chance of a deadlock
        thread::sleep(Duration::from_millis(100));
        
        let _b = resource_b_clone.lock().unwrap();
        println!("Thread 1: Locked resource B");
    });
    
    // Thread 2: Lock B, then lock A
    let thread_2 = thread::spawn(move || {
        let _b = resource_b.lock().unwrap();
        println!("Thread 2: Locked resource B");
        
        // Sleep to increase the chance of a deadlock
        thread::sleep(Duration::from_millis(100));
        
        let _a = resource_a.lock().unwrap();
        println!("Thread 2: Locked resource A");
    });
    
    thread_1.join().unwrap();
    thread_2.join().unwrap();
}
