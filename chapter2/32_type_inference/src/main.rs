// This program demonstrates type inference in Rust.
// Rust can automatically determine the types of variables based on their values.

fn main() {
    // Rust infers these types
    let x = 5;          // i32
    let y = 3.0;        // f64
    let active = true;  // bool
    let letter = 'c';   // char
    
    println!("x: {}, y: {}, active: {}, letter: {}", 
             x, y, active, letter);
}
