use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::game::GameOutputDto,
    domain::entities::games,
    shared::errors::StandardOutput,
};

#[async_trait]
pub trait GamesRepo: Send + Sync + Copy {
    async fn create<C>(
        &self,
        game: &games::ActiveModel,
        db: &C,
    ) -> StandardOutput<GameOutputDto>
    where
        C: ConnectionTrait;

    async fn list_paginated<C>(
        &self,
        page: u64,
        per_page: u64,
        db: &C,
    ) -> StandardOutput<Vec<GameOutputDto>>
    where
        C: ConnectionTrait;

    async fn get_by_id<C>(&self, id: &Uuid, db: &C) -> StandardOutput<GameOutputDto>
    where
        C: ConnectionTrait;

    /// Gets the last active game (where closed_at is NULL and deleted_at is NULL)
    /// ordered by created_at DESC
    async fn get_last_active<C>(&self, db: &C) -> StandardOutput<GameOutputDto>
    where
        C: ConnectionTrait;

    async fn get_batch_by_ids<C>(&self, db: &C, ids: &Vec<Uuid>) -> StandardOutput<Vec<GameOutputDto>>
    where
        C: ConnectionTrait;
}
