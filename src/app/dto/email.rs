use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateEmailRequest {
    pub address: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEmailRequest {
    pub address: Option<String>,
    pub label: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct EmailResponse {
    pub id: i32,
    pub person_id: i32,
    pub address: String,
    pub label: Option<String>,
}
