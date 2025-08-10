// This program demonstrates the precision issues with floating-point arithmetic in Rust.
// It shows how adding 0.1 and 0.2 does not yield exactly 0.3 due to the way floating-point numbers are represented in binary.

fn main() {
    let result = 0.1 + 0.2;
    println!("0.1 + 0.2 = {}", result); // Prints: 0.1 + 0.2 = 0.30000000000000004
}
