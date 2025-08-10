// src/errors.rs
// Error handling for the TaskRust API
// Defines custom error types and implements ResponseError trait
// Provides consistent JSON error responses

use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Task with ID {0} not found")]
    TaskNotFound(String),
    
    #[error("Invalid request: {0}")]
    BadRequest(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::TaskNotFound(_) => {
                HttpResponse::NotFound().json(ErrorResponse::from(self))
            }
            ApiError::BadRequest(_) => {
                HttpResponse::BadRequest().json(ErrorResponse::from(self))
            }
            ApiError::InternalError(_) => {
                HttpResponse::InternalServerError().json(ErrorResponse::from(self))
            }
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<&ApiError> for ErrorResponse {
    fn from(error: &ApiError) -> Self {
        ErrorResponse {
            error: error.to_string(),
        }
    }
}