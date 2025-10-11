use chrono::{DateTime, FixedOffset};
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

    /// Maximum number of tickets per lottery purchase
    #[schema(example = 5)]
    pub max_tickets_per_lottery: i32,

    /// Game finish date and time (when the game was closed)
    #[schema(example = "2025-12-25T23:59:59Z")]
    pub finished_at: Option<DateTime<FixedOffset>>,

    /// Game close date and time (when purchases should stop)
    #[schema(example = "2025-12-24T18:00:00Z")]
    pub closes_at: Option<DateTime<FixedOffset>>,

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
            max_tickets_per_lottery: game.max_tickets_per_lottery,
            finished_at: game.finished_at,
            closes_at: game.closes_at,
            created_at: game.created_at,
            updated_at: game.updated_at,
        }
    }
}
