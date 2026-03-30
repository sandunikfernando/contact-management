pub mod dto;
pub mod entity;       
pub mod handler;
pub mod repository;
pub mod route;
pub mod service;
pub mod state;


// IntoResponse lets Axum convert it directly into an HTTP response.
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ServiceError::NotFound(msg)      => (StatusCode::NOT_FOUND, msg.clone()),
            ServiceError::BadRequest(msg)    => (StatusCode::BAD_REQUEST, msg.clone()),
            ServiceError::Conflict(msg)      => (StatusCode::CONFLICT, msg.clone()),
            ServiceError::InternalError(msg) => {
                // Log internally but return a generic message to the client
                eprintln!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
