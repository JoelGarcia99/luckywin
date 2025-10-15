use std::sync::Arc;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::lottery::LotteryOutputDto,
    domain::repositories::lotteries::LotteriesRepo, shared::errors::StandardOutput,
};

const DEFAULT_PAGE: u64 = 0;
const DEFAULT_PER_PAGE: u64 = 20;

/// Gets paginated lottery tickets for a specific game (Admin only)
pub struct GetLotteriesByGameUseCase<R: LotteriesRepo> {
    lottery_repo: R,
    db: Arc<DatabaseConnection>,
}

impl<R: LotteriesRepo> GetLotteriesByGameUseCase<R> {
    pub fn new(lottery_repo: R, db: Arc<DatabaseConnection>) -> Self {
        Self { lottery_repo, db }
    }

    pub async fn execute(
        &self,
        game_id: Uuid,
        page: Option<u64>,
        per_page: Option<u64>,
        serial: Option<String>,
    ) -> StandardOutput<Vec<LotteryOutputDto>> {
        // Set defaults for pagination
        let page = page.unwrap_or(DEFAULT_PAGE);
        let per_page = per_page.unwrap_or(DEFAULT_PER_PAGE);

        // Parse serial filter, removing leading zeros
        let serial_filter = serial.and_then(|s| s.trim().parse::<i64>().ok());

        self.lottery_repo
            .get_by_game_id_paginated(game_id, page, per_page, serial_filter, &*self.db)
            .await
    }
}
