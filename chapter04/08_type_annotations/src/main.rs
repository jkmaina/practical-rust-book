// This program demonstrates the use of type annotations in Rust.
// It defines a function that expects an i32 parameter and prints it.
// The function is called with an integer, which works correctly, but calling it with
// a floating-point number or a string will result in a compile-time error due to type mismatch
// This illustrates how Rust's type system helps catch errors at compile time.

fn print_value(value: i32) {
    println!("The value is: {}", value);
}
fn main() {
    print_value(42);    // OK
    print_value(3.14);  // Error: expected i32, found floating-point number
    print_value("hello"); // Error: expected i32, found &str
}
