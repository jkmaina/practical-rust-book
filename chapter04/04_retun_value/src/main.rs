// This program defines a function `add` that takes two integers and returns their sum.
// The main function calls `add` with two numbers and prints the result.

fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}
