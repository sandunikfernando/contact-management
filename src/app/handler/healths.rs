use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn livez() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "alive" })))
}

pub async fn readyz() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({ "status": "ready" })))
}
