// src/commands/export.rs
use anyhow::Result;
use colored::Colorize;
use std::path::Path;
use crate::storage::TaskStore;

pub fn execute(file_path: &str) -> Result<()> {
    let store = TaskStore::load()?;
    
    let path = Path::new(file_path);
    let format = path.extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("json");
        
    store.export_tasks(file_path, format)?;
    
    println!("{} {}", "Exported tasks to".green().bold(), file_path);
    
    Ok(())
}