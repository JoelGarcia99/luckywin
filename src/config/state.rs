use std::sync::Arc;

use reqwest::Client;
use sea_orm::DatabaseConnection;

use crate::config::env::SecretsVariables;

pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub secrets: Arc<SecretsVariables>,
    pub http_client: Arc<Client>,
}
