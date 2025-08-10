// TaskRust Client - Command-line interface for the TaskRust API
// Demonstrates HTTP client implementation and REST API consumption in Rust
// Provides user-friendly CLI for all task management operations

mod models;
mod client;
mod cli;
mod commands;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use cli::{Cli, Commands, parse_uuid};
use client::TaskClient;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = TaskClient::new(cli.server.clone());
    
    println!("{}", "🚀 TaskRust Client".blue().bold());
    println!("{}", format!("📡 Server: {}", cli.server).dimmed());
    println!();
    
    let result = match cli.command {
        Commands::Info => commands::handle_info(&client).await,
        
        Commands::List => commands::handle_list(&client).await,
        
        Commands::Get { id } => {
            let uuid = parse_uuid(&id)?;
            commands::handle_get(&client, uuid).await
        },
        
        Commands::Create { title, description, priority, due } => {
            commands::handle_create(&client, title, description, priority, due).await
        },
        
        Commands::Update { id, title, description, completed, priority, due } => {
            let uuid = parse_uuid(&id)?;
            commands::handle_update(&client, uuid, title, description, completed, priority, due).await
        },
        
        Commands::Delete { id } => {
            let uuid = parse_uuid(&id)?;
            commands::handle_delete(&client, uuid).await
        },
        
        Commands::Complete { id } => {
            let uuid = parse_uuid(&id)?;
            commands::handle_complete(&client, uuid).await
        },
    };
    
    if let Err(e) = result {
        eprintln!("{} {}", "❌ Error:".red().bold(), e);
        std::process::exit(1);
    }
    
    Ok(())
}