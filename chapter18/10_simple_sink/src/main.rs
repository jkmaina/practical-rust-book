use tokio::sync::mpsc;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(32);
    
    // Send values to the channel as a sink
    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
        }
    });
    
    // Receive values from the channel
    while let Some(value) = rx.recv().await {
        println!("Got value: {}", value);
    }
    
    Ok(())
}
