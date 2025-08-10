// This program demonstrates the use of ordering operators in Rust.
// Ordering operators are used to compare two values and determine their relative order.
// Common ordering operators include: <, >, <=, >=.
// In this example, we will compare integers and floating-point numbers.
// The output will show whether the comparisons are true or false.
// This is useful for making decisions in your code based on the values of variables.
fn main() {
    let x = 5;
    let y = 10;

    println!("x < y: {}", x < y); // true
    println!("x > y: {}", x > y); // false
    println!("x <= y: {}", x <= y); // true
    println!("x >= y: {}", x >= y); // false

    let a = 5.0;
    let b = 10.0;

    println!("a < b: {}", a < b); // true
    println!("a > b: {}", a > b); // false
}
