// Deref trait: enables smart pointers to act like regular references
// Provides automatic dereferencing and deref coercion for function calls
// Essential for creating custom smart pointer types

use std::ops::Deref;

// Custom smart pointer type
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implementing Deref allows * operator and deref coercion
impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0  // Return reference to wrapped value
    }
}

// Function that takes a string slice
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    // Basic dereferencing
    let x = 5;
    let y = MyBox::new(x);
    
    println!("x = {}", x);
    println!("*y = {}", *y);  // Deref allows * operator
    println!("y == x: {}", *y == x);
    
    // Deref coercion with strings
    let m = MyBox::new(String::from("Rust"));
    
    // Deref coercion: MyBox<String> -> &String -> &str
    hello(&m);  // Automatically converts MyBox<String> to &str
    
    // Without deref coercion, we'd need:
    // hello(&(*m)[..]);
    
    // Nested dereferencing
    let nested = MyBox::new(MyBox::new(42));
    println!("**nested = {}", **nested);
    
    // Method calls work through deref coercion
    let boxed_string = MyBox::new(String::from("Hello World"));
    println!("Length: {}", boxed_string.len());  // Calls String::len()
    println!("Uppercase: {}", boxed_string.to_uppercase());
    
    // Comparison with standard Box
    let standard_box = Box::new(100);
    let custom_box = MyBox::new(100);
    println!("Standard Box: {}", *standard_box);
    println!("Custom Box: {}", *custom_box);
}
