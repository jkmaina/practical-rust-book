// This code demonstrates the use of the `dbg!` macro in Rust to print debug information.
// The `dbg!` macro is useful for debugging by printing values to the console.
// It can be used to inspect values at runtime without needing to implement the `Debug` trait.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
}
