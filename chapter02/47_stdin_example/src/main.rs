// This program reads a line of input from the user and prints a greeting message.
// It demonstrates how to use the standard input in Rust.

use std::io;
fn main() {
    println!("What is your name?");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("Hello, {}!", input.trim());
}
