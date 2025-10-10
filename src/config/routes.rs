use std::sync::Arc;

use crate::{
    adapters::inbounds::rest::controllers::{
        game::get_last_active_game, health::check_health, lottery::purchase_lottery,
    },
    config::state::AppState,
};

pub const CHECK_HEALTH: &str = "/health";

// lottery
pub const PURCHASE_LOTTERY: &str = "/lottery/purchase";

// games
pub const GET_LAST_ACTIVE_GAME: &str = "/games/active";

pub fn build_routes_v1() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route(CHECK_HEALTH, axum::routing::get(check_health))
        .route(PURCHASE_LOTTERY, axum::routing::post(purchase_lottery))
        .route(
            GET_LAST_ACTIVE_GAME,
            axum::routing::get(get_last_active_game),
        )
}
