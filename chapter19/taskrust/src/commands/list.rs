// src/commands/list.rs
use anyhow::Result;
use colored::Colorize;
use crate::storage::TaskStore;
use crate::task::Priority;

pub fn execute(priority_filter: &Option<String>, due_soon: bool, show_completed: bool) -> Result<()> {
    let store = TaskStore::load()?;
    let tasks = store.get_tasks();
    
    if tasks.is_empty() {
        println!("{}", "No tasks found.".yellow());
        return Ok(());
    }
    
    let mut filtered_tasks = tasks.to_vec();
    
    // Apply priority filter if specified
    if let Some(priority_str) = priority_filter {
        let priority = match priority_str.to_lowercase().as_str() {
            "low" => Priority::Low,
            "high" => Priority::High,
            _ => Priority::Medium,
        };
        
        filtered_tasks.retain(|task| task.priority == priority);
    }
    
    // Filter by due soon if requested
    if due_soon {
        filtered_tasks.retain(|task| task.is_due_soon());
    }
    
    // Filter by completion status
    if !show_completed {
        filtered_tasks.retain(|task| !task.completed);
    }
    
    if filtered_tasks.is_empty() {
        println!("{}", "No tasks match the specified filters.".yellow());
        return Ok(());
    }
    
    println!("{}", "Tasks:".blue().bold());
    for task in filtered_tasks {
        let task_str = format!("{}", task);
        
        if task.completed {
            println!("{}", task_str.dimmed());
        } else if task.is_due_soon() {
            println!("{}", task_str.yellow());
        } else {
            match task.priority {
                Priority::High => println!("{}", task_str.red()),
                Priority::Medium => println!("{}", task_str.white()),
                Priority::Low => println!("{}", task_str.green()),
            }
        }
    }
    
    Ok(())
}