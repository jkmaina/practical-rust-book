// This is a simple Rust program demonstrating dereferencing
// to access the value stored in a reference.
// The program asserts that the value of `x` is equal to the value
// pointed to by `y`, which is a reference to `x`.

fn main() {
    let x = 5;
    let y = &x;
    
    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference y to get the value it points to
}