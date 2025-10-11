use std::sync::Arc;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::purchases::PurchaseOutputDto,
    domain::repositories::purchases::PurchasesRepo,
    shared::errors::StandardOutput,
};

pub struct MarkPurchaseAsPaidUseCase<R: PurchasesRepo> {
    purchases_repo: R,
    db: Arc<DatabaseConnection>,
}

impl<R: PurchasesRepo> MarkPurchaseAsPaidUseCase<R> {
    pub fn new(purchases_repo: R, db: Arc<DatabaseConnection>) -> Self {
        Self { purchases_repo, db }
    }

    pub async fn execute(&self, purchase_id: &Uuid) -> StandardOutput<PurchaseOutputDto> {
        self.purchases_repo.mark_as_paid(purchase_id, &*self.db).await
    }
}
