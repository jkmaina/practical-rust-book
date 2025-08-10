// Ownership in Rust
// This program demonstrates ownership in Rust, where each value has a single owner.
// When ownership is transferred, the previous owner can no longer access the value.
// This example shows how ownership works with strings and how to handle ownership transfer.
// It also illustrates how to avoid dangling references by ensuring that the owner is valid.

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1's ownership is moved to s2
    
    // println!("{}", s1); // Error: s1 is no longer valid
    println!("{}", s2); // This works fine
}
 
