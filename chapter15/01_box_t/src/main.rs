// Box<T>: smart pointer for heap allocation with single ownership
// Use cases: recursive types, large data, trait objects, transferring ownership

// Recursive type example - requires Box to have known size
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    // Basic heap allocation
    let b = Box::new(5);
    println!("Boxed value: {}", *b);
    
    // Box enables recursive data structures
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive list: {:?}", list);
    
    // Moving large data efficiently
    let large_array = Box::new([0; 1000]);
    let moved_array = large_array; // Moves pointer, not data
    println!("Large array first element: {}", moved_array[0]);
    
    // Box with trait objects
    let boxed_closure: Box<dyn Fn(i32) -> i32> = Box::new(|x| x * 2);
    println!("Closure result: {}", boxed_closure(21));
    
    // Automatic deallocation when Box goes out of scope
    {
        let temp_box = Box::new(String::from("temporary"));
        println!("Temp value: {}", temp_box);
    } // temp_box and its contents are deallocated here
    
    println!("Box demonstrates RAII - automatic cleanup");
}
