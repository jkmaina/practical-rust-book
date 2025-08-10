// This program demonstrates the concept of copying in Rust.
// In Rust, some types implement the Copy trait, which allows them to be copied rather than moved.
// This example shows how a custom struct can implement the Copy and Clone traits, allowing it to
// be copied rather than moved. This means both the original and the copy can be used without
// invalidating the original value.
// The `Point` struct is defined with the `Copy` and `Clone` traits, allowing instances of it to be copied.

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = p1; // p1 is copied to p2, not moved
    
    println!("p1 = {:?}, p2 = {:?}", p1, p2); // Both p1 and p2 are valid
}
