// This program demonstrates type inference constraints in Rust.
// It will not compile because the vector is expected to hold only integers.

fn main() {
    let mut data = Vec::new();
    data.push(42);
    data.push(3.14); // This will cause a compilation error
}
