use std::fs::File;
fn main() {
    // Using unwrap
    let f = File::open("hello.txt").unwrap();
    // Using expect
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
