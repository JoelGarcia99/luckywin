use serde::Deserialize;
use serde_valid::Validate;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Validate, Deserialize, ToSchema, Default)]
pub struct LotteriesPurchaseDto {
    #[validate(min_length = 3, message = "El nombre de usuario debe tener al menos 3 caracteres")]
    #[validate(max_length = 255, message = "El nombre de usuario debe tener menos de 255 caracteres")]
    pub client_name: String,
    #[validate(min_length = 3, message = "El apellido de usuario debe tener al menos 3 caracteres")]
    #[validate(max_length = 255, message = "El apellido de usuario debe tener menos de 255 caracteres")]
    pub client_last_name: Option<String>,
    #[validate(pattern = r#"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4}"#, message = "El correo electrónico no es válido")]
    pub client_email: String,
    /// mobile phone must start with +593 or 09. If it belongs with +593 it must be 13 chars long. If it belongs with 09 it must be 10 chars long
    #[validate(pattern = r#"^(?:\+593\s?9\d{8}|09\d{8})$"#, message = "El número de celular no es válido")]
    pub client_cellphone: String,
    pub location: Option<String>,
    /// Purchased numbers for lottery. Constrained to 5 numbers
    pub tickets: Vec<CreateLotteryTicketDto>,
}

#[derive(Debug, Deserialize, ToSchema, Default)]
pub struct CreateLotteryTicketDto {
    pub numbers: Vec<u8>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct MarkLotteryAsPaidQueryParams {
    /// ID of the lottery ticket to mark as paid
    pub lottery_id: Uuid,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct GetLotteriesByGameQueryParams {
    /// Page number (starts at 0)
    #[serde(default)]
    pub page: Option<u64>,
    /// Number of records per page (default: 20)
    #[serde(default)]
    pub per_page: Option<u64>,
    /// Filter by serial number (leading zeros will be removed)
    #[serde(default)]
    pub serial: Option<String>,
}
