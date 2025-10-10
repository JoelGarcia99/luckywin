use std::sync::Arc;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::lottery::LotteryOutputDto,
    domain::{enums::lottery_status::LotteryStatus, repositories::lotteries::LotteriesRepo},
    shared::errors::StandardOutput,
};

/// Marks a lottery ticket as paid
pub struct MarkLotteryAsPaidUseCase<R: LotteriesRepo> {
    lottery_repo: R,
    db: Arc<DatabaseConnection>,
}

impl<R: LotteriesRepo> MarkLotteryAsPaidUseCase<R> {
    pub fn new(lottery_repo: R, db: Arc<DatabaseConnection>) -> Self {
        Self { lottery_repo, db }
    }

    pub async fn execute(&self, lottery_id: Uuid) -> StandardOutput<LotteryOutputDto> {
        self.lottery_repo
            .update_status(lottery_id, LotteryStatus::Paid.to_string(), &*self.db)
            .await
    }
}
