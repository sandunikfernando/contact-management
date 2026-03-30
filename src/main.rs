// main.rs
// Entry point of the application.
// We delegate everything to lib.rs so that integration tests can also import the app.

use contact_management::pkg::web::web::init_web;

#[tokio::main]
async fn main() {
    init_web().await;
}
