// This program defines a function `absolute_value` that returns the absolute value of an integer.
// The main function calls `absolute_value` with two numbers and prints the results.
// The function uses an `if` statement to check if the number is non-negative and returns it directly.
// If the number is negative, it returns its negation.

fn absolute_value(x: i32) -> i32 {
    if x >= 0 {
        return x;
    }
    -x
}
fn main() {
    println!("The absolute value of -5 is: {}", absolute_value(-5));
    println!("The absolute value of 10 is: {}", absolute_value(10));
}
