use std::sync::Arc;

use crate::{
    adapters::inbounds::rest::controllers::{
        auth::{login, logout, refresh_token},
        game::{close_game, create_game, get_last_active_game, reopen_game},
        health::check_health,
        lottery::{
            get_lotteries_by_game, mark_lottery_as_paid, mark_lottery_as_unpaid, purchase_lottery,
        },
        user::{get_current_user, get_user_by_cellphone, update_current_user},
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
pub const MARK_LOTTERY_AS_PAID: &str = "/lottery/mark-as-paid";
pub const MARK_LOTTERY_AS_UNPAID: &str = "/lottery/mark-as-unpaid";
pub const GET_LOTTERIES_BY_GAME: &str = "/lottery/by-game/{game_id}";

// games
pub const CREATE_GAME: &str = "/games";
pub const CLOSE_GAME: &str = "/games/{game_id}/close";
pub const REOPEN_GAME: &str = "/games/{game_id}/reopen";
pub const GET_LAST_ACTIVE_GAME: &str = "/games/active";

// users
pub const GET_CURRENT_USER: &str = "/users/me";
pub const GET_USER_BY_CELLPHONE: &str = "/users/by-cellphone";

pub fn build_routes_v1() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route(CHECK_HEALTH, axum::routing::get(check_health))
        // Auth routes
        .route(LOGIN, axum::routing::post(login))
        .route(REFRESH_TOKEN, axum::routing::post(refresh_token))
        .route(LOGOUT, axum::routing::post(logout))
        // Lottery routes
        .route(PURCHASE_LOTTERY, axum::routing::post(purchase_lottery))
        .route(
            MARK_LOTTERY_AS_PAID,
            axum::routing::post(mark_lottery_as_paid),
        )
        .route(
            MARK_LOTTERY_AS_UNPAID,
            axum::routing::post(mark_lottery_as_unpaid),
        )
        .route(
            GET_LOTTERIES_BY_GAME,
            axum::routing::get(get_lotteries_by_game),
        )
        // Game routes
        .route(CREATE_GAME, axum::routing::post(create_game))
        .route(CLOSE_GAME, axum::routing::post(close_game))
        .route(REOPEN_GAME, axum::routing::post(reopen_game))
        .route(
            GET_LAST_ACTIVE_GAME,
            axum::routing::get(get_last_active_game),
        )
        // User routes
        .route(
            GET_CURRENT_USER,
            axum::routing::get(get_current_user).patch(update_current_user),
        )
        .route(
            GET_USER_BY_CELLPHONE,
            axum::routing::get(get_user_by_cellphone),
        )
}
