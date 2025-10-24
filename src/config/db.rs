use std::sync::Arc;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

/// Stablishes connection with DB and logs some details about it
pub async fn connect_db(secrets: &crate::config::env::SecretsVariables) -> Arc<DatabaseConnection> {
    // defining DB connection pool using the postgres DSN
    let mut db_opts = ConnectOptions::new(&secrets.dsn);

    db_opts
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(std::time::Duration::from_secs(8))
        // close
        .idle_timeout(std::time::Duration::from_secs(8))
        // Recycle connections after 30 seconds of inactivity to avoid stale connections
        .max_lifetime(std::time::Duration::from_secs(1800))
        // NOTE: prints SQL queries to stdout. Activate for debugging purposes
        .sqlx_logging(false)
        .set_schema_search_path("public");

    tracing::info!("Connected to database");
    tracing::debug!("DB DSN: {}", secrets.dsn);
    match Database::connect(db_opts).await {
        Ok(db) => Arc::new(db),
        Err(err) => panic!("Failed to connect to database: {}", err),
    }
}
