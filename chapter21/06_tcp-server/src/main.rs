use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");
    loop {
        // Accept a new connection
        let (socket, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);
        // Spawn a new task to handle the connection
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                println!("Error handling connection: {}", e);
            }
        });
    }
}
async fn handle_connection(mut socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];
    // Read data from the socket
    loop {
        let n = socket.read(&mut buffer).await?;
        
        if n == 0 {
            // Connection closed
            return Ok(());
        }
        // Write the data back to the socket
        socket.write_all(&buffer[0..n]).await?;
    }
}