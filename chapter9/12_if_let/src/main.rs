// This Rust program demonstrates the use of `if let` for pattern matching.
// It shows how to match a specific value in an `Option` type and ignore others.
// The `if let` construct is a concise way to handle cases where you only care about
// one specific pattern, making the code cleaner and easier to read.
// It is particularly useful when dealing with `Option` or `Result` types in Rust.
// The underscore (_) is used to catch all other cases, similar to a wildcard match.
// The unit value () indicates that we do not care about the other cases.

fn main() {
    // This value contains 0, not 3, so the if let won't execute
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");  // This won't print because value is 0, not 3
    }
    
    // Example that will match
    let another_value = Some(3u8);
    if let Some(3) = another_value {
        println!("Found three!");  // This will print
    }
    
    // More practical example: extract any value from Some
    let config_value = Some(42u8);
    if let Some(value) = config_value {
        println!("Config value is: {}", value);  // Prints: Config value is: 42
    }
    
    // Equivalent match statement (more verbose)
    match config_value {
        Some(value) => println!("Match version: {}", value),
        None => (), // We ignore None case
    }
}