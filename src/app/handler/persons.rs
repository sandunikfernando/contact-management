use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::app::{
    dto::person::{CreatePersonRequest, UpdatePersonRequest},
    service::person_service,
    state::AppState,
    ServiceError,
};

pub async fn list_persons(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let persons = person_service::list_persons(&state).await?;
    Ok(Json(serde_json::json!(persons)))
}

pub async fn create_person(
    State(state): State<AppState>,
    Json(req): Json<CreatePersonRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), ServiceError> {
    let person = person_service::create_person(&state, req).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!(person))))
}

pub async fn get_person(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let person = person_service::get_person(&state, id).await?;
    Ok(Json(serde_json::json!(person)))
}

pub async fn update_person(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdatePersonRequest>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let person = person_service::update_person(&state, id, req).await?;
    Ok(Json(serde_json::json!(person)))
}

pub async fn delete_person(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ServiceError> {
    person_service::delete_person(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
