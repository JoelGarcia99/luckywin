use sea_orm::DatabaseConnection;

use crate::{adapters::outbound::db::error_parsing::process_db_err, shared::errors::{StandardOutput, StandardResponse}};

/// Checks if infra is ok, including DB and tracin
pub async fn check_health(db: &DatabaseConnection) -> StandardOutput<String> {
    db.ping().await.map_err(|e| process_db_err(&e))?;

    Ok(StandardResponse::simple(String::from("OK"), None))
}
