use chrono::{DateTime, Utc};
use sea_orm::FromQueryResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

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

    /// Game finish date and time
    #[schema(example = "2025-12-25T23:59:59Z")]
    pub finished_at: DateTime<Utc>,

    /// Record creation timestamp
    pub created_at: DateTime<Utc>,

    /// Record last update timestamp
    pub updated_at: DateTime<Utc>,
}
