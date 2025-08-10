// This is an example of using if-else statements in Rust
// to check the value of a number and print corresponding messages.
// The program checks if the number is less than, equal to, or greater than 5
fn main() {
    let number = 6;
    
    if number < 5 {
        println!("The number is less than 5");
    } else if number == 5 {
        println!("The number is exactly 5");
    } else {
        println!("The number is greater than 5");
    }
}
