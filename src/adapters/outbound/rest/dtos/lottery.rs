use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Default)]
pub struct LotteryOutputDto {
    pub client_name: String,
    pub client_last_name: Option<String>,
    pub client_email: String,
    pub client_cellphone: String,
    pub location: Option<String>,
    pub serial: i64,
    /// Purchased numbers for lottery. Constrained to 5 numbers
    pub numbers: Vec<u8>
}
