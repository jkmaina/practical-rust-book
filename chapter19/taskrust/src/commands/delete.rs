// src/commands/delete.rs
use anyhow::Result;
use colored::Colorize;
use crate::storage::TaskStore;

pub fn execute(id: usize) -> Result<()> {
    let mut store = TaskStore::load()?;
    store.delete_task(id)?;
    store.save()?;
    
    println!("{} {}", "Deleted task".red().bold(), id);
    
    Ok(())
}