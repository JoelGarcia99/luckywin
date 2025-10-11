use std::sync::Arc;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::game::GameOutputDto,
    domain::repositories::games::GamesRepo,
    shared::errors::StandardOutput,
};

/// Closes a game by setting finished_at to current timestamp (Admin only)
pub struct CloseGameUseCase<R: GamesRepo> {
    repo: R,
    db: Arc<DatabaseConnection>,
}

impl<R: GamesRepo> CloseGameUseCase<R> {
    pub fn new(repo: R, db: Arc<DatabaseConnection>) -> Self {
        Self { repo, db }
    }

    pub async fn execute(&self, game_id: Uuid) -> StandardOutput<GameOutputDto> {
        self.repo.close_game(&game_id, &*self.db).await
    }
}
