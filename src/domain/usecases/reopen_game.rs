use std::sync::Arc;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::game::GameOutputDto, domain::repositories::games::GamesRepo,
    shared::errors::StandardOutput,
};

pub struct ReopenGameUseCase<R: GamesRepo> {
    games_repo: R,
    db: Arc<DatabaseConnection>,
}

impl<R: GamesRepo> ReopenGameUseCase<R> {
    pub fn new(games_repo: R, db: Arc<DatabaseConnection>) -> Self {
        Self { games_repo, db }
    }

    pub async fn execute(&self, game_id: &Uuid) -> StandardOutput<GameOutputDto> {
        self.games_repo.reopen_game(game_id, &*self.db).await
    }
}
