// This program demonstrates functions with no return type in Rust.
// Functions can either return a value or not return anything, which is represented by the unit type `()`.
// Here, we define two functions: one with an implicit return type of `()` and another with an explicit return type of `()`.

fn say_hello() { // Return type is implicitly ()
    println!("Hello!");
}

fn say_goodbye() -> () { // Explicitly specifying the unit type
    println!("Goodbye!");
}

fn main() {
    say_hello();
    say_goodbye();
}
