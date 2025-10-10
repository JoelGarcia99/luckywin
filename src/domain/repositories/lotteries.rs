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

    /// Updates the status of a lottery
    async fn update_status<C>(
        &self,
        lottery_id: Uuid,
        status: String,
        db: &C,
    ) -> StandardOutput<LotteryOutputDto>
    where
        C: ConnectionTrait;
}
