use std::sync::Arc;

use crate::{
    adapters::inbounds::rest::controllers::{
        auth::{login, logout, refresh_token},
        game::get_last_active_game,
        health::check_health,
        lottery::purchase_lottery,
        user::get_current_user,
    },
    config::state::AppState,
};

pub const CHECK_HEALTH: &str = "/health";

// auth
pub const LOGIN: &str = "/auth/login";
pub const REFRESH_TOKEN: &str = "/auth/refresh";
pub const LOGOUT: &str = "/auth/logout";

// lottery
pub const PURCHASE_LOTTERY: &str = "/lottery/purchase";

// games
pub const GET_LAST_ACTIVE_GAME: &str = "/games/active";

// users
pub const GET_CURRENT_USER: &str = "/users/me";

pub fn build_routes_v1() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route(CHECK_HEALTH, axum::routing::get(check_health))
        // Auth routes
        .route(LOGIN, axum::routing::post(login))
        .route(REFRESH_TOKEN, axum::routing::post(refresh_token))
        .route(LOGOUT, axum::routing::post(logout))
        // Lottery routes
        .route(PURCHASE_LOTTERY, axum::routing::post(purchase_lottery))
        // Game routes
        .route(
            GET_LAST_ACTIVE_GAME,
            axum::routing::get(get_last_active_game),
        )
        // User routes
        .route(GET_CURRENT_USER, axum::routing::get(get_current_user))
}
