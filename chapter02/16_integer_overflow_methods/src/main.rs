//This code demonstrates different methods of handling integer overflow in Rust.
// In debug mode, it will panic on overflow, while in release mode, it will wrap around.
// To run this code, use `cargo run` in the terminal.
// This code uses methods like wrapping, checked, saturating, and overflowing to handle integer overflow
// in a controlled manner.
fn main() {
    let value: u8 = 255;
    
    // Wrapping (returns the value modulo the range)
    let wrapped = value.wrapping_add(1);
    println!("255 + 1 with wrapping = {}", wrapped); // Prints: 255 + 1 with wrapping = 0
    
    // Checked (returns None if overflow would occur)
    let checked = value.checked_add(1);
    match checked {
        Some(v) => println!("255 + 1 = {}", v),
        None => println!("Overflow occurred!"), // This will print
    }
    
    // Saturating (returns the max or min value if overflow would occur)
    let saturated = value.saturating_add(1);
    println!("255 + 1 with saturation = {}", saturated); // Prints: 255 + 1 with saturation = 255
    
    // Overflowing (returns the wrapped value and a boolean indicating overflow)
    let (overflowed, did_overflow) = value.overflowing_add(1);
    println!("255 + 1 = {} (overflow: {})", overflowed, did_overflow); // Prints: 255 + 1 = 0 (overflow: true)
}
