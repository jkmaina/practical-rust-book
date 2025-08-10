// src/handlers.rs
// HTTP request handlers for the TaskRust API
// Implements RESTful endpoints for task management
// Handles JSON serialization/deserialization and error responses

use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::errors::ApiError;
use crate::models::{CreateTaskRequest, UpdateTaskRequest};
use crate::repository::TaskRepository;

/// GET /tasks - List all tasks
pub async fn get_tasks(repo: web::Data<TaskRepository>) -> impl Responder {
    let tasks = repo.get_all();
    HttpResponse::Ok().json(tasks)
}

/// GET /tasks/{id} - Get a specific task by ID
pub async fn get_task(
    path: web::Path<String>,
    repo: web::Data<TaskRepository>,
) -> Result<HttpResponse, ApiError> {
    let id = parse_uuid(path.into_inner())?;
    
    match repo.get_by_id(id) {
        Some(task) => Ok(HttpResponse::Ok().json(task)),
        None => Err(ApiError::TaskNotFound(id.to_string())),
    }
}

/// POST /tasks - Create a new task
pub async fn create_task(
    req: web::Json<CreateTaskRequest>,
    repo: web::Data<TaskRepository>,
) -> impl Responder {
    let task = repo.create(req.into_inner());
    HttpResponse::Created().json(task)
}

/// PUT /tasks/{id} - Update an existing task
pub async fn update_task(
    path: web::Path<String>,
    req: web::Json<UpdateTaskRequest>,
    repo: web::Data<TaskRepository>,
) -> Result<HttpResponse, ApiError> {
    let id = parse_uuid(path.into_inner())?;
    
    match repo.update(id, req.into_inner()) {
        Some(task) => Ok(HttpResponse::Ok().json(task)),
        None => Err(ApiError::TaskNotFound(id.to_string())),
    }
}

/// DELETE /tasks/{id} - Delete a task
pub async fn delete_task(
    path: web::Path<String>,
    repo: web::Data<TaskRepository>,
) -> Result<HttpResponse, ApiError> {
    let id = parse_uuid(path.into_inner())?;
    
    if repo.delete(id) {
        Ok(HttpResponse::NoContent().finish())
    } else {
        Err(ApiError::TaskNotFound(id.to_string()))
    }
}

/// Helper function to parse UUID from string
fn parse_uuid(id: String) -> Result<Uuid, ApiError> {
    Uuid::parse_str(&id).map_err(|_| {
        ApiError::BadRequest(format!("Invalid UUID: {}", id))
    })
}

/// GET / - API information endpoint
pub async fn api_info() -> impl Responder {
    let info = serde_json::json!({
        "name": "TaskRust API",
        "version": "0.1.0",
        "description": "A RESTful API for task management built with Rust and Actix Web",
        "endpoints": {
            "GET /": "API information",
            "GET /tasks": "List all tasks",
            "GET /tasks/{id}": "Get a specific task",
            "POST /tasks": "Create a new task",
            "PUT /tasks/{id}": "Update a task",
            "DELETE /tasks/{id}": "Delete a task"
        }
    });
    
    HttpResponse::Ok().json(info)
}