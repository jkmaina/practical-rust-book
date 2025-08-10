// This program demonstrates ownership and borrowing in Rust.
// It shows how ownership works with functions, including the concept of moving values
// and the Copy trait for types that can be copied rather than moved.
// The program defines a function that takes ownership of a String and another function
// that takes a Copy type, demonstrating how values are moved and copied in Rust.

fn main() {
    let s = String::from("hello"); // s comes into scope
    
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    
    // println!("{}", s); // Error: s is no longer valid
    
    let x = 5; // x comes into scope
    
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
    
    println!("{}", x); // This works fine
}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called.
  // The backing memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
