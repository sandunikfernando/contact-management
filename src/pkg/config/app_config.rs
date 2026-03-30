// pkg/config/app_config.rs
// Holds application-wide configuration loaded from environment variables.
// Values come from the .env file (loaded by dotenvy at startup) or from
// the host environment in production.
//
// .env example:
//   HOST=127.0.0.1
//   PORT=3000
//   DATABASE_URL=postgres://user:password@localhost:5432/contact_db

pub struct AppConfig {
    pub host: String,
    pub port: u16,
    /// Full PostgreSQL connection URL.
    /// Format: postgres://USER:PASSWORD@HOST:PORT/DATABASE
    pub database_url: String,
}

impl AppConfig {
    /// Read all config from environment variables with sensible defaults.
    pub fn from_env() -> Self {
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        // DATABASE_URL is required — panic early with a clear message if missing.
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env or environment");

        AppConfig { host, port, database_url }
    }

    /// Returns "host:port" string for TcpListener::bind.
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
