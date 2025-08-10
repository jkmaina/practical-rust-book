pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}

#[test]
#[should_panic]
fn test_divide_by_zero() {
    divide(1, 0);
}

#[test]
fn test_with_custom_message() {
    let result = add(2, 2);
    assert_eq!(result, 4, "Addition didn't work! Got {}, expected 4", result);
}
