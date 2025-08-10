/// A simple example of reading a line from standard input in Rust.
/// // This program prompts the user to enter a line of text and then reads it,
/// displaying the number of bytes read and the input itself.
use std::io;
fn main() {
    println!("Enter a line of text:");
    
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(n) => println!("{} bytes read", n),
        Err(error) => println!("Error reading input: {}", error),
    }
    
    println!("You entered: {}", input);
}