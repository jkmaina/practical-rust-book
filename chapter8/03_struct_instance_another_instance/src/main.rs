    
// This program demonstrates how to create a struct instance and then create another instance
// using the values from the first instance, while overriding some fields.
// It prints the details of the second user instance.
// The second user instance will have the same username and sign-in count as the first,
// but a different email.
// The active status will also remain the same.
// This is useful for creating variations of a struct instance without repeating all fields.
// The `..` syntax allows us to use the values from the first instance for the fields
// that are not explicitly set in the second instance.
// This is a common pattern in Rust for creating new instances based on existing ones.
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    
    println!("User 2's email is: {}", user2.email);    
    println!("User 2's username is: {}", user2.username);
    println!("User 2's sign in count is: {}", user2.sign_in_count);
    println!("User 2's active status is: {}", user2.active);
}
