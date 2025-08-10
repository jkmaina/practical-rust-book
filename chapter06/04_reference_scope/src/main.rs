// This is a simple Rust program demonstrating reference scope.
// It shows how multiple immutable references can coexist,
// and how a mutable reference can be created after the immutable references are no longer used.

fn main() {
    let mut s = String::from("hello");
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // no problem
    println!("{}", r3);
}
