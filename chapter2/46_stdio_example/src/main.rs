// This program demonstrates how to read input from the standard input (stdin) in Rust.
// It prompts the user for their name and then greets them with that name.
// The program uses the `io` module from the standard library to handle input and output.
// The `read_line` method is used to read a line of input from the user,
// and `trim` is used to remove any trailing newline characters from the user input.
// This is a simple example of how to interact with users in a console application.
use std::io;

fn main() {
    println!("What is your name?");

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let name = name.trim();
    println!("Hello, {}!", name);
}
