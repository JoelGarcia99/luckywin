use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

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

#[derive(Debug, Serialize, ToSchema)]
pub struct TicketOutputDto {
    pub id: Uuid,
    pub serial: i64,
    pub numbers: String, // Formatted lottery numbers
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
