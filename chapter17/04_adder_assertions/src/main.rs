// Assertion macros example demonstrating different ways to test conditions
// Shows assert!, assert_eq!, and assert_ne! macros with their specific use cases
// Each macro provides different error messages and debugging information

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("Run 'cargo test' to see assertion examples");
}

// Test using assert! - for boolean conditions
#[test]
fn test_assert() {
    let result = add(2, 2);
    assert!(result == 4); // Panics if condition is false
}

// Test using assert_eq! - for equality comparison (preferred for equality)
#[test]
fn test_assert_eq() {
    let result = add(2, 2);
    assert_eq!(result, 4); // Shows both values if they don't match
}

// Test using assert_ne! - for inequality comparison
#[test]
fn test_assert_ne() {
    let result = add(2, 2);
    assert_ne!(result, 5); // Panics if values are equal
}