// src/repository.rs
// Data access layer for task management
// Provides thread-safe in-memory storage using HashMap with Arc<Mutex<>>
// Implements CRUD operations for tasks

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use crate::models::{Task, CreateTaskRequest, UpdateTaskRequest};

#[derive(Clone)]
pub struct TaskRepository {
    tasks: Arc<Mutex<HashMap<Uuid, Task>>>,
}

impl TaskRepository {
    pub fn new() -> Self {
        TaskRepository {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn create(&self, req: CreateTaskRequest) -> Task {
        let task = Task::new(req);
        let mut tasks = self.tasks.lock().unwrap();
        let task_clone = task.clone();
        tasks.insert(task.id, task);
        task_clone
    }
    
    pub fn get_all(&self) -> Vec<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.values().cloned().collect()
    }
    
    pub fn get_by_id(&self, id: Uuid) -> Option<Task> {
        let tasks = self.tasks.lock().unwrap();
        tasks.get(&id).cloned()
    }
    
    pub fn update(&self, id: Uuid, req: UpdateTaskRequest) -> Option<Task> {
        let mut tasks = self.tasks.lock().unwrap();
        
        if let Some(task) = tasks.get_mut(&id) {
            task.update(req);
            return Some(task.clone());
        }
        
        None
    }
    
    pub fn delete(&self, id: Uuid) -> bool {
        let mut tasks = self.tasks.lock().unwrap();
        tasks.remove(&id).is_some()
    }
    
    // Add some sample data for testing
    pub fn add_sample_data(&self) {
        let sample_tasks = vec![
            CreateTaskRequest {
                title: "Complete Rust web service".to_string(),
                description: Some("Build a RESTful API with Actix Web".to_string()),
                due_date: None,
                priority: Some(crate::models::Priority::High),
            },
            CreateTaskRequest {
                title: "Write API documentation".to_string(),
                description: Some("Document all endpoints and request/response formats".to_string()),
                due_date: None,
                priority: Some(crate::models::Priority::Medium),
            },
            CreateTaskRequest {
                title: "Add unit tests".to_string(),
                description: Some("Test all repository and handler functions".to_string()),
                due_date: None,
                priority: Some(crate::models::Priority::Low),
            },
        ];
        
        for task_req in sample_tasks {
            self.create(task_req);
        }
    }
}