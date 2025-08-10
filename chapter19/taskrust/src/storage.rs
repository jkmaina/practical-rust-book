// src/storage.rs
use crate::task::{Task, Priority};
use anyhow::Result;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to read storage file: {0}")]
    ReadError(#[from] std::io::Error),
    
    #[error("Failed to parse storage data: {0}")]
    ParseError(String),
    
    #[error("Failed to serialize data: {0}")]
    SerializeError(String),
    
    #[error("Task with ID {0} not found")]
    TaskNotFound(usize),
}
const STORAGE_FILE: &str = "tasks.json";
#[derive(Serialize, Deserialize, Default)]
pub struct TaskStore {
    tasks: Vec<Task>,
    next_id: usize,
}
impl TaskStore {
    pub fn load() -> Result<Self> {
        if !Path::new(STORAGE_FILE).exists() {
            return Ok(TaskStore::default());
        }
        
        let mut file = File::open(STORAGE_FILE)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        let store: TaskStore = serde_json::from_str(&contents)
            .map_err(|e| StorageError::ParseError(e.to_string()))?;
            
        Ok(store)
    }
    
    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| StorageError::SerializeError(e.to_string()))?;
            
        let mut file = File::create(STORAGE_FILE)?;
        file.write_all(json.as_bytes())?;
        
        Ok(())
    }
    
    pub fn add_task(&mut self, description: String, priority: Priority, due_date: Option<DateTime<Local>>) -> Task {
        let id = self.next_id;
        self.next_id += 1;
        
        let task = Task::new(id, description, priority, due_date);
        self.tasks.push(task.clone());
        
        task
    }
    
    pub fn get_tasks(&self) -> &[Task] {
        &self.tasks
    }
    
    pub fn get_task(&self, id: usize) -> Result<&Task> {
        self.tasks.iter()
            .find(|t| t.id == id)
            .ok_or_else(|| StorageError::TaskNotFound(id).into())
    }
    
    pub fn complete_task(&mut self, id: usize) -> Result<()> {
        let task = self.tasks.iter_mut()
            .find(|t| t.id == id)
            .ok_or(StorageError::TaskNotFound(id))?;
            
        task.mark_completed();
        Ok(())
    }
    
    pub fn delete_task(&mut self, id: usize) -> Result<()> {
        let position = self.tasks.iter()
            .position(|t| t.id == id)
            .ok_or(StorageError::TaskNotFound(id))?;
            
        self.tasks.remove(position);
        Ok(())
    }
    
    pub fn export_tasks<P: AsRef<Path>>(&self, path: P, format: &str) -> Result<()> {
        match format.to_lowercase().as_str() {
            "json" => {
                let json = serde_json::to_string_pretty(&self.tasks)
                    .map_err(|e| StorageError::SerializeError(e.to_string()))?;
                fs::write(path, json)?;
            },
            "csv" => {
                let file = File::create(path)?;
                let mut writer = csv::Writer::from_writer(file);
                
                for task in &self.tasks {
                    writer.serialize(task)
                        .map_err(|e| StorageError::SerializeError(e.to_string()))?;
                }
                
                writer.flush()?;
            },
            _ => return Err(StorageError::ParseError(format!("Unsupported format: {}", format)).into()),
        }
        
        Ok(())
    }
    
    pub fn import_tasks<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .ok_or_else(|| StorageError::ParseError("Unknown file format".to_string()))?;
            
        match extension.to_lowercase().as_str() {
            "json" => {
                let content = fs::read_to_string(path)?;
                let imported_tasks: Vec<Task> = serde_json::from_str(&content)
                    .map_err(|e| StorageError::ParseError(e.to_string()))?;
                    
                for task in imported_tasks {
                    if self.next_id <= task.id {
                        self.next_id = task.id + 1;
                    }
                    self.tasks.push(task);
                }
            },
            "csv" => {
                let file = File::open(path)?;
                let mut reader = csv::Reader::from_reader(file);
                
                for result in reader.deserialize() {
                    let task: Task = result
                        .map_err(|e| StorageError::ParseError(e.to_string()))?;
                        
                    if self.next_id <= task.id {
                        self.next_id = task.id + 1;
                    }
                    self.tasks.push(task);
                }
            },
            _ => return Err(StorageError::ParseError(format!("Unsupported format: {}", extension)).into()),
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Priority;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_add_task() {
        let mut store = TaskStore::default();
        let task = store.add_task("Test task".to_string(), Priority::Medium, None);
        
        assert_eq!(task.id, 0);
        assert_eq!(task.description, "Test task");
        assert_eq!(store.get_tasks().len(), 1);
        assert_eq!(store.next_id, 1);
    }
    
    #[test]
    fn test_complete_task() {
        let mut store = TaskStore::default();
        store.add_task("Test task".to_string(), Priority::Medium, None);
        
        assert!(!store.get_tasks()[0].completed);
        
        store.complete_task(0).unwrap();
        assert!(store.get_tasks()[0].completed);
    }
    
    #[test]
    fn test_delete_task() {
        let mut store = TaskStore::default();
        store.add_task("Task 1".to_string(), Priority::Medium, None);
        store.add_task("Task 2".to_string(), Priority::Medium, None);
        
        assert_eq!(store.get_tasks().len(), 2);
        
        store.delete_task(0).unwrap();
        assert_eq!(store.get_tasks().len(), 1);
        assert_eq!(store.get_tasks()[0].description, "Task 2");
    }
    
    #[test]
    fn test_export_import_json() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        let file_path_str = file_path.to_str().unwrap();
        
        // Create and export tasks
        let mut store = TaskStore::default();
        store.add_task("Task 1".to_string(), Priority::Low, None);
        store.add_task("Task 2".to_string(), Priority::High, None);
        
        store.export_tasks(file_path_str, "json").unwrap();
        
        // Import into a new store
        let mut new_store = TaskStore::default();
        new_store.import_tasks(file_path_str).unwrap();
        
        assert_eq!(new_store.get_tasks().len(), 2);
        assert_eq!(new_store.get_tasks()[0].description, "Task 1");
        assert_eq!(new_store.get_tasks()[1].description, "Task 2");
        assert_eq!(new_store.get_tasks()[0].priority, Priority::Low);
        assert_eq!(new_store.get_tasks()[1].priority, Priority::High);
    }
}
 
