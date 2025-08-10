// This is a simple Rust program that demonstrates shadowing of variables.
// It defines a variable `x` in the outer scope and then shadows it in an inner scope.
// The inner scope modifies the value of `x`, but it does not affect the outer scope.
// This illustrates how shadowing works in Rust, allowing you to reuse variable names in different scopes.
fn main() {
    let x = 5;
    
    {
        let x = x * 2; // This shadows x within this block only
        println!("Value of x in the inner scope: {}", x);
    }
    
    println!("Value of x in the outer scope: {}", x);
}
