// TaskRust - A command-line task manager built with Rust
// Demonstrates CLI application development using clap for argument parsing
// Features task management with priorities, due dates, and persistence
// Shows modular Rust project structure with separate modules for different concerns

use clap::{Parser, Subcommand};
use anyhow::Result;
mod task;
mod storage;
mod commands;
#[derive(Parser)]
#[command(name = "taskrust")]
#[command(about = "A Rust-powered task manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add {
        /// Task description
        description: String,
        
        /// Task priority (low, medium, high)
        #[arg(short, long, default_value = "medium")]
        priority: String,
        
        /// Due date in YYYY-MM-DD format
        #[arg(short, long)]
        due: Option<String>,
    },
    
    /// List all tasks
    List {
        /// Filter by priority
        #[arg(short, long)]
        priority: Option<String>,
        
        /// Show only tasks due within 3 days
        #[arg(long)]
        due_soon: bool,
        
        /// Show completed tasks
        #[arg(short, long)]
        completed: bool,
    },
    
    /// Mark a task as completed
    Complete {
        /// Task ID
        id: usize,
    },
    
    /// Delete a task
    Delete {
        /// Task ID
        id: usize,
    },
    
    /// Export tasks to a file
    Export {
        /// File path
        file: String,
    },
    
    /// Import tasks from a file
    Import {
        /// File path
        file: String,
    },
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Add { description, priority, due } => {
            commands::add::execute(description, priority, due)?;
        }
        Commands::List { priority, due_soon, completed } => {
            commands::list::execute(priority, *due_soon, *completed)?;
        }
        Commands::Complete { id } => {
            commands::complete::execute(*id)?;
        }
        Commands::Delete { id } => {
            commands::delete::execute(*id)?;
        }
        Commands::Export { file } => {
            commands::export::execute(file)?;
        }
        Commands::Import { file } => {
            commands::import::execute(file)?;
        }
    }
    
    Ok(())
}