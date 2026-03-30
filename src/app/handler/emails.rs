use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::app::{
    dto::email::{CreateEmailRequest, UpdateEmailRequest},
    service::email_service,
    state::AppState,
    ServiceError,
};

pub async fn list_emails(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let emails = email_service::list_emails(&state, person_id).await?;
    Ok(Json(serde_json::json!(emails)))
}

pub async fn add_email(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
    Json(req): Json<CreateEmailRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), ServiceError> {
    let email = email_service::add_email(&state, person_id, req).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!(email))))
}

pub async fn get_email(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let email = email_service::get_email(&state, id).await?;
    Ok(Json(serde_json::json!(email)))
}

pub async fn update_email(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateEmailRequest>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let email = email_service::update_email(&state, id, req).await?;
    Ok(Json(serde_json::json!(email)))
}

pub async fn delete_email(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ServiceError> {
    email_service::delete_email(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
