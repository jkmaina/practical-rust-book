use rust_decimal::Decimal;
use rust_decimal::dec;
fn main() {
    let x = Decimal::new(1, 1); // Represents 0.1
    let y = Decimal::new(2, 1); // Represents 0.2
    let result = x + y; // Adds the two decimals
    println!("0.1 + 0.2 = {}", result); // Prints: 0.1 + 0.2 = 0.3

    //Alternative, and easier way, to create Decimal using macros
    let a = dec!(0.1); // Represents 0.1
    let b = dec!(0.2); // Represents 0.2
    let sum = a + b; // Adds the two decimals
    println!("0.1 + 0.2 = {}", sum); // Prints: 0.1 + 0.2 = 0.3
}
