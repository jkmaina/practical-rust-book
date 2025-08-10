// src/models.rs
// Data models for the TaskRust API
// Defines Task struct, Priority enum, and request/response types
// Uses serde for JSON serialization and UUID for unique identifiers

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: Priority,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: Option<Priority>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: Option<Priority>,
}

impl Task {
    pub fn new(req: CreateTaskRequest) -> Self {
        Task {
            id: Uuid::new_v4(),
            title: req.title,
            description: req.description,
            completed: false,
            created_at: Utc::now(),
            due_date: req.due_date,
            priority: req.priority.unwrap_or(Priority::Medium),
        }
    }
    
    pub fn update(&mut self, req: UpdateTaskRequest) {
        if let Some(title) = req.title {
            self.title = title;
        }
        if let Some(description) = req.description {
            self.description = Some(description);
        }
        if let Some(completed) = req.completed {
            self.completed = completed;
        }
        if let Some(due_date) = req.due_date {
            self.due_date = Some(due_date);
        }
        if let Some(priority) = req.priority {
            self.priority = priority;
        }
    }
    
    pub fn is_due_soon(&self) -> bool {
        if let Some(due) = self.due_date {
            let now = Utc::now();
            let duration = due.signed_duration_since(now);
            duration.num_days() <= 3 && duration.num_seconds() > 0
        } else {
            false
        }
    }
}