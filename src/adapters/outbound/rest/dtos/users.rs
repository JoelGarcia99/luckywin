use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::entities::users;

#[derive(Serialize, ToSchema)]
pub struct UserOutputDto {
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

impl From<users::Model> for UserOutputDto {
    fn from(user: users::Model) -> Self {
        Self {
            id: user.id,
            first_name: user.first_name,
            last_name: user.last_name,
            location: user.location,
            cellphone: user.cellphone,
            email: user.email,
            role: user.role,
            created_at: user.created_at.into(),
            updated_at: user.updated_at.into(),
        }
    }
}
