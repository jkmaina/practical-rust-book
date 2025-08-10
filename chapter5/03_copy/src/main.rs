// This program demonstrates the concept of copying in Rust.
// In Rust, some types implement the Copy trait, which allows them to be copied rather than moved.
// This example shows how integers are copied, allowing both the original and the copy to be used
// without invalidating the original value.

fn main() {
    let x = 5;
    let y = x; // x is copied to y, not moved
    
    println!("x = {}, y = {}", x, y); // Both x and y are valid
}
