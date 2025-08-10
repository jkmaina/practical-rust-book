// src/cli.rs
// Command-line interface definition using clap
// Defines all commands and their arguments for the TaskRust client

use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Parser)]
#[command(name = "taskrust-client")]
#[command(about = "A command-line client for the TaskRust API")]
pub struct Cli {
    /// API server URL
    #[arg(long, default_value = "http://localhost:8080")]
    pub server: String,
    
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show API information
    Info,
    
    /// List all tasks
    List,
    
    /// Get a specific task by ID
    Get {
        /// Task ID (UUID)
        id: String,
    },
    
    /// Create a new task
    Create {
        /// Task title
        title: String,
        
        /// Task description
        #[arg(short, long)]
        description: Option<String>,
        
        /// Task priority (low, medium, high)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        
        /// Due date in YYYY-MM-DD format
        #[arg(long)]
        due: Option<String>,
    },
    
    /// Update an existing task
    Update {
        /// Task ID (UUID)
        id: String,
        
        /// New title
        #[arg(short, long)]
        title: Option<String>,
        
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        
        /// Mark as completed
        #[arg(short, long)]
        completed: Option<bool>,
        
        /// New priority (low, medium, high)
        #[arg(short, long)]
        priority: Option<String>,
        
        /// New due date in YYYY-MM-DD format
        #[arg(long)]
        due: Option<String>,
    },
    
    /// Delete a task
    Delete {
        /// Task ID (UUID)
        id: String,
    },
    
    /// Mark a task as completed
    Complete {
        /// Task ID (UUID)
        id: String,
    },
}

pub fn parse_uuid(id: &str) -> anyhow::Result<Uuid> {
    Uuid::parse_str(id).map_err(|_| anyhow::anyhow!("Invalid UUID format: {}", id))
}