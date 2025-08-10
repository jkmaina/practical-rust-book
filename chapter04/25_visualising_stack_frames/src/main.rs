// This program demonstrates a simple function call and how to visualize the stack frames.
// It defines a function `add_one` that takes an integer, adds one to it,
// and returns the result. The main function calls `add_one` and prints the result.

fn main() {
    let x = 5;
    let y = add_one(x);
    println!("{} + 1 = {}", x, y);
}
fn add_one(n: i32) -> i32 {
    let result = n + 1;
    result
}
