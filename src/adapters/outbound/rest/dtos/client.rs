use sea_orm::FromQueryResult;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema, FromQueryResult)]
pub struct ClientOutputDto {
    pub id: Uuid,
    pub name: String,
    pub last_name: Option<String>,
    pub email: String,
    pub cellphone: String,
    pub location: Option<String>,
}
