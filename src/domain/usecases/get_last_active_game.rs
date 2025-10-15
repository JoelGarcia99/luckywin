use sea_orm::ConnectionTrait;

use crate::{
    adapters::outbound::rest::dtos::game::GameOutputDto, domain::repositories::games::GamesRepo,
    shared::errors::StandardOutput,
};

/// Gets the last active game (where closed_at is NULL and deleted_at is NULL)
pub struct GetLastActiveGameUseCase<'a, R: GamesRepo, C: ConnectionTrait> {
    repo: R,
    db: &'a C,
}

impl<'a, R: GamesRepo, C: ConnectionTrait> GetLastActiveGameUseCase<'a, R, C> {
    pub fn new(repo: R, db: &'a C) -> Self {
        Self { repo, db }
    }

    pub async fn execute(&self) -> StandardOutput<GameOutputDto> {
        self.repo.get_last_active(self.db).await
    }
}
