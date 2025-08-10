// Advanced vector operations for manipulation and transformation

fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", v);
    
    // Insert an element at a specific position
    v.insert(1, 10);
    println!("After insert(1, 10): {:?}", v);
    
    // Append another vector
    let mut v2 = vec![6, 7, 8];
    v.append(&mut v2);
    println!("After append: {:?}", v);
    println!("v2 after append: {:?}", v2); // v2 is now empty
    
    // Extend with any iterator
    v.extend([9, 10].iter());
    println!("After extend: {:?}", v);
    
    // Retain only elements that satisfy a predicate
    v.retain(|&x| x % 2 == 0);
    println!("After retain (even numbers): {:?}", v);
    
    // Dedup removes consecutive duplicate elements
    let mut v3 = vec![1, 2, 2, 3, 3, 3, 4];
    println!("\nBefore dedup: {:?}", v3);
    v3.dedup();
    println!("After dedup: {:?}", v3);
    
    // Split off elements after a certain position
    let mut v4 = vec![1, 2, 3, 4, 5];
    println!("\nBefore split_off: {:?}", v4);
    let v5 = v4.split_off(3);
    println!("After split_off(3) - v4: {:?}", v4);
    println!("After split_off(3) - v5: {:?}", v5);
}