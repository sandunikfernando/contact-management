use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMobileRequest {
    pub number: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMobileRequest {
    pub number: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct MobileResponse {
    pub id: i32,
    pub person_id: i32,
    pub number: String,
    pub label: Option<String>,
}
