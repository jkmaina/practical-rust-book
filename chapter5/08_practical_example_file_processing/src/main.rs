// This example demonstrates file processing in Rust, focusing on ownership and resource management.
// It shows how to read the content of a file and handle ownership transfer.
// The code is designed to be run in a Rust environment with a file named "hello.txt

use std::fs::File;
use std::io::Read;
fn main() {
    // Open a file - this creates a unique resource
    let file = File::open("hello.txt").expect("Failed to open file");
    
    // Process the file
    let content = read_file_content(file);
    
    // At this point, 'file' is no longer valid because ownership was moved
    // to the read_file_content function
    
    println!("File content: {}", content);
    
    // If we tried to use 'file' again, we'd get a compile error:
    // let more_content = read_more_from_file(file); // Error: file is no longer valid
}
fn read_file_content(mut file: File) -> String {
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");
    content
}
