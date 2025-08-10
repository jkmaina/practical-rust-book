// This program demonstrates how to use a for loop to count down from 5 to 1.
// It prints each number followed by an exclamation mark, and finally prints "Liftoff!".

fn main() {
    for number in (1..=5).rev() {
        println!("{}!", number);
    }
    println!("Liftoff!");
}
