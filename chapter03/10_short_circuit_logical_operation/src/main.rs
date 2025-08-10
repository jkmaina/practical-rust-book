// This Rust program demonstrates short-circuit evaluation in logical operations.
// The second condition is not evaluated if the first is false.
// This is useful for avoiding unnecessary computations or function calls.

fn main() {
    let x = 5;
    
    // The second condition is never evaluated because the first is false
    if x > 10 && some_function() {
        println!("x is greater than 10 and some_function returned true");
    }
}

fn some_function() -> bool {
    println!("some_function was called");
    true
}
