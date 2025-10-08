use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::{config::state::AppState, shared::infra::health_service};

/// Checks if infra is ok, including DB and tracing
pub async fn check_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    health_service::check_health(&state.db).await
}
