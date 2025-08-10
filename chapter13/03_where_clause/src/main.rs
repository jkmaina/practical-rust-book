// Where clauses improve readability for complex trait bounds
use std::fmt::{Debug, Display};

// Without where clause (harder to read)
fn some_function_inline<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    println!("T: {}", t);
    println!("U: {:?}", u);
    42
}

// With where clause (cleaner)
fn some_function_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("T: {}", t);
    println!("U: {:?}", u);
    42
}

// More complex example showing where clause benefits
fn complex_function<T, U, V>(t: T, u: U, v: V) -> String
where
    T: Display + Clone + Debug,
    U: Clone + Debug + PartialEq,
    V: Display + Copy,
{
    format!("T: {}, U: {:?}, V: {}", t, u, v)
}

fn main() {
    let text = "Hello";
    let numbers = vec![1, 2, 3];
    
    println!("=== Inline syntax ===");
    let result1 = some_function_inline(&text, &numbers);
    println!("Result: {}\n", result1);
    
    println!("=== Where clause syntax ===");
    let result2 = some_function_where(&text, &numbers);
    println!("Result: {}\n", result2);
    
    println!("=== Complex function ===");
    let result3 = complex_function("Rust", vec![1, 2], 42);
    println!("Result: {}", result3);
}