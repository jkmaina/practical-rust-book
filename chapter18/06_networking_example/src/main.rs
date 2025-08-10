// Async networking example demonstrating TCP server with Tokio
// Shows concurrent client handling using tokio::spawn for each connection
// Uses async I/O operations that don't block the thread
// Demonstrates real-world async programming for network services

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    println!("Server listening on 127.0.0.1:8080");
    
    loop {
        let (socket, _) = listener.accept().await?;
        
        // Handle each client in a separate task
        tokio::spawn(async move {
            handle_client(socket).await.unwrap();
        });
    }
}
async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];
    
    // Read data from the socket
    let n = socket.read(&mut buf).await?;
    
    // Write the data back to the socket
    socket.write_all(&buf[0..n]).await?;
    
    Ok(())
}
