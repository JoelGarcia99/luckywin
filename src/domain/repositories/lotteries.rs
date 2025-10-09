use async_trait::async_trait;
use sea_orm::ConnectionTrait;

use crate::{
    adapters::outbound::rest::dtos::lottery::LotteryOutputDto, domain::entities::lotteries,
    shared::errors::StandardOutput,
};

#[async_trait]
pub trait LotteriesRepo: Send + Sync {
    async fn create<C>(
        &self,
        lotteries: Vec<lotteries::ActiveModel>,
        db: &C,
    ) -> StandardOutput<LotteryOutputDto>
    where
        C: ConnectionTrait;
}
