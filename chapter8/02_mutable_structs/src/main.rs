    
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    user1.sign_in_count += 1;
    println!("User's email is: {}", user1.email);
    println!("User's username is: {}", user1.username);
    println!("User's sign in count is: {}", user1.sign_in_count);
    println!("User's active status is: {}", user1.active);
}
