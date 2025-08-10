fn main() {
    // Creating a string
    let s1 = String::from("hello");
    println!("s1: {}", s1);
    
    // Moving ownership
    let s2 = s1;
    // println!("s1: {}", s1); // Error: s1 is no longer valid
    println!("s2: {}", s2);
    
    // Cloning a string
    let s3 = String::from("world");
    let s4 = s3.clone();
    println!("s3: {}, s4: {}", s3, s4);
    
    // Ownership and functions
    let s5 = String::from("ownership");
    takes_ownership(s5);
    // println!("s5: {}", s5); // Error: s5 is no longer valid
    
    // Returning ownership
    let s6 = gives_ownership();
    println!("s6: {}", s6);
    
    // Taking and returning ownership
    let s7 = String::from("hello");
    let s8 = takes_and_gives_back(s7);
    // println!("s7: {}", s7); // Error: s7 is no longer valid
    println!("s8: {}", s8);
}
fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string);
}
fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

