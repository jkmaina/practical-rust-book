// This is a simple Rust program that demonstrates integer overflow.
// In debug mode, it will panic when an overflow occurs.
// In release mode, it will wrap around without panicking.
// To run this code, use `cargo run` in the terminal.
fn main() {
    let mut value: u8 = 255;
    println!("Current value: {}", value);
    
    // This will cause a panic in debug mode
    value += 1;
    println!("After adding 1: {}", value);
}
