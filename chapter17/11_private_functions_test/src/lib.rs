// Testing private functions example
// Unit tests can access private functions in the same module
// Integration tests can only access public functions
// Demonstrates Rust's visibility rules in testing context

// Private function - only accessible within this module
fn private_add(a: i32, b: i32) -> i32 {
    a + b
}

// Public function that uses the private function
pub fn public_add(a: i32, b: i32) -> i32 {
    private_add(a, b)
}

// Unit tests module - can access both private and public functions
#[cfg(test)]
mod tests {
    use super::*; // Import all items from parent module
    
    // Test private function - only possible in unit tests
    #[test]
    fn test_private_add() {
        assert_eq!(private_add(2, 2), 4);
    }
    
    // Test public function - accessible from both unit and integration tests
    #[test]
    fn test_public_add() {
        assert_eq!(public_add(2, 2), 4);
    }
}
