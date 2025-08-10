// src/commands/import.rs
use anyhow::Result;
use colored::Colorize;
use crate::storage::TaskStore;

pub fn execute(file_path: &str) -> Result<()> {
    let mut store = TaskStore::load()?;
    store.import_tasks(file_path)?;
    store.save()?;
    
    println!("{} {}", "Imported tasks from".green().bold(), file_path);
    
    Ok(())
}