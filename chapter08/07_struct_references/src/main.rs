// This program defines a struct `User` with fields that are references.
// The struct is initialized with values, demonstrating how to use references in structs.
    
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
