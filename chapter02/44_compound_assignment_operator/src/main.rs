// This Rust program demonstrates the use of compound assignment operators
// to modify the value of a variable in various ways.
// It initializes a variable, applies compound assignments, and prints the updated value.

fn main() {
    let mut x = 5;
    println!("x starts as {}", x);
    
    x += 1;  // Equivalent to: x = x + 1
    println!("After x += 1, x = {}", x);
    
    x -= 2;  // Equivalent to: x = x - 2
    println!("After x -= 2, x = {}", x);
    
    x *= 3;  // Equivalent to: x = x * 3
    println!("After x *= 3, x = {}", x);
    
    x /= 4;  // Equivalent to: x = x / 4
    println!("After x /= 4, x = {}", x);
    
    x %= 2;  // Equivalent to: x = x % 2
    println!("After x %= 2, x = {}", x);
}
