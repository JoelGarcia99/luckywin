use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::lottery::LotteryOutputDto, domain::entities::lotteries,
    shared::errors::StandardOutput,
};

#[async_trait]
pub trait LotteriesRepo: Send + Sync {
    /// Creates a batch of lotteries
    async fn batch_create<C>(
        &self,
        lotteries: Vec<lotteries::ActiveModel>,
        db: &C,
    ) -> StandardOutput<Vec<LotteryOutputDto>>
    where
        C: ConnectionTrait;

    /// Gets a list of lotteries by ids
    async fn list_batch_by_ids<C>(
        &self,
        ids: Vec<Uuid>,
        db: &C,
    ) -> StandardOutput<Vec<LotteryOutputDto>>
    where
        C: ConnectionTrait;

    /// Gets paginated lotteries by game ID with optional serial filter
    async fn get_by_game_id_paginated<C>(
        &self,
        game_id: Uuid,
        page: u64,
        per_page: u64,
        serial_filter: Option<i64>,
        db: &C,
    ) -> StandardOutput<Vec<LotteryOutputDto>>
    where
        C: ConnectionTrait;
}
