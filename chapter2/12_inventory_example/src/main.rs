// This is a simple Rust program that manages an inventory system.
// It demonstrates the use of constants, mutable and immutable variables, shadowing, and basic arithmetic
// operations.

fn main() {
    // Constant for initial inventory
    const MAX_INVENTORY: u32 = 100;
    
    // Initial inventory items (immutable)
    let product_name = "Widget";
    let product_id = "W-123";
    
    // Current inventory (mutable as it will change)
    let mut inventory_count = 50;
    
    // Display initial state
    println!("Product: {} (ID: {})", product_name, product_id);
    println!("Initial inventory: {}/{}", inventory_count, MAX_INVENTORY);
    
    // Simulate a sale
    let items_sold = 5;
    inventory_count -= items_sold;
    println!("Sold {} items", items_sold);
    println!("Current inventory: {}/{}", inventory_count, MAX_INVENTORY);
    
    // Simulate a delivery
    let items_received = 15;
    inventory_count += items_received;
    println!("Received {} items", items_received);
    println!("Current inventory: {}/{}", inventory_count, MAX_INVENTORY);
    
    // Calculate inventory value (shadowing to transform data)
    let unit_price = 10.99;
    let inventory_value = inventory_count as f64 * unit_price;
    println!("Inventory value: ${:.2}", inventory_value);
    
    // Check if we need to reorder
    const REORDER_THRESHOLD: u32 = 30;
    let needs_reorder = inventory_count < REORDER_THRESHOLD;
    println!("Needs reorder: {}", needs_reorder);
}
