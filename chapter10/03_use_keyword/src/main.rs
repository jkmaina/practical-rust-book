// This code demonstrates how to use a module in Rust.
// It defines a module named `front_of_house` with a submodule `hosting`.
// The `hosting` module contains a function `add_to_waitlist` that prints a message.
// The main function calls this function to demonstrate its usage.
// The `use` keyword is used to bring the `hosting` module into scope.


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

use front_of_house::hosting;

fn main() {
    hosting::add_to_waitlist();
}