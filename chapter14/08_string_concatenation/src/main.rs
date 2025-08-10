
// String concatenation methods: format!, join(), and manual building

fn main() {
    // Using format! macro for interpolation
    let name = "Alice";
    let age = 30;
    let greeting = format!("Hello, my name is {} and I am {} years old.", name, age);
    println!("{}", greeting);
    
    // Manual string building with loop
    let items = vec!["apple", "banana", "cherry"];
    let mut list = String::new();
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            list.push_str(", ");
        }
        list.push_str(item);
    }
    println!("Shopping list (manual): {}", list);
    
    // Using join() method (more idiomatic)
    let list2 = items.join(", ");
    println!("Shopping list (join): {}", list2);
    
    // Multiple format! examples
    let formatted = format!("Items: [{}] - Total: {}", items.join(", "), items.len());
    println!("{}", formatted);
}
