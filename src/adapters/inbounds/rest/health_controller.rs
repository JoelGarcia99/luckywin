use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};
use utoipa::OpenApi;

use crate::{
    config::{routes::CHECK_HEALTH, state::AppState},
    shared::{errors::Rfc7807Problem, infra::health_service},
};

#[derive(OpenApi)]
#[openapi(paths(check_health))]
pub struct ApiDoc;

/// Check health
///
/// Checks if infra is ok including DB connection
#[utoipa::path(
    get,
    path = "",
    tags = ["infra"],
    responses(
        (status = 200, description = "Infra is ok", body = String),
        (status = 400, description = "Infra not ok", body = Rfc7807Problem)
    )
)]
pub async fn check_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    health_service::check_health(&state.db).await
}
