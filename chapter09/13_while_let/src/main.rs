// This Rust program demonstrates the use of `while let` for looping
// It shows how to repeatedly match a pattern until it no longer matches.
// The `while let` construct is useful for iterating over collections or
// handling values that may change, such as popping from a stack.
// while let loops as long as the pattern matches

fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    // Loop while pop() returns Some(value), stop when None
    while let Some(top) = stack.pop() {
        println!("{}", top);  // Prints: 3, 2, 1
    }
}