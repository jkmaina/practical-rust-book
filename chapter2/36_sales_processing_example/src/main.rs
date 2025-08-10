// This program demonstrates a simple sales processing example in Rust.
// It includes type inference, formatting, and basic calculations.

fn main() {
    // Sales data for a product
    let product_id: String = String::from("PRD-12345");
    let product_name = "Ergonomic Keyboard"; // Type inference: &str
    
    let price = 79.99; // Type inference: f64
    let in_stock = true; // Type inference: bool
    
    let stock_quantity: u32 = 42;
    let category = 'E'; // Type inference: char (for Electronics)
    
    // Calculate sales tax (explicit type for clarity)
    let tax_rate: f64 = 0.08;
    let tax_amount = price * tax_rate;
    let total_price = price + tax_amount;
    
    // Format currency for display
    let formatted_price = format!("${:.2}", price);
    let formatted_tax = format!("${:.2}", tax_amount);
    let formatted_total = format!("${:.2}", total_price);
    
    // Display product information
    println!("Product: {} ({})", product_name, product_id);
    println!("Category: {}", category);
    println!("Price: {}", formatted_price);
    println!("Tax: {}", formatted_tax);
    println!("Total: {}", formatted_total);
    println!("In stock: {} (Quantity: {})", in_stock, stock_quantity);
    
    // Calculate potential revenue
    let potential_revenue: f64 = price * stock_quantity as f64;
    println!("Potential revenue: ${:.2}", potential_revenue);
    
    // Determine if this is a high-value product
    const HIGH_VALUE_THRESHOLD: f64 = 50.0;
    let is_high_value = price > HIGH_VALUE_THRESHOLD;
    println!("High-value product: {}", is_high_value);
    
    // Parse customer review score
    let review_score: u8 = "4".parse().expect("Invalid review score");
    println!("Customer review score: {}/5", review_score);
}
