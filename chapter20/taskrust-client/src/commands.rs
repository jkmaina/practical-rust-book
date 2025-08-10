// src/commands.rs
// Command implementations for the TaskRust client
// Handles all user commands and formats output appropriately

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use colored::Colorize;
use uuid::Uuid;
use crate::client::TaskClient;
use crate::models::{CreateTaskRequest, UpdateTaskRequest, Priority};

pub async fn handle_info(client: &TaskClient) -> Result<()> {
    println!("{}", "📡 Fetching API information...".cyan());
    
    let info = client.get_api_info().await?;
    println!("{}", "✅ API Information:".green().bold());
    println!("{}", serde_json::to_string_pretty(&info)?);
    
    Ok(())
}

pub async fn handle_list(client: &TaskClient) -> Result<()> {
    println!("{}", "📋 Fetching tasks...".cyan());
    
    let tasks = client.list_tasks().await?;
    
    if tasks.is_empty() {
        println!("{}", "No tasks found.".yellow());
        return Ok(());
    }
    
    println!("{}", format!("Found {} tasks:", tasks.len()).green().bold());
    println!();
    
    for task in tasks {
        let color = match task.priority {
            Priority::High => "red",
            Priority::Medium => "yellow", 
            Priority::Low => "green",
        };
        
        if task.completed {
            println!("{}", format!("{}", task).dimmed());
        } else {
            println!("{}", format!("{}", task).color(color));
        }
    }
    
    Ok(())
}

pub async fn handle_get(client: &TaskClient, id: Uuid) -> Result<()> {
    println!("{}", format!("🔍 Fetching task {}...", id).cyan());
    
    let task = client.get_task(id).await?;
    
    println!("{}", "✅ Task details:".green().bold());
    println!("  ID: {}", task.id);
    println!("  Title: {}", task.title.bold());
    println!("  Description: {}", task.description.as_deref().unwrap_or("None"));
    println!("  Completed: {}", if task.completed { "✅ Yes".green() } else { "⏳ No".yellow() });
    println!("  Priority: {}", format!("{}", task.priority).color(match task.priority {
        Priority::High => "red",
        Priority::Medium => "yellow",
        Priority::Low => "green",
    }));
    println!("  Created: {}", task.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
    if let Some(due) = task.due_date {
        println!("  Due: {}", due.format("%Y-%m-%d %H:%M:%S UTC"));
    }
    
    Ok(())
}

pub async fn handle_create(
    client: &TaskClient,
    title: String,
    description: Option<String>,
    priority: String,
    due: Option<String>,
) -> Result<()> {
    println!("{}", "➕ Creating new task...".cyan());
    
    let priority: Priority = priority.parse()
        .map_err(|e| anyhow::anyhow!("Invalid priority: {}", e))?;
    
    let due_date = if let Some(due_str) = due {
        let naive_date = NaiveDate::parse_from_str(&due_str, "%Y-%m-%d")
            .map_err(|_| anyhow::anyhow!("Invalid date format. Use YYYY-MM-DD"))?;
        Some(DateTime::<Utc>::from_naive_utc_and_offset(
            naive_date.and_hms_opt(23, 59, 59).unwrap(),
            Utc,
        ))
    } else {
        None
    };
    
    let request = CreateTaskRequest {
        title: title.clone(),
        description,
        due_date,
        priority: Some(priority),
    };
    
    let task = client.create_task(request).await?;
    
    println!("{}", "✅ Task created successfully:".green().bold());
    println!("  ID: {}", task.id);
    println!("  Title: {}", task.title.bold());
    
    Ok(())
}

pub async fn handle_update(
    client: &TaskClient,
    id: Uuid,
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
    priority: Option<String>,
    due: Option<String>,
) -> Result<()> {
    println!("{}", format!("📝 Updating task {}...", id).cyan());
    
    let priority = if let Some(p) = priority {
        Some(p.parse().map_err(|e| anyhow::anyhow!("Invalid priority: {}", e))?)
    } else {
        None
    };
    
    let due_date = if let Some(due_str) = due {
        let naive_date = NaiveDate::parse_from_str(&due_str, "%Y-%m-%d")
            .map_err(|_| anyhow::anyhow!("Invalid date format. Use YYYY-MM-DD"))?;
        Some(DateTime::<Utc>::from_naive_utc_and_offset(
            naive_date.and_hms_opt(23, 59, 59).unwrap(),
            Utc,
        ))
    } else {
        None
    };
    
    let request = UpdateTaskRequest {
        title,
        description,
        completed,
        due_date,
        priority,
    };
    
    let task = client.update_task(id, request).await?;
    
    println!("{}", "✅ Task updated successfully:".green().bold());
    println!("{}", task);
    
    Ok(())
}

pub async fn handle_delete(client: &TaskClient, id: Uuid) -> Result<()> {
    println!("{}", format!("🗑️  Deleting task {}...", id).cyan());
    
    client.delete_task(id).await?;
    
    println!("{}", "✅ Task deleted successfully.".green().bold());
    
    Ok(())
}

pub async fn handle_complete(client: &TaskClient, id: Uuid) -> Result<()> {
    println!("{}", format!("✅ Marking task {} as completed...", id).cyan());
    
    let request = UpdateTaskRequest {
        title: None,
        description: None,
        completed: Some(true),
        due_date: None,
        priority: None,
    };
    
    let task = client.update_task(id, request).await?;
    
    println!("{}", "✅ Task marked as completed:".green().bold());
    println!("{}", task);
    
    Ok(())
}