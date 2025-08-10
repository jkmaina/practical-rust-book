// chapter3/18_for_loop/src/main.rs
// This program demonstrates a simple for loop in Rust.
// It iterates over a range of numbers and prints them.
// The range is inclusive of the start and end values.
// The output will be the numbers from 1 to 5, followed by "Liftoff!".
fn main() {
    // Iterate over a range
    for number in 1..=5 {
        println!("{}!", number);
    }
    println!("Liftoff!");
}
