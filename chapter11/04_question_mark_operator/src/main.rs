// Demonstrates the ? operator for error propagation
// The ? operator automatically returns early if an error occurs

use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;  // ? propagates error if file open fails
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // ? propagates error if read fails
    Ok(s)
}

fn main() {
    match read_username_from_file() {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {}", error),
    }
}
