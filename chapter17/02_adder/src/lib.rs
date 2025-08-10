// Library crate example with tests
// Demonstrates proper library structure and testing conventions
// Tests are placed in a separate module with #[cfg(test)] attribute

// Public function that can be used by other crates
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// Test module - only compiled when running tests
#[cfg(test)]
mod tests {
    // Import all items from parent module
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
