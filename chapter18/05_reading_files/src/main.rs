use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::open("hello.txt").await?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    
    println!("File contents: {}", contents);
    
    Ok(())
}
