struct ResourceManager {
    resource_id: String,
}
impl ResourceManager {
    fn new(id: &str) -> ResourceManager {
        println!("Acquiring resource: {}", id);
        ResourceManager {
            resource_id: id.to_string(),
        }
    }
}
impl Drop for ResourceManager {
    fn drop(&mut self) {
        println!("Releasing resource: {}", self.resource_id);
    }
}
fn main() {
    {
        let manager = ResourceManager::new("Database Connection");
        // Use the resource
        println!("Using resource: {}", manager.resource_id);
        // No need to explicitly release the resource
    } // manager goes out of scope here, and drop() is called automatically
    
    println!("Resource has been released");
    
    let manager1 = ResourceManager::new("File Handle");
    let manager2 = manager1; // Ownership moves from manager1 to manager2
    
    // println!("Using resource: {}", manager1.resource_id); // Error: manager1 is no longer valid
    println!("Using resource: {}", manager2.resource_id); // This works fine
    
    // Resource will be released when manager2 goes out of scope
}
