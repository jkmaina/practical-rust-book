// Basic vector operations and creation methods

fn main() {
    // Creating an empty vector
    let mut v1: Vec<i32> = Vec::new();
    v1.push(10);
    v1.push(20);
    println!("v1: {:?}", v1);
    
    // Creating a vector with initial values using the vec! macro
    let v2 = vec![1, 2, 3, 4, 5];
    println!("v2: {:?}", v2);
    
    // Creating a vector with a specific capacity
    let mut v3: Vec<String> = Vec::with_capacity(10);
    println!("v3 capacity: {}", v3.capacity());
    
    // Creating a vector with repeated values
    let v4 = vec!["hello"; 3]; // Creates ["hello", "hello", "hello"]
    println!("v4: {:?}", v4);

    // Accessing elements
    let first = &v2[0];
    println!("First element in v2: {}", first);
    
    let second = v2.get(1);
    match second {
        Some(value) => println!("Second element in v2: {}", value),
        None => println!("No second element in v2"),
    }

    // Modifying elements
    v3.push("world".to_string());
    v3.push("rust".to_string());
    println!("v3 after pushes: {:?}", v3);

    // Iterating over a vector
    println!("Iterating over v3:");
    for s in &v3 {
        println!("{}", s);
    }

    //Removing elements
    let mut v = vec![1, 2, 3, 4, 5, 6];
    // Remove and return the last element
    let last = v.pop(); // last = Some(5), v = [1, 2, 3, 4]
    println!("Popped element: {:?}", last);
    // Remove element at a specific index
    let second = v.remove(1); // second = 2, v = [1, 3, 4]
    println!("Removed element at index 1: {}", second);
    // Clear all elements
    v.clear(); // v = []
    println!("Vector after clearing: {:?}", v);
}
