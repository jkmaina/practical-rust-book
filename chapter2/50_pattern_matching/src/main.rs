// This is a simple Rust program that reads a line from standard input
// and prints it to the console. It uses pattern matching to handle
// the result of reading the input.

use std::io;
fn main() {
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("You entered: {}", input),
        Err(error) => println!("Error: {}", error),
    }
}
