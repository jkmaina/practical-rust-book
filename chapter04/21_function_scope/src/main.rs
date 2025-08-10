// This program demonstrates function scope in Rust.
// Variables defined in an inner scope are not accessible outside that scope.
// However, variables defined in an outer scope are accessible in inner scopes.
// Uncommenting the last println! will cause a compile-time error.

fn main() {
    // Outer scope
    let x = 5;
    
    {
        // Inner scope
        let y = 10;
        println!("x: {}, y: {}", x, y); // Both x and y are accessible here
    }
    
    println!("x: {}", x); // x is still accessible here
    // println!("y: {}", y); // Error: y is no longer in scope
}
 
