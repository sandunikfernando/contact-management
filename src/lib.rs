// lib.rs
// The crate root. Declares the two top-level modules so they are visible
// both to main.rs and to any future integration tests.

pub mod app;  // Everything HTTP: handlers, services, repositories, DTOs, routes
pub mod pkg;  // Cross-cutting: config, server bootstrap
