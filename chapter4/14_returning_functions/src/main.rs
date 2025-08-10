// This program demonstrates returning functions in Rust.
// It defines a function that returns another function based on a string input.
// The returned function can then be used to perform operations on two integers.

fn get_operation(operation_name: &str) -> fn(i32, i32) -> i32 {
    match operation_name {
        "add" => add,
        "multiply" => multiply,
        _ => add, // Default to add
    }
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
fn main() {
    let operation = get_operation("multiply");
    let result = operation(5, 3);
    println!("Result: {}", result);
}
