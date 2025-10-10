use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_valid::Validate;
use utoipa::ToSchema;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateGameDto {
    /// Game title (must be between 10 and 255 characters)
    #[validate(min_length = 10, message = "El título debe tener al menos 10 caracteres")]
    #[validate(max_length = 255, message = "El título debe tener menos de 255 caracteres")]
    #[schema(example = "Lotería Navideña 2025")]
    pub title: String,

    /// Banner image URL for the game
    #[schema(example = "https://example.com/banners/christmas-lottery.png")]
    pub banner_url: Option<String>,

    /// Ticket price in cents (max 10000 = $100 USD)
    #[validate(maximum = 10000, message = "El precio del ticket no puede superar los 10000 centavos (100 USD)")]
    #[validate(minimum = 1, message = "El precio del ticket debe ser mayor a 0")]
    #[schema(example = 150)]
    pub ticket_price: i32,

    /// Game finish date and time (optional)
    #[schema(example = "2025-12-25T23:59:59Z")]
    pub finished_at: Option<DateTime<Utc>>,
}
