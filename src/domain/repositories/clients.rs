use async_trait::async_trait;
use sea_orm::ConnectionTrait;
use uuid::Uuid;

use crate::{
    adapters::outbound::rest::dtos::client::ClientOutputDto, domain::entities::users,
    shared::errors::StandardOutput,
};

#[async_trait]
pub trait ClientsRepo: Send + Sync + Copy {
    async fn create_client<C>(
        &self,
        client: &users::ActiveModel,
        db: &C,
    ) -> StandardOutput<ClientOutputDto>
    where
        C: ConnectionTrait;

    async fn update_client<C>(
        &self,
        client_id: &Uuid,
        client: &users::ActiveModel,
        db: &C,
    ) -> StandardOutput<ClientOutputDto>
    where
        C: ConnectionTrait;

    async fn get_client_by_id<C>(&self, id: &str, db: &C) -> StandardOutput<ClientOutputDto>
    where
        C: ConnectionTrait;

    async fn get_client_by_cellphone<C>(
        &self,
        cellphone: &str,
        db: &C,
    ) -> StandardOutput<ClientOutputDto>
    where
        C: ConnectionTrait;

    async fn list_clients<C>(
        &self,
        db: &C,
        page: u64,
        per_page: u64,
    ) -> StandardOutput<Vec<ClientOutputDto>>
    where
        C: ConnectionTrait;
}
