// This program demonstrates the difference between integer and floating-point division in Rust.
// It shows how integer division truncates the result, while floating-point division provides a precise result
// when using floats or mixed types.
// The program prints the results of both types of division to the console.

fn main() {
    let integer_division = 5 / 2;
    let float_division = 5.0 / 2.0;
    let mixed_division = 5 as f64 / 2.0;
    
    println!("5 / 2 = {}", integer_division);     // Outputs: 5 / 2 = 2
    println!("5.0 / 2.0 = {}", float_division);   // Outputs: 5.0 / 2.0 = 2.5
    println!("5 as f64 / 2.0 = {}", mixed_division); // Outputs: 5 as f64 / 2.0 = 2.5
}
