use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::purchases::PurchaseOutputDto,
    shared::errors::StandardOutput,
};

#[async_trait::async_trait]
pub trait PurchasesRepo: Send + Sync {
    async fn create<C>(
        &self,
        id_game: &Uuid,
        id_client: &Uuid,
        client_name: &str,
        client_email: Option<&str>,
        client_cellphone: Option<&str>,
        location: Option<&str>,
        total_paid: i32,
        db: &C,
    ) -> StandardOutput<PurchaseOutputDto>
    where
        C: ConnectionTrait;

    async fn find_by_id<C>(&self, id: &Uuid, db: &C) -> StandardOutput<Option<PurchaseOutputDto>>
    where
        C: ConnectionTrait;

    async fn mark_as_paid<C>(&self, id: &Uuid, db: &C) -> StandardOutput<PurchaseOutputDto>
    where
        C: ConnectionTrait;

    async fn find_by_game_with_tickets<C>(
        &self,
        game_id: &Uuid,
        page: u64,
        per_page: u64,
        serial_filter: Option<i64>,
        db: &C,
    ) -> StandardOutput<Vec<PurchaseOutputDto>>
    where
        C: ConnectionTrait;
}
