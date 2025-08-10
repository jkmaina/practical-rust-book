// This code demonstrates the use of generic functions in Rust.
// It defines a generic function `print_value` that can accept any type
// that implements the `Display` trait, allowing it to print values of
// various types.

fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}
fn main() {
    print_value(42);       // Works with i32
    print_value(3.14);     // Works with f64
    print_value("hello");  // Works with &str
}
