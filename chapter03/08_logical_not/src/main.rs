// This program demonstrates the use of the logical NOT operator (!)
// in Rust to check the negation of a boolean value.
// It prints a message if the condition is true.

fn main() {
    let is_available = false;
    
    if !is_available {
        println!("The item is not available.");
    }
}
