// RefCell<T>: Interior mutability pattern allowing mutation of immutable references
// Enforces borrowing rules at runtime instead of compile time
// Panics if borrowing rules are violated (multiple mutable borrows)
// Often used with Rc<T> for shared mutable data

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Counter {
    value: i32,
    name: String,
}

impl Counter {
    fn new(name: &str) -> Self {
        Counter {
            value: 0,
            name: name.to_string(),
        }
    }
    
    fn increment(&mut self) {
        self.value += 1;
        println!("{} incremented to {}", self.name, self.value);
    }
}

fn main() {
    println!("=== Basic RefCell Usage ===");
    // Create a shared, mutable integer
    let shared_data = Rc::new(RefCell::new(5));
    println!("Initial value: {}", *shared_data.borrow());
    
    // Create another reference to the same data
    let shared_data2 = Rc::clone(&shared_data);
    
    // Modify the data through the first reference
    *shared_data.borrow_mut() += 1;
    println!("After first increment: {}", *shared_data.borrow());
    
    // Modify the data through the second reference
    *shared_data2.borrow_mut() += 1;
    println!("After second increment: {}", *shared_data2.borrow());
    
    // Both references see the updated value
    println!("shared_data: {}", *shared_data.borrow());
    println!("shared_data2: {}", *shared_data2.borrow());
    
    println!("\n=== Multiple Immutable Borrows (OK) ===");
    {
        let borrow1 = shared_data.borrow();
        let borrow2 = shared_data.borrow();
        println!("borrow1: {}, borrow2: {}", *borrow1, *borrow2);
    } // borrows dropped here
    
    println!("\n=== Struct with RefCell ===");
    let counter = Rc::new(RefCell::new(Counter::new("SharedCounter")));
    let counter2 = Rc::clone(&counter);
    
    // Modify through different references
    counter.borrow_mut().increment();
    counter2.borrow_mut().increment();
    
    println!("Final counter state: {:?}", *counter.borrow());
    
    println!("\n=== Safe Borrowing with try_borrow ===");
    let data = RefCell::new(42);
    
    // Safe borrowing that returns Result
    match data.try_borrow() {
        Ok(value) => println!("Successfully borrowed: {}", *value),
        Err(e) => println!("Borrow failed: {}", e),
    }
    
    // Demonstrate borrow checking
    let _mutable_borrow = data.borrow_mut();
    match data.try_borrow() {
        Ok(_) => println!("This won't print"),
        Err(e) => println!("Expected borrow failure: {}", e),
    }
    
    println!("\n=== Runtime Borrow Checking ===");
    println!("RefCell enforces Rust's borrowing rules at runtime:");
    println!("- Multiple immutable borrows: OK");
    println!("- One mutable borrow: OK");
    println!("- Mutable + immutable borrow: PANIC");
    println!("- Multiple mutable borrows: PANIC");
    
    // Uncomment to see panic:
    // let panic_cell = RefCell::new(0);
    // let _borrow1 = panic_cell.borrow_mut();
    // let _borrow2 = panic_cell.borrow_mut(); // This would panic!
}
