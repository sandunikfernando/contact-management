// app/repository/person_repository.rs
// Data access for the `persons` table using SeaORM.
//
// Key SeaORM concepts used here:
//
//   Entity::find()              → SELECT * FROM persons
//   Entity::find_by_id(id)      → SELECT * FROM persons WHERE id = $1
//   ActiveModel { .. }          → represents a row to INSERT or UPDATE
//   ActiveValue::Set(v)         → "set this column to v"
//   ActiveValue::NotSet         → "leave this column alone" (for partial update)
//   model.into_active_model()   → converts a fetched Model into an ActiveModel
//   active_model.insert(&db)    → executes INSERT, returns the saved Model
//   active_model.update(&db)    → executes UPDATE, returns the saved Model
//   Entity::delete_by_id(id)    → DELETE FROM persons WHERE id = $1

use crate::app::{
    dto::person::{CreatePersonRequest, PersonResponse, UpdatePersonRequest},
    entity::person::{self, ActiveModel},
    ServiceError,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait,
    IntoActiveModel,
};

fn to_response(m: person::Model) -> PersonResponse {
    PersonResponse {
        id: m.id,
        name: m.name,
        display_name: m.display_name,
        notes: m.notes,
    }
}

pub async fn create(
    db: &DatabaseConnection,
    req: CreatePersonRequest,
) -> Result<PersonResponse, ServiceError> {
    let model = ActiveModel {
        id: ActiveValue::NotSet,   // SERIAL — PostgreSQL generates this
        name: ActiveValue::Set(req.name),
        display_name: ActiveValue::Set(req.display_name),
        notes: ActiveValue::Set(req.notes),
    };

    let saved = model
        .insert(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(to_response(saved))
}

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<PersonResponse>, ServiceError> {
    let models = person::Entity::find()
        .all(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(models.into_iter().map(to_response).collect())
}

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<PersonResponse>, ServiceError> {
    let model = person::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(model.map(to_response))
}

pub async fn update(
    db: &DatabaseConnection,
    id: i32,
    req: UpdatePersonRequest,
) -> Result<Option<PersonResponse>, ServiceError> {
    let existing = person::Entity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    let Some(existing) = existing else {
        return Ok(None);
    };

    // into_active_model() wraps each field in ActiveValue::Unchanged —
    // SeaORM only emits SET clauses for columns we explicitly Set(..)
    let mut active: ActiveModel = existing.into_active_model();

    if let Some(name) = req.name {
        active.name = ActiveValue::Set(name);
    }
    if req.display_name.is_some() {
        active.display_name = ActiveValue::Set(req.display_name);
    }
    if req.notes.is_some() {
        active.notes = ActiveValue::Set(req.notes);
    }

    let saved = active
        .update(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(Some(to_response(saved)))
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<bool, ServiceError> {
    let result = person::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| ServiceError::InternalError(e.to_string()))?;

    Ok(result.rows_affected > 0)
}
