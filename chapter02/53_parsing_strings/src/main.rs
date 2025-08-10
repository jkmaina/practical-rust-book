// This program reads a line from standard input, attempts to parse it as an integer,
// and handles any parsing errors gracefully.
use std::io;
fn main() {
    println!("Enter a number:");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("That's not a valid number!");
            0 // Default value
        }
    };
    
    println!("You entered: {}", number);
}
