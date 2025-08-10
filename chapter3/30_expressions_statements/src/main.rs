fn is_even(n: i32) -> bool {
    // This is an expression (no semicolon)
    n % 2 == 0
}
fn describe_number(n: i32) -> String {
    // Using if as an expression
    let _description = if is_even(n) {
        "even"
    } else {
        "odd"
    };
    
    // Using match as an expression
    let magnitude = match n {
        0 => "zero",
        1..=10 => "small",
        11..=100 => "medium",
        _ => "large",
    };
    
    // Using a block as an expression
    let sign = {
        if n < 0 {
            "negative"
        } else if n > 0 {
            "positive"
        } else {
            "neither positive nor negative"
        }
    };
    
    // Format the final string
    format!("{} is a {} {} number", n, sign, magnitude)
}
fn main() {
    println!("{}", describe_number(42));  // Prints: 42 is a positive medium number
    println!("{}", describe_number(-5));  // Prints: -5 is a negative small number
    println!("{}", describe_number(0));   // Prints: 0 is a neither positive nor negative zero number
}