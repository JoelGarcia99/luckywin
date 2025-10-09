use std::sync::Arc;

use crate::{
    adapters::inbounds::rest::controllers::game::purchase_lottery, config::state::AppState,
    shared::infra::health_service::check_health,
};

pub const CHECK_HEALTH: &str = "/health";

// lottery
pub const PURCHASE_LOTTERY: &str = "/lottery/purchase";

pub fn build_routes_v1() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route(CHECK_HEALTH, axum::routing::get(check_health))
        .route(PURCHASE_LOTTERY, axum::routing::post(purchase_lottery))
}
