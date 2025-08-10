#[derive(Debug)]
struct User {
    id: u64,
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}
impl User {
    fn new(id: u64, username: String, email: String) -> User {
        User {
            id,
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }
    fn change_email(&mut self, new_email: String) {
        self.email = new_email;
    }
}

#[derive(Debug)]
struct InventoryItem {
    id: u64,
    name: String,
    price: f64,
    quantity: u32,
    category: String,
}
impl InventoryItem {
    fn new(id: u64, name: String, price: f64, quantity: u32, category: String) -> InventoryItem {
        InventoryItem {
            id,
            name,
            price,
            quantity,
            category,
        }
    }
    fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
    fn is_in_stock(&self) -> bool {
        self.quantity > 0
    }
    fn restock(&mut self, amount: u32) {
        self.quantity += amount;
    }
    fn sell(&mut self, amount: u32) -> Result<(), String> {
        if amount <= self.quantity {
            self.quantity -= amount;
            Ok(())
        } else {
            Err(format!("Not enough stock. Only {} available.", self.quantity))
        }
    }
}

#[derive(Debug)]
struct Store {
    users: Vec<User>,
    inventory: Vec<InventoryItem>,
}
impl Store {
    fn new() -> Store {
        Store {
            users: Vec::new(),
            inventory: Vec::new(),
        }
    }
    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    fn add_item(&mut self, item: InventoryItem) {
        self.inventory.push(item);
    }
    fn find_user_by_id(&self, id: u64) -> Option<&User> {
        self.users.iter().find(|user| user.id == id)
    }
    fn find_item_by_id(&self, id: u64) -> Option<&InventoryItem> {
        self.inventory.iter().find(|item| item.id == id)
    }
    fn total_inventory_value(&self) -> f64 {
        self.inventory.iter().map(|item| item.total_value()).sum()
    }
}

fn main() {
    // Create a new store
    let mut store = Store::new();
    // Add some users
    let mut user1 = User::new(
        1,
        String::from("john_doe"),
        String::from("john@example.com"),
    );
    let user2 = User::new(
        2,
        String::from("jane_smith"),
        String::from("jane@example.com"),
    );
    // Add some inventory items
    let mut item1 = InventoryItem::new(
        101,
        String::from("Laptop"),
        999.99,
        10,
        String::from("Electronics"),
    );
    let mut item2 = InventoryItem::new(
        102,
        String::from("Headphones"),
        99.99,
        20,
        String::from("Electronics"),
    );
    let item3 = InventoryItem::new(
        103,
        String::from("Book"),
        19.99,
        50,
        String::from("Books"),
    );
    // Modify some data
    user1.increment_sign_in_count();
    item1.restock(5);
    if let Err(e) = item2.sell(5) {
        println!("Error: {}", e);
    }
    // Add everything to the store
    store.add_user(user1);
    store.add_user(user2);
    store.add_item(item1);
    store.add_item(item2);
    store.add_item(item3);
    // Calculate total inventory value
    let total_value = store.total_inventory_value();
    println!("Total inventory value: ${:.2}", total_value);
    // Find a user and an item
    if let Some(user) = store.find_user_by_id(1) {
        println!("Found user: {:?}", user);
    }
    if let Some(item) = store.find_item_by_id(102) {
        println!("Found item: {:?}", item);
        println!("In stock: {}", item.is_in_stock());
    }
}
