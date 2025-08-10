// src/client.rs
// HTTP client for communicating with the TaskRust API
// Handles all REST operations with proper error handling and JSON serialization

use anyhow::{anyhow, Result};
use reqwest::Client;
use uuid::Uuid;
use crate::models::{Task, CreateTaskRequest, UpdateTaskRequest};

pub struct TaskClient {
    client: Client,
    base_url: String,
}

impl TaskClient {
    pub fn new(base_url: String) -> Self {
        TaskClient {
            client: Client::new(),
            base_url,
        }
    }
    
    pub async fn get_api_info(&self) -> Result<serde_json::Value> {
        let url = format!("{}/", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow!("Failed to get API info: {}", response.status()))
        }
    }
    
    pub async fn list_tasks(&self) -> Result<Vec<Task>> {
        let url = format!("{}/tasks", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow!("Failed to list tasks: {}", response.status()))
        }
    }
    
    pub async fn get_task(&self, id: Uuid) -> Result<Task> {
        let url = format!("{}/tasks/{}", self.base_url, id);
        let response = self.client.get(&url).send().await?;
        
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            404 => Err(anyhow!("Task not found")),
            _ => Err(anyhow!("Failed to get task: {}", response.status())),
        }
    }
    
    pub async fn create_task(&self, request: CreateTaskRequest) -> Result<Task> {
        let url = format!("{}/tasks", self.base_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(anyhow!("Failed to create task: {}", response.status()))
        }
    }
    
    pub async fn update_task(&self, id: Uuid, request: UpdateTaskRequest) -> Result<Task> {
        let url = format!("{}/tasks/{}", self.base_url, id);
        let response = self.client
            .put(&url)
            .json(&request)
            .send()
            .await?;
        
        match response.status().as_u16() {
            200 => Ok(response.json().await?),
            404 => Err(anyhow!("Task not found")),
            _ => Err(anyhow!("Failed to update task: {}", response.status())),
        }
    }
    
    pub async fn delete_task(&self, id: Uuid) -> Result<()> {
        let url = format!("{}/tasks/{}", self.base_url, id);
        let response = self.client.delete(&url).send().await?;
        
        match response.status().as_u16() {
            204 => Ok(()),
            404 => Err(anyhow!("Task not found")),
            _ => Err(anyhow!("Failed to delete task: {}", response.status())),
        }
    }
}