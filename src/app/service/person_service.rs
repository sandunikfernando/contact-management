// app/service/person_service.rs
// Business logic for Persons. All functions are now async because the
// repository functions await real PostgreSQL queries.
//
// What changed from in-memory version:
//   - Functions are `async fn` and `await` repository calls
//   - State passes `&db` (DatabaseConnection) instead of locking a Mutex
//   - Cascade delete is handled by the DB's ON DELETE CASCADE constraint,
//     but we still call delete_by_person_id in service for explicitness
//     (the DB constraint is a safety net, the service is the authoritative logic)

use crate::app::{
    dto::person::{CreatePersonRequest, PersonResponse, UpdatePersonRequest},
    repository::person_repository,
    state::AppState,
    ServiceError,
};

pub async fn create_person(
    state: &AppState,
    req: CreatePersonRequest,
) -> Result<PersonResponse, ServiceError> {
    if req.name.trim().is_empty() {
        return Err(ServiceError::BadRequest("Name must not be blank".into()));
    }
    person_repository::create(&state.db, req).await
}

pub async fn list_persons(
    state: &AppState
) -> Result<Vec<PersonResponse>, ServiceError> {
    person_repository::find_all(&state.db).await
}

pub async fn get_person(
    state: &AppState, id: i32
) -> Result<PersonResponse, ServiceError> {
    person_repository::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Person {} not found", id)))
}

pub async fn update_person(
    state: &AppState,
    id: i32,
    req: UpdatePersonRequest,
) -> Result<PersonResponse, ServiceError> {
    if let Some(ref name) = req.name {
        if name.trim().is_empty() {
            return Err(ServiceError::BadRequest("Name must not be blank".into()));
        }
    }

    person_repository::update(&state.db, id, req)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Person {} not found", id)))
}

pub async fn delete_person(
    state: &AppState, 
    id: i32
) -> Result<(), ServiceError> {
    person_repository::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Person {} not found", id)))?;

    person_repository::delete(&state.db, id).await?;
    Ok(())
}
