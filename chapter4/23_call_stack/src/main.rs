// The call stack evolves as follows:
// 1. `main` is called, and its stack frame is created
// 2. `first_function` is called, and its stack frame is created on top of `main`'s
// 3. `second_function` is called, and its stack frame is created on top of `first_function`'s
// 4. `second_function` returns, and its stack frame is removed
// 5. Execution continues in `first_function`
// 6. `first_function` returns, and its stack frame is removed
// 7. Execution continues in `main`

fn main() {
    println!("Starting main");
    first_function();
    println!("Back in main");
}
fn first_function() {
    println!("In first_function");
    second_function();
    println!("Back in first_function");
}
fn second_function() {
    println!("In second_function");
}

