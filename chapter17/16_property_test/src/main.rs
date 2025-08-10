// Property-based testing example using proptest crate
// Tests mathematical properties that should hold for all valid inputs
// Generates random test cases to verify properties like commutativity and associativity
// More thorough than example-based testing by exploring edge cases automatically

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    proptest! {
        #[test]
        fn test_add_commutative(a in -1000..1000, b in -1000..1000) {
            assert_eq!(add(a, b), add(b, a));
        }
        #[test]
        fn test_add_associative(a in -1000..1000, b in -1000..1000, c in -1000..1000) {
            assert_eq!(add(add(a, b), c), add(a, add(b, c)));
        }
        #[test]
        fn test_add_identity(a in -1000..1000) {
            assert_eq!(add(a, 0), a);
        }
    }
}
