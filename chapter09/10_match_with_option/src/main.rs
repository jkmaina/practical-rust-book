// Option<T> represents a value that might exist (Some) or not exist (None)
// This prevents null pointer errors by forcing explicit handling of "no value" cases

// Function that safely adds 1 to an optional integer
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {  // Pattern match on the Option
        None => None,           // If no value, return no value
        Some(i) => Some(i + 1), // If has value, add 1 and wrap in Some
    }
}

fn main() {
    // Create Option values
    let five = Some(5);    // Option containing the value 5
    let six = plus_one(five);  // Adds 1, result: Some(6)
    let none = plus_one(None); // No value to add to, result: None
    
    // Print results - {:?} is debug formatting for Option
    println!("five: {:?}", five);  // Output: five: Some(5)
    println!("six: {:?}", six);    // Output: six: Some(6)
    println!("none: {:?}", none);  // Output: none: None
}
