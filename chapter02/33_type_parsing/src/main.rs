// This program demonstrates how to parse a string into a number in Rust.
// The `parse` method is used to convert a string to a specific type, like i32.
// If the string cannot be parsed, it will panic with an error message "Not a number".
// Otherwise it will print the parsed number.

fn main() {
    // Now Rust knows you want an i32
    let number: i32 = "10".parse().expect("Not a number");
    println!("number: {}", number);
}
