// This program demonstrates the difference between stack and heap memory in Rust.
// Stack memory is used for fixed-size data, while heap memory is used for dynamically sized data
// such as strings and vectors. The program shows how values are copied on the stack
// and how ownership works when moving data between variables.
// It also illustrates how to clone data to keep multiple variables valid.

fn main() {
    // Stack allocation
    let x = 5; // Integer stored directly on the stack
    let y = x; // Copy of x's value also stored on the stack

    // Heap allocation
    let s1 = String::from("hello"); // String data stored on the heap, pointer on the stack
    let s2 = s1; // s1's pointer is moved to s2, s1 is no longer valid

    // If you want to keep both variables valid, use clone()
    let s3 = String::from("world");
    let s4 = s3.clone(); // Deep copy: heap data is duplicated

    println!("x: {}, y: {}", x, y);
    println!("s2: {}", s2);
    println!("s3: {}, s4: {}", s3, s4);
}
