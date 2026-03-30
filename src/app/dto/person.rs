use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePersonRequest {
    pub name: String,
    pub display_name: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePersonRequest {
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct PersonResponse {
    pub id: i32,
    pub name: String,
    pub display_name: Option<String>,
    pub notes: Option<String>,
}
