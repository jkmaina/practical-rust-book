// This is a simple Rust program that demonstrates a continuous loop
// where the user can input messages until they type 'quit' to exit the program.
// The loop will keep running, allowing the user to enter messages,
// and it will print the messages back to the user until 'quit' is entered.
// The program uses the `io` module to read user input from the standard input.
// The loop will run indefinitely until a break statement is encountered.
use std::io;
fn main() {
    println!("Type 'quit' to exit the program.");
    
    loop {
        println!("Enter a message:");
        
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim();
        
        if input == "quit" {
            println!("Exiting the program...");
            break;
        }
        
        println!("You entered: {}", input);
    }
    
    println!("Program terminated.");
}
