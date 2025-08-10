// This is a basic module example in Rust
// demonstrating public and private functions.
// The `front_of_house` module contains a public function `add`
// and a private function `subtract`.
// The `main` function calls the public function and prints the result.

mod front_of_house {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    // Can call public function
    let result = front_of_house::add(5, 3);
    println!("5 + 3 = {}", result);

    // Cannot call private function   
    // This would cause a compile error - private function:
    // let result2 = front_of_house::subtract(5, 3);
}