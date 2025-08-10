// This program demonstrates a builder function in Rust for creating a User struct.
// The `build_user` function makes it easy to create a new User with default values for some fields.

struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

// Builder function to create a new User
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // Create a new user using the builder function
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // Print user details
    println!(
        "User: {}, Email: {}, Active: {}, Sign-ins: {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );
}
