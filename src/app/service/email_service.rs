use crate::app::{
    dto::email::{CreateEmailRequest, EmailResponse, UpdateEmailRequest},
    repository::{email_repository, person_repository},
    state::AppState,
    ServiceError,
};

pub async fn add_email(
    state: &AppState,
    person_id: i32,
    req: CreateEmailRequest,
) -> Result<EmailResponse, ServiceError> {
    person_repository::find_by_id(&state.db, person_id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Person {} not found", person_id)))?;

    if req.address.trim().is_empty() {
        return Err(ServiceError::BadRequest("Address must not be blank".into()));
    }

    email_repository::create(&state.db, person_id, req).await
}

pub async fn list_emails(
    state: &AppState,
    person_id: i32,
) -> Result<Vec<EmailResponse>, ServiceError> {
    person_repository::find_by_id(&state.db, person_id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Person {} not found", person_id)))?;

    email_repository::find_by_person_id(&state.db, person_id).await
}

pub async fn get_email(state: &AppState, id: i32) -> Result<EmailResponse, ServiceError> {
    email_repository::find_by_id(&state.db, id)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Email {} not found", id)))
}

pub async fn update_email(
    state: &AppState,
    id: i32,
    req: UpdateEmailRequest,
) -> Result<EmailResponse, ServiceError> {
    if let Some(ref address) = req.address {
        if address.trim().is_empty() {
            return Err(ServiceError::BadRequest("Address must not be blank".into()));
        }
    }

    email_repository::update(&state.db, id, req)
        .await?
        .ok_or_else(|| ServiceError::NotFound(format!("Email {} not found", id)))
}

pub async fn delete_email(state: &AppState, id: i32) -> Result<(), ServiceError> {
    let deleted = email_repository::delete(&state.db, id).await?;
    if deleted {
        Ok(())
    } else {
        Err(ServiceError::NotFound(format!("Email {} not found", id)))
    }
}
