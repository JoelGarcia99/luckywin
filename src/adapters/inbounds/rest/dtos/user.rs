use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateUserDto {
    #[validate(min_length = 2, message = "El nombre debe tener al menos 2 caracteres")]
    pub first_name: Option<String>,
    #[validate(
        min_length = 2,
        message = "El apellido debe tener al menos 2 caracteres"
    )]
    pub last_name: Option<String>,
    pub location: Option<String>,
    // TODO: implement custom validator for phone
    pub cellphone: Option<String>,
    // TODO: implement custom validator for email
    pub email: Option<String>,
}

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
