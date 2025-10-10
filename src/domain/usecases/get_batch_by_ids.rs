use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::game::GameOutputDto, domain::repositories::games::GamesRepo,
    shared::errors::StandardOutput,
};

/// Gets the last active game (where closed_at is NULL and deleted_at is NULL)
pub struct GetBatchGamesByIdsUseCase<'a, R: GamesRepo, C: ConnectionTrait> {
    repo: R,
    db: &'a C,
    ids: &'a Vec<Uuid>,
}

impl<'a, R: GamesRepo, C: ConnectionTrait> GetBatchGamesByIdsUseCase<'a, R, C> {
    pub fn new(repo: R, db: &'a C, ids: &'a Vec<Uuid>) -> Self {
        Self { repo, db, ids }
    }

    pub async fn execute(&self) -> StandardOutput<Vec<GameOutputDto>> {
        self.repo.get_batch_by_ids(&*self.db, self.ids).await
    }
}
