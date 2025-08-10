// This code demonstrates the immutability of variables in Rust.
fn main() {
    let age = 28;
    age = 29; // This will cause a compilation error!
    println!("I am {} years old.", age);
}
