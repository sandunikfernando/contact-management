use crate::app::{
    dto::mobile::{CreateMobileRequest, MobileResponse, UpdateMobileRequest},
    entity::mobile::{self, ActiveModel, Column},
    ServiceError,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter,
};

fn to_response(m: mobile::Model) -> MobileResponse {
    MobileResponse {
        id: m.id,
        person_id: m.person_id,
        number: m.number,
        label: m.label,
    }
}

pub async fn create(
    db: &DatabaseConnection,
    person_id: i32,
    req: CreateMobileRequest,
) -> Result<MobileResponse, ServiceError> {
    let model = ActiveModel {
        id: ActiveValue::NotSet,
        person_id: ActiveValue::Set(person_id),
        number: ActiveValue::Set(req.number),
        label: ActiveValue::Set(req.label),
    };

    let saved = model
        .insert(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(to_response(saved))
}

/// SELECT * FROM mobiles WHERE person_id = $1
pub async fn find_by_person_id(
    db: &DatabaseConnection,
    person_id: i32,
) -> Result<Vec<MobileResponse>, ServiceError> {
    let models = mobile::Entity::find()
        .filter(Column::PersonId.eq(person_id))  // WHERE person_id = $1
        .all(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(models.into_iter().map(to_response).collect())
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<MobileResponse>, ServiceError> {
    let model = mobile::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(model.map(to_response))
}

pub async fn update(
    db: &DatabaseConnection,
    id: i32,
    req: UpdateMobileRequest,
) -> Result<Option<MobileResponse>, ServiceError> {
    let existing = mobile::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    let Some(existing) = existing else {
        return Ok(None);
    };

    let mut active: ActiveModel = existing.into_active_model();

    if let Some(number) = req.number {
        active.number = ActiveValue::Set(number);
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
    let result = mobile::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(result.rows_affected > 0)
}
