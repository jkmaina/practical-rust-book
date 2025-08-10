// Result-based testing example showing alternative to panic-based assertions
// Tests can return Result<(), E> instead of panicking on failure
// Useful for tests that need custom error handling or early returns
// The ? operator can be used for convenient error propagation

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Test function returning Result - test fails if Err is returned
#[test]
fn test_with_result() -> Result<(), String> {
    if add(2, 2) == 4 {
        Ok(()) // Test passes
    } else {
        Err(String::from("2 + 2 did not equal 4")) // Test fails with custom message
    }
}
