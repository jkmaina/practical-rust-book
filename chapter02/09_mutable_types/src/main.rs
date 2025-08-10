// This is a simple Rust program that demonstrates the use of mutable types.
// It attempts to parse a string into an integer, which will cause a compilation error
// because the string is not a valid integer.
// The code is intentionally incorrect to illustrate the concept of mutable types.
// To fix the error, you would need to ensure that the string can be parsed into an integer.

fn main() {
    let mut data = "123";
    data = data.parse::<i32>().expect("Not a valid number"); // This will cause a compilation error!
}
