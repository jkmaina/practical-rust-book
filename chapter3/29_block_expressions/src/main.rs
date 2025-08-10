// This code demonstrates the use of block expressions in Rust.
// A block expression is a sequence of statements enclosed in curly braces `{}`
// that can return a value. The last expression in the block is the value returned.

fn main() {

    let y = {
    let x = 5;
    x + 1  // Note: no semicolon here
};

    // The block expression returns the value of `x + 1`, which is 6
    // and assigns it to `y`.

println!("y = {}", y);  // Prints: y = 6
}
