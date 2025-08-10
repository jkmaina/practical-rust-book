// chapter6/08_dangling_reference/src/main.rs
// This example demonstrates a dangling reference in Rust.
// A dangling reference occurs when a reference points to data that has been freed or dropped.
// This code will not compile due to the dangling reference error.

fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String { // ERROR: missing lifetime specifier
    let s = String::from("hello");
    &s // we return a reference to s
} // s goes out of scope and is dropped, so its memory is freed
