use std::sync::Arc;

use sea_orm::DatabaseConnection;

use crate::config::env::SecretsVariables;

pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub secrets: Arc<SecretsVariables>,
}
