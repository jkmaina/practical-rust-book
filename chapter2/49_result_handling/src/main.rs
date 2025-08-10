// This program reads a line from standard input and prints it.
// If an error occurs while reading, it will panic with a custom message.
// Alternatively, it can use `unwrap()` to panic with a default message.
// This is useful for quick prototyping or when you are sure that the input will be valid
// and you want to avoid handling errors explicitly.

use std::io;
fn main() {
    let mut input = String::new();
    println!("Enter a line of text:");

    // Using expect will crash the program with the given message if an error occurs
    io::stdin().read_line(&mut input).expect("Failed to read line");
      
    println!("You entered: {}", input);

    // Reset input for another read
    input.clear();    
    // Using wrap will crash the program with a default message if an error occurs
    io::stdin().read_line(&mut input).unwrap();

    println!("You entered again: {}", input);
}
