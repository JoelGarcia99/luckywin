use chrono::{DateTime, Utc};
use sea_orm::FromQueryResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{domain::entities::lotteries, shared::utils::bitboards};

#[derive(Debug, Serialize, ToSchema)]
pub struct PurchaseOutputDto {
    pub id: Uuid,
    pub serial: i64,
    pub id_game: Uuid,
    pub id_client: Uuid,
    pub client_name: String,
    pub client_email: Option<String>,
    pub client_cellphone: Option<String>,
    pub location: Option<String>,
    pub total_paid: i32,
    pub status: String,
    pub tickets: Vec<TicketOutputDto>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema, FromQueryResult, Clone)]
pub struct TicketOutputDto {
    pub id: Uuid,
    pub serial: i64,
    pub numbers: Vec<u8>, // Formatted lottery numbers
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<lotteries::Model> for TicketOutputDto {
    fn from(model: lotteries::Model) -> Self {
        Self {
            id: model.id,
            serial: model.serial,
            numbers: bitboards::parse_bitboard_into_vec(model.numbers).unwrap_or_default(),
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}
