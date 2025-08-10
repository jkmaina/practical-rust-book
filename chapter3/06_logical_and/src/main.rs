// This Rust program demonstrates the use of logical AND (`&&`) in an `if` statement.
// It checks multiple conditions to determine if the weather is suitable for a picnic.
fn main() {
    let temperature = 25;
    let is_sunny = true;
    
    if temperature > 20 && is_sunny {
        println!("It's warm and sunny - perfect for a picnic!");
    }
}
