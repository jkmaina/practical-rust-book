// examples/demo.rs
// This example demonstrates the TaskRust functionality programmatically

use taskrust::storage::TaskStore;
use taskrust::task::Priority;
use chrono::{Local, Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("TaskRust Demo");
    println!("=============");
    
    // Create a new task store
    let mut store = TaskStore::default();
    
    // Add some sample tasks
    let due_date = Some(Local::now() + Duration::days(2));
    store.add_task("Complete Rust project".to_string(), Priority::High, due_date);
    store.add_task("Review code".to_string(), Priority::Medium, None);
    store.add_task("Update documentation".to_string(), Priority::Low, None);
    
    // Display all tasks
    println!("\nAll tasks:");
    for task in store.get_tasks() {
        println!("  {}", task);
    }
    
    // Complete a task
    store.complete_task(1)?;
    println!("\nAfter completing task 1:");
    for task in store.get_tasks() {
        println!("  {}", task);
    }
    
    // Show only incomplete tasks
    println!("\nIncomplete tasks:");
    for task in store.get_tasks() {
        if !task.completed {
            println!("  {}", task);
        }
    }
    
    Ok(())
}