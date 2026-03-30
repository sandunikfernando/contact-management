use crate::app::{
    dto::email::{CreateEmailRequest, EmailResponse, UpdateEmailRequest},
    entity::email::{self, ActiveModel, Column},
    ServiceError,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};

fn to_response(m: email::Model) -> EmailResponse {
    EmailResponse {
        id: m.id,
        person_id: m.person_id,
        address: m.address,
        label: m.label,
    }
}

pub async fn create(
    db: &DatabaseConnection,
    person_id: i32,
    req: CreateEmailRequest,
) -> Result<EmailResponse, ServiceError> {
    let model = ActiveModel {
        id: ActiveValue::NotSet,
        person_id: ActiveValue::Set(person_id),
        address: ActiveValue::Set(req.address),
        label: ActiveValue::Set(req.label),
    };

    let saved = model
        .insert(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(to_response(saved))
}

pub async fn find_by_person_id(
    db: &DatabaseConnection,
    person_id: i32,
) -> Result<Vec<EmailResponse>, ServiceError> {
    let models = email::Entity::find()
        .filter(Column::PersonId.eq(person_id))
        .all(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(models.into_iter().map(to_response).collect())
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<EmailResponse>, ServiceError> {
    let model = email::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(model.map(to_response))
}

pub async fn update(
    db: &DatabaseConnection,
    id: i32,
    req: UpdateEmailRequest,
) -> Result<Option<EmailResponse>, ServiceError> {
    let existing = email::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    let Some(existing) = existing else {
        return Ok(None);
    };

    let mut active: ActiveModel = existing.into_active_model();

    if let Some(address) = req.address {
        active.address = ActiveValue::Set(address);
    }
    if req.label.is_some() {
        active.label = ActiveValue::Set(req.label);
    }

    let saved = active
        .update(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(Some(to_response(saved)))
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<bool, ServiceError> {
    let result = email::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(result.rows_affected > 0)
}
