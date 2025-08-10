// This program demonstrates simple function definitions and calls in Rust.
// It includes functions for printing messages and performing basic arithmetic.
// Each function is defined with a specific purpose and is called in the main function.

fn main() {
    say_hello();
    print_number(42);
    let sum = add(5, 10);
    println!("The sum is: {}", sum);
}
fn say_hello() {
    println!("Hello, world!");
}
fn print_number(x: i32) {
    println!("The number is: {}", x);
}
fn add(a: i32, b: i32) -> i32 {
    a + b
}