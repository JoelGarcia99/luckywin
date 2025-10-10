use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct CurrentUserDto {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: Option<String>,
    pub location: Option<String>,
    pub cellphone: String,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
