// Basic testing example demonstrating Rust's built-in test framework
// Tests are functions annotated with #[test] attribute
// Run tests with: cargo test
// Tests use assert! macros to verify expected behavior
fn main() {
    println!("Run 'cargo test' to execute tests");
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

