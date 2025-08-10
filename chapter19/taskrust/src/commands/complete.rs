// src/commands/complete.rs
use anyhow::Result;
use colored::Colorize;
use std::io::{self, Write};
use crate::storage::TaskStore;

pub fn execute(id: usize) -> Result<()> {
    let mut store = TaskStore::load()?;
    
    // Check if we're in test mode (skip confirmation)
    if std::env::var("TASKRUST_TEST").is_ok() {
        store.complete_task(id)?;
        store.save()?;
        println!("{} {}", "Completed task".green().bold(), id);
        return Ok(());
    }
    
    // Check if task exists before asking for confirmation
    let task = store.get_task(id)?;
    
    print!("{} {} \"{}\"? [y/N] ", "Complete task".yellow(), id, task.description);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() == "y" {
        store.complete_task(id)?;
        store.save()?;
        println!("{} {}", "Completed task".green().bold(), id);
    } else {
        println!("{}", "Task completion cancelled.".yellow());
    }
    
    Ok(())
}