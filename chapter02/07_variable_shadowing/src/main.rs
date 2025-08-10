// This is a simple Rust program that demonstrates variable shadowing.
// It defines a variable `message`, prints it, then redefines it and prints the new
fn main() {
    let message = "Hello";
    println!("Message initially is: {}", message);
    
    let message = "Hello, World!";
    println!("Message is now: {}", message);
}
