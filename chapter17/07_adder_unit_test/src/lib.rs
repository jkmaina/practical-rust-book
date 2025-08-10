// Unit testing example demonstrating comprehensive test coverage
// Shows testing of multiple functions with various scenarios
// Includes edge cases, error conditions, and boundary testing

// Calculator module with basic arithmetic operations
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn subtract(left: i32, right: i32) -> i32 {
    left - right
}

pub fn multiply(left: u32, right: u32) -> u32 {
    left * right
}

pub fn divide(left: f64, right: f64) -> Result<f64, String> {
    if right == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(left / right)
    }
}

pub fn is_even(n: u32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test addition with various inputs
    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(100, 200), 300);
    }

    #[test]
    fn test_add_large_numbers() {
        assert_eq!(add(u64::MAX - 1, 1), u64::MAX);
    }

    // Test subtraction including negative results
    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(3, 5), -2);
        assert_eq!(subtract(0, 0), 0);
    }

    // Test multiplication including edge cases
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
        assert_eq!(multiply(0, 100), 0);
        assert_eq!(multiply(1, 42), 42);
    }

    // Test successful division
    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10.0, 2.0), Ok(5.0));
        assert_eq!(divide(7.0, 2.0), Ok(3.5));
        assert_eq!(divide(0.0, 5.0), Ok(0.0));
    }

    // Test division by zero error case
    #[test]
    fn test_divide_by_zero() {
        assert!(divide(5.0, 0.0).is_err());
        assert_eq!(divide(5.0, 0.0), Err("Division by zero".to_string()));
    }

    // Test boolean function with various inputs
    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(100));
        assert!(!is_even(1));
        assert!(!is_even(99));
    }

    // Test using Result-based testing
    #[test]
    fn test_comprehensive_calculator() -> Result<(), String> {
        // Test a sequence of operations
        let sum = add(10, 5);
        let difference = subtract(sum as i32, 3);
        let product = multiply(difference as u32, 2);
        let quotient = divide(product as f64, 4.0)?;
        
        if quotient == 6.0 {
            Ok(())
        } else {
            Err(format!("Expected 6.0, got {}", quotient))
        }
    }
}
