use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use serde_valid::Validate;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    adapters::{
        inbounds::rest::dtos::game::CreateGameDto,
        outbound::{db::games::GamesRepoImpl, rest::dtos::game::GameOutputDto},
    },
    config::state::AppState,
    domain::{
        enums::roles::Roles,
        usecases::{
            close_game::CloseGameUseCase, create_game::CreateGameUseCase,
            get_last_active_game::GetLastActiveGameUseCase, reopen_game::ReopenGameUseCase,
        },
    },
    shared::{
        errors::{Rfc7807Problem, StandardOutput},
        payloads::AccessTokenPayload,
        utils::jwt::decode_jwt,
    },
};

#[derive(OpenApi)]
#[openapi(paths(get_last_active_game, create_game, close_game, reopen_game))]
pub struct ApiDoc;

/// Get the last active game
///
/// Returns the last active game (where closed_at is NULL and deleted_at is NULL)
#[utoipa::path(
    get,
    path = "/active",
    tags = ["games"],
    responses(
        (status = 200, description = "Last active game retrieved", body = GameOutputDto),
        (status = 404, description = "No active game found", body = Rfc7807Problem)
    )
)]
pub async fn get_last_active_game(
    State(state): State<Arc<AppState>>,
) -> StandardOutput<GameOutputDto> {
    // Instantiate the repository implementation
    let games_repo = GamesRepoImpl;

    // Create the use case with the repository implementation
    let use_case = GetLastActiveGameUseCase::new(games_repo, &*state.db);

    // Execute the use case
    use_case.execute().await
}

/// @admin Create a new game
///
/// Creates a new game/lottery. Only one active game is allowed at a time (Admin only)
#[utoipa::path(
    post,
    path = "",
    tags = ["games"],
    request_body(content = CreateGameDto, content_type = "application/json"),
    responses(
        (status = 201, description = "Game created successfully", body = GameOutputDto),
        (status = 400, description = "Validation error or another game is active", body = Rfc7807Problem),
        (status = 401, description = "Unauthorized - Admin access required", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_game(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<CreateGameDto>,
) -> StandardOutput<GameOutputDto> {
    // Validate input
    body.validate()
        .map_err(|r| Rfc7807Problem::from_validator_err(&r.to_string()))?;

    // Extract and validate access token
    let access_token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            Box::new(Rfc7807Problem::unauthorized(
                Some("Token de acceso requerido"),
                Some("El header Authorization es requerido"),
                Some("MISSING_ACCESS_TOKEN"),
            ))
        })?;

    // Decode JWT and verify admin role
    let payload = decode_jwt::<AccessTokenPayload>(access_token, &state.secrets, false)?;

    if payload.role != Roles::Admin.to_string() {
        return Err(Box::new(Rfc7807Problem::unauthorized(
            Some("Acceso denegado"),
            Some("Solo los administradores pueden crear juegos"),
            Some("ADMIN_ACCESS_REQUIRED"),
        )));
    }

    // Instantiate repository
    let games_repo = GamesRepoImpl;

    // Create and execute use case
    let use_case = CreateGameUseCase::new(games_repo, Arc::clone(&state.db));

    use_case.execute(&body).await
}

/// @admin Close a game
///
/// Closes a game by setting finished_at to the current timestamp (Admin only)
#[utoipa::path(
    post,
    path = "/{game_id}/close",
    tags = ["games"],
    params(
        ("game_id" = Uuid, Path, description = "ID of the game to close")
    ),
    responses(
        (status = 200, description = "Game closed successfully", body = GameOutputDto),
        (status = 400, description = "Game already closed", body = Rfc7807Problem),
        (status = 401, description = "Unauthorized - Admin access required", body = Rfc7807Problem),
        (status = 404, description = "Game not found", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn close_game(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(game_id): Path<Uuid>,
) -> StandardOutput<GameOutputDto> {
    // Extract and validate access token
    let access_token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            Box::new(Rfc7807Problem::unauthorized(
                Some("Token de acceso requerido"),
                Some("El header Authorization es requerido"),
                Some("MISSING_ACCESS_TOKEN"),
            ))
        })?;

    // Decode JWT and verify admin role
    let payload = decode_jwt::<AccessTokenPayload>(access_token, &state.secrets, false)?;

    if payload.role != Roles::Admin.to_string() {
        return Err(Box::new(Rfc7807Problem::unauthorized(
            Some("Acceso denegado"),
            Some("Solo los administradores pueden cerrar juegos"),
            Some("ADMIN_ACCESS_REQUIRED"),
        )));
    }

    // Instantiate repository
    let games_repo = GamesRepoImpl;

    // Create and execute use case
    let use_case = CloseGameUseCase::new(games_repo, Arc::clone(&state.db));

    use_case.execute(game_id).await
}

/// @admin Reopen a game
///
/// Reopens a closed game by setting finished_at to NULL (Admin only)
#[utoipa::path(
    post,
    path = "/{game_id}/reopen",
    tags = ["games"],
    params(
        ("game_id" = Uuid, Path, description = "ID of the game to reopen")
    ),
    responses(
        (status = 200, description = "Game reopened successfully", body = GameOutputDto),
        (status = 400, description = "Game already open", body = Rfc7807Problem),
        (status = 401, description = "Unauthorized - Admin access required", body = Rfc7807Problem),
        (status = 404, description = "Game not found", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn reopen_game(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(game_id): Path<Uuid>,
) -> StandardOutput<GameOutputDto> {
    // Extract and validate access token
    let access_token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            Box::new(Rfc7807Problem::unauthorized(
                Some("Token de acceso requerido"),
                Some("El header Authorization es requerido"),
                Some("MISSING_ACCESS_TOKEN"),
            ))
        })?;

    // Decode JWT and verify admin role
    let payload = decode_jwt::<AccessTokenPayload>(access_token, &state.secrets, false)?;

    if payload.role != Roles::Admin.to_string() {
        return Err(Box::new(Rfc7807Problem::unauthorized(
            Some("Acceso denegado"),
            Some("Solo los administradores pueden reabrir juegos"),
            Some("ADMIN_ACCESS_REQUIRED"),
        )));
    }

    // Instantiate repository
    let games_repo = GamesRepoImpl;

    // Create and execute use case
    let use_case = ReopenGameUseCase::new(games_repo, Arc::clone(&state.db));

    use_case.execute(&game_id).await
}
