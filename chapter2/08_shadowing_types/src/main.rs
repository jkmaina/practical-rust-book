// This is a simple Rust program that demonstrates shadowing types.
// It starts with a string slice and then shadows it with an integer.
// The program prints both the string and the integer values.
fn main() {
    let data = "123"; // data is a &str (string slice)
    println!("Data as string: {}", data); 
    let data = data.parse::<i32>().expect("Not a valid number"); // data is now an i32 (integer)
    println!("Data as number: {}", data);
}
