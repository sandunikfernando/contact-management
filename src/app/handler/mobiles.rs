use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::app::{
    dto::mobile::{CreateMobileRequest, UpdateMobileRequest},
    service::mobile_service,
    state::AppState,
    ServiceError,
};

pub async fn list_mobiles(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let mobiles = mobile_service::list_mobiles(&state, person_id).await?;
    Ok(Json(serde_json::json!(mobiles)))
}

pub async fn add_mobile(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
    Json(req): Json<CreateMobileRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), ServiceError> {
    let mobile = mobile_service::add_mobile(&state, person_id, req).await?;
    Ok((StatusCode::CREATED, Json(serde_json::json!(mobile))))
}

pub async fn get_mobile(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let mobile = mobile_service::get_mobile(&state, id).await?;
    Ok(Json(serde_json::json!(mobile)))
}

pub async fn update_mobile(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateMobileRequest>,
) -> Result<Json<serde_json::Value>, ServiceError> {
    let mobile = mobile_service::update_mobile(&state, id, req).await?;
    Ok(Json(serde_json::json!(mobile)))
}

pub async fn delete_mobile(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ServiceError> {
    mobile_service::delete_mobile(&state, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
