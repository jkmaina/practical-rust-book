// This program demonstrates the use of the remainder operator (%) in Rust.
// The remainder operator is useful for checking divisibility, cycling through ranges, or wrapping around values (like a clock).

fn main() {
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder); // Outputs: 43 % 5 = 3

    // Check if a number is divisible by another number
    let number = 10;
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    // Cycle through a range (like wrapping around a clock)
    let hour = 13;
    let hour_on_clock = hour % 12;
    println!("{} o'clock on a 12-hour clock is {}", hour, hour_on_clock);
}
