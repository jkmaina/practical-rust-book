fn greet(name: &str, greeting: Option<&str>) {
    let greeting = greeting.unwrap_or("Hello");
    println!("{}, {}!", greeting, name);
}
fn main() {
    greet("Alice", Some("Hi"));  // Custom greeting
    greet("Bob", None);          // Default greeting
}
