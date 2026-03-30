// pkg/web/web.rs
// Bootstraps the HTTP server:
//   1. Loads .env variables
//   2. Connects to PostgreSQL via SeaORM
//   3. Builds AppState with the DB connection
//   4. Builds the Axum router
//   5. Binds TcpListener and starts serving

use crate::app::{route::build_router, state::AppState};
use crate::pkg::config::app_config::AppConfig;
use sea_orm::{ConnectOptions, Database};
use std::time::Duration;
use tokio::net::TcpListener;

pub async fn init_web() {
    // 1. Load .env file into environment variables.
    //    dotenvy::dotenv() is a no-op (not an error) if .env doesn't exist,
    //    which is fine in production where env vars come from the host.
    dotenvy::dotenv().ok();

    // 2. Load app config (HOST, PORT, DATABASE_URL from env).
    let config = AppConfig::from_env();

    // 3. Configure the SeaORM connection pool.
    //    ConnectOptions wraps the DATABASE_URL and lets you tune pool behaviour.
    let mut opt = ConnectOptions::new(&config.database_url);
    opt
        // Maximum simultaneous DB connections in the pool
        .max_connections(10)
        // Minimum connections kept alive (avoids cold-start latency)
        .min_connections(2)
        // How long to wait for a connection from the pool before timing out
        .acquire_timeout(Duration::from_secs(8))
        // How long an idle connection is kept before being closed
        .idle_timeout(Duration::from_secs(60))
        // Log SQL queries at DEBUG level (set RUST_LOG=debug to see them)
        .sqlx_logging(true);

    // 4. Establish the connection pool.  This will fail fast if the DB is
    //    unreachable, which is better than failing on the first request.
    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to PostgreSQL");

    println!("Connected to PostgreSQL");

    // 5. Build shared state — just the DB pool handle.
    let state = AppState { db };

    // 6. Build the Axum router.
    let app = build_router(state);

    // 7. Bind the TCP listener.
    let addr = config.addr();
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind address");

    println!("Server listening on http://{}", addr);

    // 8. Serve forever.
    axum::serve(listener, app)
        .await
        .expect("Server failed");
}
