use std::sync::Arc;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::purchases::PurchaseOutputDto,
    domain::repositories::purchases::PurchasesRepo, shared::errors::StandardOutput,
};

const DEFAULT_PAGE: u64 = 0;
const DEFAULT_PER_PAGE: u64 = 20;

/// Gets paginated purchases with their tickets for a specific game (Admin only)
pub struct GetPurchasesByGameUseCase<R: PurchasesRepo> {
    purchases_repo: R,
    db: Arc<DatabaseConnection>,
}

impl<R: PurchasesRepo> GetPurchasesByGameUseCase<R> {
    pub fn new(purchases_repo: R, db: Arc<DatabaseConnection>) -> Self {
        Self { purchases_repo, db }
    }

    pub async fn execute(
        &self,
        game_id: Uuid,
        page: Option<u64>,
        per_page: Option<u64>,
        serial: Option<String>,
    ) -> StandardOutput<Vec<PurchaseOutputDto>> {
        // Set defaults for pagination
        let page = page.unwrap_or(DEFAULT_PAGE);
        let per_page = per_page.unwrap_or(DEFAULT_PER_PAGE);

        // Parse serial filter, removing leading zeros
        let serial_filter = serial.and_then(|s| s.trim().parse::<i64>().ok());

        self.purchases_repo
            .find_by_game_with_tickets(&game_id, page, per_page, serial_filter, &*self.db)
            .await
    }
}
