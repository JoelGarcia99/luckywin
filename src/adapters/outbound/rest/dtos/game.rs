use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::FromQueryResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::games;

#[derive(Debug, Serialize, ToSchema, FromQueryResult)]
pub struct GameOutputDto {
    /// Unique identifier for the game
    pub id: Uuid,

    /// Game title
    #[schema(example = "Lotería Navideña 2025")]
    pub title: String,

    /// Banner image URL for the game
    #[schema(example = "https://example.com/banners/christmas-lottery.png")]
    pub banner_url: Option<String>,

    /// Ticket price in cents
    #[schema(example = 150)]
    pub ticket_price: i32,

    /// Game finish date and time (optional)
    #[schema(example = "2025-12-25T23:59:59Z")]
    pub finished_at: Option<DateTime<FixedOffset>>,

    /// Game status
    #[schema(example = 5)]
    pub max_tickets_per_lottery: i32,


    /// Record creation timestamp
    pub created_at: DateTime<FixedOffset>,

    /// Record last update timestamp
    pub updated_at: DateTime<FixedOffset>,
}

impl From<games::Model> for GameOutputDto {
    fn from(game: games::Model) -> Self {
        Self {
            id: game.id,
            title: game.title,
            banner_url: game.banner_url,
            ticket_price: game.ticket_price,
            finished_at: game.finished_at,
            max_tickets_per_lottery: game.max_tickets_per_lottery,
            created_at: game.created_at,
            updated_at: game.updated_at,
        }
    }
}
