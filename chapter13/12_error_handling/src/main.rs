use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
fn read_file_contents<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn main() {
    // Both String and &str can be converted to &Path
    let result1 = read_file_contents("Cargo.toml");
    let result2 = read_file_contents(String::from("Cargo.toml"));
    
    match result1 {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
