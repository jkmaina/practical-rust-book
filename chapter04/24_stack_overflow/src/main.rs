// This code demonstrates a stack overflow by using a recursive function
// that calls itself too many times without a base case to stop the recursion.
// This will lead to a stack overflow error when run.
// The function `recursive_function` calls itself with a decremented value until it reaches zero,
// but since the recursion depth is too high, it will exceed the stack limit.

fn recursive_function(n: u32) {
    println!("n = {}", n);
    if n > 0 {
        recursive_function(n - 1);
    }
}
fn main() {
    recursive_function(1_000_000); // This will likely cause a stack overflow
}
