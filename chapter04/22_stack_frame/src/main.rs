// This is a simple Rust program that demonstrates the use of a function to double a number.
// The function `double` takes an integer as input and returns its double.
// The main function calls `double` with a value and prints the result.

// 1. A stack frame is created for `main`
//    a. `x` is stored in this frame with the value 5
// 2. When `double(x)` is called, a new stack frame is created for `double`
//    b. `n` is stored in this frame with the value 5 (copied from `x`)
//    c. `result` is stored in this frame with the value 10
// 4. `double` returns 10, and its stack frame is removed
// 5. Back in `main`, `y` is stored in the `main` frame with the value 10
// 6. The program prints "x: 5, y: 10"
// 7. `main` returns, and its stack frame is removed

fn main() {
    let x = 5;
    let y = double(x);
    println!("x: {}, y: {}", x, y);
}

fn double(n: i32) -> i32 {
    let result = n * 2;
    result
}
