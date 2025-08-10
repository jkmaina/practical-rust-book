// Demonstrates sending multiple messages through a channel
// Shows how channels can handle streams of data between threads
// Illustrates the iterator pattern with channel receivers

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Multiple Messages Example ===");
    
    // Create a channel for string messages
    let (tx, rx) = mpsc::channel();
    
    // Spawn a thread that sends multiple messages
    thread::spawn(move || {
        // Create a vector of messages to send
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        println!("Spawned thread: sending {} messages...", vals.len());
        
        // Send each message with a delay
        for (i, val) in vals.into_iter().enumerate() {
            println!("Spawned thread: sending message {}: '{}'", i + 1, val);
            
            // send() transfers ownership of val to the receiver
            tx.send(val).unwrap();
            
            // Sleep to simulate work and show asynchronous nature
            thread::sleep(Duration::from_secs(1));
        }
        
        println!("Spawned thread: all messages sent, transmitter will be dropped");
        // tx is automatically dropped here when the thread ends
    });
    
    println!("Main thread: waiting for messages...");
    
    // Iterate over the receiver to get all messages
    // This loop continues until all transmitters are dropped
    // rx acts as an iterator that blocks on each iteration until a message arrives
    for received in rx {
        println!("Main thread got: '{}'", received);
    }
    
    println!("Main thread: no more messages (all transmitters dropped)");
    
    println!("\n=== Key Concepts ===");
    println!("1. Channels can send multiple messages sequentially");
    println!("2. rx can be used as an iterator with 'for' loops");
    println!("3. Iterator ends when all transmitters are dropped");
    println!("4. Each recv() blocks until a message is available");
    println!("5. Messages arrive in the order they were sent (FIFO)");
}
