// Documentation tests - executable code examples in doc comments

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = documentation_test::add(2, 2);
/// assert_eq!(result, 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b + 1 //Intentional wrong so that documentation test fails
}



