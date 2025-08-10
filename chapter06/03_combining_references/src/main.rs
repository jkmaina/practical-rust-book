// This example demonstrates the rules of borrowing in Rust.
// You can have multiple immutable references or one mutable reference,
// but not both at the same time.

fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{} and {}", r1, r2);
}
