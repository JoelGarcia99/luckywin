use sea_orm::DatabaseConnection;

use crate::{adapters::outbound::db::error_parsing::process_db_err, shared::errors::Rfc7807Problem};

/// Checks if infra is ok, including DB and tracin
pub async fn check_health(db: &DatabaseConnection) -> Result<&'static str, Rfc7807Problem> {
    db.ping().await.map_err(|e| process_db_err(&e))?;

    Ok("OK")
}
