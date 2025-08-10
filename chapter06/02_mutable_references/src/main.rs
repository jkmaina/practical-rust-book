// This code demonstrates the rules of mutable references in Rust.
// You can only have one mutable reference to a piece of data in a particular scope.

fn main() {
    let mut s = String::from("hello");
    
    let r1 = &mut s;
    // let r2 = &mut s; // ERROR: cannot borrow `s` as mutable more than once
    
    println!("{}", r1);
}
