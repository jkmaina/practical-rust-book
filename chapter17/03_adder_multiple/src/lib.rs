pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test_negative_numbers() {
        assert_eq!(add(-1, -2), -3);
    }
    #[test]
    fn test_positive_and_negative() {
        assert_eq!(add(5, -3), 2);
    }
}
