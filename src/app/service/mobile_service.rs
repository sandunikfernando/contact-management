use crate::app::{
    dto::mobile::{CreateMobileRequest, MobileResponse, UpdateMobileRequest},
    repository::{mobile_repository, person_repository},
    state::AppState,
    ServiceError,
};

pub async fn add_mobile(
    state: &AppState,
    person_id: i32,
    req: CreateMobileRequest,
) -> Result<MobileResponse, ServiceError> {
    person_repository::find_by_id(&state.db, person_id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Person {} not found", person_id)))?;

    if req.number.trim().is_empty() {
        return Err(ServiceError::BadRequest("Number must not be blank".into()));
    }

    mobile_repository::create(&state.db, person_id, req).await
}

pub async fn list_mobiles(
    state: &AppState,
    person_id: i32,
) -> Result<Vec<MobileResponse>, ServiceError> {
    person_repository::find_by_id(&state.db, person_id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Person {} not found", person_id)))?;

    mobile_repository::find_by_person_id(&state.db, person_id).await
}

pub async fn get_mobile(state: &AppState, id: i32) -> Result<MobileResponse, ServiceError> {
    mobile_repository::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Mobile {} not found", id)))
}

pub async fn update_mobile(
    state: &AppState,
    id: i32,
    req: UpdateMobileRequest,
) -> Result<MobileResponse, ServiceError> {
    if let Some(ref number) = req.number {
        if number.trim().is_empty() {
            return Err(ServiceError::BadRequest("Number must not be blank".into()));
        }
    }

    mobile_repository::update(&state.db, id, req)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Mobile {} not found", id)))
}

pub async fn delete_mobile(state: &AppState, id: i32) -> Result<(), ServiceError> {
    let deleted = mobile_repository::delete(&state.db, id).await?;
    if deleted {
        Ok(())
    } else {
        Err(ServiceError::NotFound(format!("Mobile {} not found", id)))
    }
}
