// Closures: anonymous functions that can capture variables from their environment
// Three types: Fn (immutable borrow), FnMut (mutable borrow), FnOnce (take ownership)
// Closures are more flexible than functions and can capture context

fn main() {
    // Basic closure capturing environment
    let x = 4;
    let equal_to_x = |z| z == x;  // Captures x by reference
    let y = 4;
    println!("equal_to_x(4): {}", equal_to_x(y));
    
    // Closure with explicit parameter types
    let add_one = |num: i32| -> i32 { num + 1 };
    println!("add_one(5): {}", add_one(5));
    
    // Closure capturing mutable variable
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("First call: {}", increment());
    println!("Second call: {}", increment());
    
    // Using closures with iterators
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
    
    // Closure taking ownership
    let name = String::from("Alice");
    let greet = move || format!("Hello, {}!", name);
    println!("{}", greet());
    // println!("{}", name); // Would error - name was moved
}