// This Rust program reads a user's age from standard input,
// converts it to a number, and then calculates what their age will be in 10 years
// It uses the `expect` method to handle potential errors during input reading and parsing.

use std::io;
fn main() {
    println!("Enter your age:");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // Remove whitespace and convert to u32
    let age: u32 = input.trim().parse().expect("Please enter a valid number");
    
    println!("In 10 years, you'll be {} years old.", age + 10);
}
