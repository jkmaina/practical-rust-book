// Rc<T>: Reference Counted smart pointer for shared ownership
// Enables multiple owners of the same data (single-threaded only)
// Memory is freed when the last reference is dropped (reference count reaches 0)

use std::rc::Rc;

// Example struct to demonstrate shared ownership
#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

fn main() {
    println!("=== Basic Rc Usage ===");
    let data = Rc::new(5);
    println!("Initial reference count: {}", Rc::strong_count(&data));
    
    // Create two more references to the same data
    let data2 = Rc::clone(&data);  // Increments reference count
    println!("After first clone: {}", Rc::strong_count(&data));
    
    let data3 = Rc::clone(&data);  // Increments reference count again
    println!("After second clone: {}", Rc::strong_count(&data));
    
    // Use the shared data
    println!("data: {}, data2: {}, data3: {}", data, data2, data3);
    
    // Demonstrate reference counting in scopes
    {
        let data4 = Rc::clone(&data);
        println!("Inside scope: {}", Rc::strong_count(&data));
    } // data4 dropped here
    println!("After scope: {}", Rc::strong_count(&data));
    
    println!("\n=== Tree Structure Example ===");
    // Create a tree structure with shared nodes
    let leaf = Rc::new(Node {
        value: 3,
        children: vec![],
    });
    
    let branch1 = Rc::new(Node {
        value: 1,
        children: vec![Rc::clone(&leaf)],  // Shared reference to leaf
    });
    
    let branch2 = Rc::new(Node {
        value: 2,
        children: vec![Rc::clone(&leaf)],  // Another shared reference to leaf
    });
    
    println!("Leaf reference count: {}", Rc::strong_count(&leaf));
    println!("Branch1: {:?}", branch1);
    println!("Branch2: {:?}", branch2);
    
    println!("\n=== Manual Drop Demonstration ===");
    drop(data2);  // Manually drop one reference
    println!("After dropping data2: {}", Rc::strong_count(&data));
    
    drop(data3);  // Drop another reference
    println!("After dropping data3: {}", Rc::strong_count(&data));
    
    println!("\nWhen main ends, remaining references will be dropped automatically");
    // When data, branch1, branch2, and leaf go out of scope,
    // reference counts drop to 0 and memory is freed
}
