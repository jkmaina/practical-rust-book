// src/commands/add.rs
use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate};
use colored::Colorize;
use crate::storage::TaskStore;
use crate::task::Priority;

pub fn execute(description: &str, priority_str: &str, due_date_str: &Option<String>) -> Result<()> {
    let priority = match priority_str.to_lowercase().as_str() {
        "low" => Priority::Low,
        "high" => Priority::High,
        _ => Priority::Medium,
    };
    
    let due_date = if let Some(date_str) = due_date_str {
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
        let naive_datetime = naive_date.and_hms_opt(23, 59, 59).unwrap();
        Some(DateTime::<Local>::from_naive_utc_and_offset(naive_datetime, *Local::now().offset()))
    } else {
        None
    };
    
    let mut store = TaskStore::load()?;
    let task = store.add_task(description.to_string(), priority, due_date);
    store.save()?;
    
    println!("{} {}", "Added task:".green().bold(), task);
    
    Ok(())
}