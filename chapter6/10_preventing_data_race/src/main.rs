// This code demonstrates how to prevent data races in Rust.

fn main() {
    let mut data = vec![1, 2, 3];
    
    let reference1 = &data;
    let reference2 = &mut data; // Error: cannot borrow `data` as mutable because it is also borrowed as immutable
    
    println!("{:?}", reference1);
    println!("{:?}", reference2);
}
