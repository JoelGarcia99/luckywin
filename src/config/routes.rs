use std::sync::Arc;

use crate::{adapters::inbounds::rest::health_controller::check_health, config::state::AppState};

pub const CHECK_HEALTH: &str = "/health";

pub fn build_routes_v1() -> axum::Router::<Arc<AppState>> {
    axum::Router::new()
    .route(CHECK_HEALTH, axum::routing::get(check_health))
}
