// src/config.rs
// Application configuration management
// Loads configuration from environment variables with sensible defaults
// Provides centralized configuration for the web service

use std::env;

pub struct Config {
    pub server_addr: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            server_addr: env::var("SERVER_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:8080".to_string()),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        }
    }
    
    pub fn print_info(&self) {
        println!("TaskRust API Configuration:");
        println!("  Server Address: {}", self.server_addr);
        println!("  Log Level: {}", self.log_level);
        println!("  Environment: {}", 
            env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()));
    }
}