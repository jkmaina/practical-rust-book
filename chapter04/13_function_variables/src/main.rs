// This program demonstrates the use of function variables in Rust.
// It defines a function that takes another function as an argument and applies it to two integers.
// The program includes two operations: addition and multiplication.
// It then calls the function with both operations and prints the results.

fn apply_operation(a: i32, b: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(a, b)
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn main() {
    let result1 = apply_operation(5, 3, add);
    let result2 = apply_operation(5, 3, multiply);
    
    println!("5 + 3 = {}", result1);
    println!("5 * 3 = {}", result2);
}
