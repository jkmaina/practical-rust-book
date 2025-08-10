// Mock testing example demonstrating dependency injection and test doubles
// Shows how to use traits to create testable code with mock implementations
// Enables testing business logic without external dependencies like databases
// Demonstrates the dependency inversion principle for better testability

pub trait Database {
    fn get_user(&self, id: u32) -> Option<String>;
}
pub struct UserService<T: Database> {
    database: T,
}
impl<T: Database> UserService<T> {
    pub fn new(database: T) -> Self {
        UserService { database }
    }
    pub fn get_user_name(&self, id: u32) -> String {
        match self.database.get_user(id) {
            Some(name) => name,
            None => String::from("User not found"),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    struct MockDatabase {
        users: Vec<(u32, String)>,
    }
    impl Database for MockDatabase {
        fn get_user(&self, id: u32) -> Option<String> {
            self.users.iter()
                .find(|(user_id, _)| *user_id == id)
                .map(|(_, name)| name.clone())
        }
    }
    #[test]
    fn test_get_user_name_existing() {
        let mock_db = MockDatabase {
            users: vec![(1, String::from("Alice")), (2, String::from("Bob"))],
        };
        let service = UserService::new(mock_db);
        assert_eq!(service.get_user_name(1), "Alice");
    }
    #[test]
    fn test_get_user_name_nonexistent() {
        let mock_db = MockDatabase {
            users: vec![(1, String::from("Alice")), (2, String::from("Bob"))],
        };
        let service = UserService::new(mock_db);
        assert_eq!(service.get_user_name(3), "User not found");
    }
}
