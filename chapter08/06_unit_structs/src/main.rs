// Unit-Like Structs
// You can also define structs that don't have any fields! These are called unit-like structs
// because they behave similarly to `()`, the unit type. Unit-like structs can be useful when
// you want to mark or tag something in your code, or when you need a type but don't need to store any data.

// This struct can be used as a marker to indicate that a value is always considered equal.
struct AlwaysEqual;

// A function that takes a value and returns true if it is AlwaysEqual.
// In real code, you might use this as a marker for configuration, state, or tagging.
fn is_always_equal(_value: AlwaysEqual) -> bool {
    true
}

fn main() {
    let subject = AlwaysEqual;
    // Use the unit-like struct in a function
    if is_always_equal(subject) {
        println!("The subject is always equal!");
    }
}
