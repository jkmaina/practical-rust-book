// This Rust program demonstrates the use of an `if` expression within a `let` statement.
// It assigns a value to a variable based on a condition.
// The value of `number` will be 5 if `condition` is true, otherwise it will be 6.

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {}", number);
}
