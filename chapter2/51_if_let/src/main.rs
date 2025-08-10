use std::io;
fn main() {
    let mut input = String::new();
    
    if let Err(error) = io::stdin().read_line(&mut input) {
        println!("Error: {}", error);
        return;
    }
    
    println!("You entered: {}", input);
}
