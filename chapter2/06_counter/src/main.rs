// This is a simple Rust program that simulates a counter for events.
// It increments a counter each time an event occurs and prints the count to the console.
// This is useful for tracking how many times a specific event has happened in a program.
fn main() {
    let mut counter = 0;
    
    // Simulate some process that counts events
    counter += 1; // First event occurred
    println!("Events processed: {}", counter);
    
    counter += 1; // Second event occurred
    println!("Events processed: {}", counter);
    
    counter += 1; // Third event occurred
    println!("Events processed: {}", counter);
}
