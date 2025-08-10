// src/task.rs
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the priority level of a task
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}

/// Represents a task with its attributes
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    /// Unique identifier for the task
    pub id: usize,
    
    /// Description of the task
    pub description: String,
    
    /// Whether the task has been completed
    pub completed: bool,
    
    /// When the task was created
    pub created_at: DateTime<Local>,
    
    /// Optional due date for the task
    pub due_date: Option<DateTime<Local>>,
    
    /// Priority level of the task
    pub priority: Priority,
}

impl Task {
    /// Creates a new task with the given attributes
    pub fn new(id: usize, description: String, priority: Priority, due_date: Option<DateTime<Local>>) -> Self {
        Task {
            id,
            description,
            completed: false,
            created_at: Local::now(),
            due_date,
            priority,
        }
    }
    
    /// Checks if the task is due within the next 3 days
    pub fn is_due_soon(&self) -> bool {
        if let Some(due) = self.due_date {
            let now = Local::now();
            let duration = due.signed_duration_since(now);
            duration.num_days() <= 3 && duration.num_seconds() > 0
        } else {
            false
        }
    }
    
    /// Marks the task as completed
    pub fn mark_completed(&mut self) {
        self.completed = true;
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✓" } else { "☐" };
        
        let due_str = match self.due_date {
            Some(date) => {
                let now = Local::now();
                let days_left = date.signed_duration_since(now).num_days();
                
                if days_left < 0 {
                    format!("Overdue by {} days", -days_left)
                } else if days_left == 0 {
                    "Due today".to_string()
                } else if days_left == 1 {
                    "Due tomorrow".to_string()
                } else {
                    format!("Due in {} days", days_left)
                }
            },
            None => "No due date".to_string(),
        };
        
        write!(
            f,
            "{} [{}] {} (Priority: {}, {})",
            status, self.id, self.description, self.priority, due_str
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    
    #[test]
    fn test_new_task() {
        let task = Task::new(1, "Test task".to_string(), Priority::Medium, None);
        
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Test task");
        assert_eq!(task.completed, false);
        assert_eq!(task.priority, Priority::Medium);
        assert!(task.due_date.is_none());
    }
    
    #[test]
    fn test_mark_completed() {
        let mut task = Task::new(1, "Test task".to_string(), Priority::Medium, None);
        assert_eq!(task.completed, false);
        
        task.mark_completed();
        assert_eq!(task.completed, true);
    }
    
    #[test]
    fn test_is_due_soon() {
        let now = Local::now();
        
        // Task due tomorrow should be "due soon"
        let tomorrow = now + Duration::days(1);
        let task = Task::new(1, "Due soon".to_string(), Priority::Medium, Some(tomorrow));
        assert!(task.is_due_soon());
        
        // Task due in 5 days should not be "due soon"
        let future = now + Duration::days(5);
        let task = Task::new(2, "Not due soon".to_string(), Priority::Medium, Some(future));
        assert!(!task.is_due_soon());
        
        // Task with no due date should not be "due soon"
        let task = Task::new(3, "No due date".to_string(), Priority::Medium, None);
        assert!(!task.is_due_soon());
    }
    
    #[test]
    fn test_display() {
        let task = Task::new(1, "Test task".to_string(), Priority::High, None);
        let display = format!("{}", task);
        
        assert!(display.contains("Test task"));
        assert!(display.contains("Priority: high"));
    }
}