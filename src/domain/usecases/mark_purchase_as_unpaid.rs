use std::sync::Arc;

use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    adapters::outbound::{db::purchases::PurchasesRepoImpl, rest::dtos::purchases::PurchaseOutputDto},
    domain::repositories::purchases::PurchasesRepo,
    shared::errors::{Rfc7807Problem, StandardOutput, StandardResponse},
};

pub struct MarkPurchaseAsUnpaidUseCase {
    purchases_repo: PurchasesRepoImpl,
    db: Arc<DatabaseConnection>,
}

impl MarkPurchaseAsUnpaidUseCase {
    pub fn new(purchases_repo: PurchasesRepoImpl, db: Arc<DatabaseConnection>) -> Self {
        Self { purchases_repo, db }
    }

    pub async fn execute(&self, purchase_id: &Uuid) -> StandardOutput<PurchaseOutputDto> {
        // Find the purchase
        let purchase = self.purchases_repo
            .find_by_id(purchase_id, &*self.db)
            .await?
            .data
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::not_found(
                    Some("Compra no encontrada"),
                    Some("La compra especificada no existe o ha sido eliminada"),
                ))
            })?;

        // Check if already unpaid
        if purchase.status == "pending" {
            return Err(Box::new(Rfc7807Problem::simple_bad_request(
                Some("Compra ya está pendiente de pago"),
                Some("Esta compra ya está marcada como pendiente"),
                Some("PURCHASE_ALREADY_UNPAID"),
            )));
        }

        // Use the existing mark_as_paid implementation but manually update to pending
        // We'll need to add a new method to the repo for this
        use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
        use crate::domain::entities::purchases;
        use crate::adapters::outbound::db::error_parsing::process_db_err;

        let purchase_entity = purchases::Entity::find_by_id(*purchase_id)
            .filter(purchases::Column::DeletedAt.is_null())
            .one(&*self.db)
            .await
            .map_err(|e| process_db_err(&e))?
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::not_found(
                    Some("Compra no encontrada"),
                    Some("La compra especificada no existe o ha sido eliminada"),
                ))
            })?;

        let mut active_model: purchases::ActiveModel = purchase_entity.into();
        active_model.status = Set("pending".to_string());
        active_model.updated_at = Set(chrono::Utc::now().into());

        let updated_purchase = active_model
            .update(&*self.db)
            .await
            .map_err(|e| process_db_err(&e))?;

        // Get with tickets
        self.purchases_repo
            .find_by_id(&updated_purchase.id, &*self.db)
            .await?
            .data
            .ok_or_else(|| {
                Box::new(Rfc7807Problem::not_found(
                    Some("Compra no encontrada"),
                    None,
                ))
            })
            .map(|p| StandardResponse::simple(p, None))
    }
}
