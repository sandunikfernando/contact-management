// app/entity/person.rs
// SeaORM entity for the `persons` table.
//
// SeaORM uses a specific module structure:
//   - `Model`        → the Rust struct that maps to a DB row
//   - `Entity`       → represents the table itself (used in queries)
//   - `ActiveModel`  → a version of Model where every field is wrapped in
//                      ActiveValue, used for INSERT and UPDATE operations
//   - `Column` enum  → each variant is a column name (used in .filter(), .order_by())
//   - `Relation` enum → defines foreign key relationships to other entities
//
// The `DeriveEntityModel` macro generates all of these from `Model` + `#[sea_orm(...)]`.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "persons")]  
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,

    pub name: String,

    pub display_name: Option<String>,
    pub notes: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::mobile::Entity")]
    Mobile,

    #[sea_orm(has_many = "super::email::Entity")]
    Email,
}

impl Related<super::mobile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Mobile.def()
    }
}

impl Related<super::email::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Email.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
