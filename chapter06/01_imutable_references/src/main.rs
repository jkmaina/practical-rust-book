// This example demonstrates immutable references in Rust.
// Immutable references allow you to read data without modifying it.

fn main() {
    let s = String::from("hello");
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    
    println!("{} and {}", r1, r2);
}
