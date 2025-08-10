// This is a simple Rust program that calculates the factorial of a number using recursion.
// The factorial of a non-negative integer n is the product of all positive integers less than or equal to n.
// It is defined as:
// n! = n * (n-1)! for n > 0
// and 0! = 1.

fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
fn main() {
    println!("5! = {}", factorial(5));  // Outputs: 5! = 120
}
