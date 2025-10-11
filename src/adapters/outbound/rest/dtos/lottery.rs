use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{domain::entities::lotteries, shared::utils::bitboards};

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct LotteryOutputDto {
    pub id: Uuid,
    pub id_game: Uuid,
    pub id_purchase: Option<Uuid>,
    pub serial: i64,
    /// Purchased numbers for lottery. Constrained to 5 numbers
    pub numbers: Vec<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<lotteries::Model> for LotteryOutputDto {
    fn from(value: lotteries::Model) -> Self {
        Self {
            id: value.id,
            id_game: value.id_game,
            id_purchase: value.id_purchase,
            serial: value.serial,
            // WARN: this could lead to some bugs if it just returns empty array. Find a way to
            // handle it properly with Result instead
            numbers: bitboards::parse_bitboard_into_vec(value.numbers).unwrap_or_default(),
            created_at: value.created_at.into(),
        }
    }
}
