// This program demonstrates a common lifetime problem in Rust.
// The function `longest` takes two string slices and is supposed to return the longer one.
// However, this code won't compile because Rust can't determine how long the returned reference is valid.
// Rust requires explicit lifetime annotations when returning references tied to input parameters,
// to ensure the returned reference does not outlive its data.
//
// In this example, both `string1` and `string2` are valid for the duration of `main`,
// but the compiler can't know that just from the function signature of `longest`.
//
// To fix this, you would need to add explicit lifetime parameters to the function signature.

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// This function will not compile as written, because the compiler
// cannot infer the lifetime of the returned reference.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
