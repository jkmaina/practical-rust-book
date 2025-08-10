use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
#[tokio::main]
async fn main() -> io::Result<()> {
    let (content1, content2) = tokio::try_join!(
        read_file("file1.txt"),
        read_file("file2.txt")
    )?;
    
    println!("File 1: {}", content1);
    println!("File 2: {}", content2);
    
    Ok(())
}
async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut content = String::new();
    file.read_to_string(&mut content).await?;
    Ok(content)
}
 
