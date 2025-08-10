// Recursive types with Box<T>: enabling self-referential data structures
// Box provides indirection needed for recursive types to have known size at compile time
// Common examples: linked lists, trees, and other recursive data structures

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    // Helper function to create a new list
    fn new() -> List {
        Nil
    }
    
    // Add element to front of list
    fn prepend(self, elem: i32) -> List {
        Cons(elem, Box::new(self))
    }
    
    // Calculate length of list
    fn len(&self) -> usize {
        match self {
            Cons(_, tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    
    // Convert to string representation
    fn stringify(&self) -> String {
        match self {
            Cons(head, tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => format!("Nil"),
        }
    }
}

fn main() {
    // Create list using constructor syntax
    let list1 = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))))));
    
    println!("List1: {:?}", list1);
    println!("List1 length: {}", list1.len());
    println!("List1 string: {}", list1.stringify());
    
    // Create list using helper methods
    let list2 = List::new()
        .prepend(3)
        .prepend(2)
        .prepend(1);
    
    println!("\nList2: {:?}", list2);
    println!("List2 length: {}", list2.len());
    
    // Demonstrate why Box is needed
    println!("\nWhy Box is needed:");
    println!("- Without Box, List would have infinite size");
    println!("- Box provides heap allocation and indirection");
    println!("- Each Cons node stores data + pointer (fixed size)");
}
