// This code demonstrates the use of nested modules in Rust, showing how to define
// public and private functions within those modules.
// The `front_of_house` module contains two nested modules: `hosting` and `serving`.
// Each nested module has public functions that can be accessed from outside the module,
// while some functions are private and cannot be accessed directly from outside the module.
// The `main` function calls the public functions and demonstrates visibility rules.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
        
        fn seat_at_table() {
            println!("Seated at table");
        }
    }
    
    pub mod serving {
        pub fn take_order() {
            println!("Took order");
        }
        
        fn serve_order() {
            println!("Served order");
        }
        
        fn take_payment() {
            println!("Took payment");
        }
    }
}

fn main() {
    // Can access public functions in public modules
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();
    
    // These would cause compile errors (private functions):
    // front_of_house::hosting::seat_at_table();
    // front_of_house::serving::serve_order();
}