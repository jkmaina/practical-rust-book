// Demonstration of using where clauses in Rust for better readability
// This example shows how to define functions with complex trait bounds


// Where clauses make complex trait bounds more readable
use std::fmt::{Debug, Display};

// Without where clause (harder to read)
fn process_data_inline<T: Clone + Display, U: Clone + Debug, V: Copy + Clone + Eq>(
    t: T, u: U, v: V
) -> i32 {
    println!("Processing: {}", t);
    println!("Debug: {:?}", u);
    42
}

// With where clause (cleaner)
fn process_data_where<T, U, V>(t: T, u: U, v: V) -> i32 
where
    T: Clone + Display,
    U: Clone + Debug,
    V: Copy + Clone + Eq,
{
    println!("Processing: {}", t);
    println!("Debug: {:?}", u);
    42
}

fn main() {
    let result1 = process_data_inline("hello", vec![1, 2, 3], 42);
    println!("Result 1: {}\n", result1);
    
    let result2 = process_data_where("world", (1, 2), 'A');
    println!("Result 2: {}", result2);
}