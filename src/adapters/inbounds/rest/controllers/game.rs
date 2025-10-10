use std::sync::Arc;

use axum::extract::State;
use utoipa::OpenApi;

use crate::{
    adapters::outbound::{db::games::GamesRepoImpl, rest::dtos::game::GameOutputDto},
    config::state::AppState,
    domain::usecases::get_last_active_game::GetLastActiveGameUseCase,
    shared::errors::{Rfc7807Problem, StandardOutput},
};

#[derive(OpenApi)]
#[openapi(paths(get_last_active_game))]
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
