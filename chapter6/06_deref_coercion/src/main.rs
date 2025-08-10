// This Rust program demonstrates deref coercion.
// It shows how Rust automatically dereferences a reference to call methods on the underlying type.
// In this case, we call the `len` method on a `String` reference without needing to explicitly dereference it.

fn main() {
    let s = String::from("hello");
    let len = s.len(); // no need for (*s).len()
    println!("The length of '{}' is {}.", s, len);
}
