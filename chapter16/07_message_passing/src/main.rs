// Demonstrates message passing between threads using channels
// Shows mpsc (multiple producer, single consumer) channel communication
// Channels provide a safe way to transfer data between threads without shared memory

use std::sync::mpsc; // mpsc = multiple producer, single consumer
use std::thread;

fn main() {
    println!("=== Message Passing Example ===");
    
    // Create a channel that returns a transmitter (tx) and receiver (rx)
    // tx = transmitter (sender), rx = receiver
    // Multiple transmitters can exist, but only one receiver
    let (tx, rx) = mpsc::channel();
    println!("Channel created: tx (transmitter) and rx (receiver)");
    
    // Spawn a thread that will send a message
    // The transmitter is moved into the spawned thread
    thread::spawn(move || {
        println!("Spawned thread: preparing to send message");
        
        // Send a message through the channel
        // send() transfers ownership of the data to the receiver
        // unwrap() panics if the receiver has been dropped
        tx.send("Hello from the spawned thread!").unwrap();
        println!("Spawned thread: message sent successfully");
        
        // tx is dropped when this thread ends
    });
    
    println!("Main thread: waiting to receive message...");
    
    // Receive the message in the main thread
    // recv() blocks until a message is available
    // unwrap() panics if all transmitters have been dropped and no more messages
    let received = rx.recv().unwrap();
    println!("Main thread received: {}", received);
    
    println!("\n=== Channel Communication Benefits ===");
    println!("1. No shared memory - data is transferred, not shared");
    println!("2. Thread-safe by design - no data races possible");
    println!("3. Ownership transfer - prevents use-after-move errors");
    println!("4. Blocking receive - automatic synchronization");
    println!("5. Type-safe - channels are generic over message type");
}
