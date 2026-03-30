// app/service/mod.rs
// The service layer owns all business logic:
//   - Validation (is the name empty?)
//   - Cross-entity rules (person must exist before adding a mobile)
//   - Cascade deletes
//
// Services receive the repositories via AppState and return ServiceError
// on failure so handlers can map them to HTTP responses.

pub mod email_service;
pub mod mobile_service;
pub mod person_service;
