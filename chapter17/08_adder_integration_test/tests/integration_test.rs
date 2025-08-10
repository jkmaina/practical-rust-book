// Integration test - tests the public API of the crate from an external perspective
// Located in tests/ directory, treated as separate crate
// Only has access to public functions and items

use adder_integration_test;

#[test]
fn test_add() {
    assert_eq!(adder_integration_test::add(2, 2), 4);
}

#[test]
fn test_add_multiple_cases() {
    assert_eq!(adder_integration_test::add(0, 0), 0);
    assert_eq!(adder_integration_test::add(1, 1), 2);
    assert_eq!(adder_integration_test::add(100, 200), 300);
}