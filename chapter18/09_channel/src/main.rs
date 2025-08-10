// Async channels example demonstrating message passing between tasks
// Uses tokio::sync::mpsc for multi-producer, single-consumer communication
// Shows how to send data from one async task to another asynchronously
// Channels provide a safe way to share data between concurrent tasks

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
   
    // Spawn a task that sends values to the channel
    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
        }
    });
   
    // Receive values from the channel as a stream
    while let Some(value) = rx.recv().await {
        println!("Got value: {}", value);
    }
}