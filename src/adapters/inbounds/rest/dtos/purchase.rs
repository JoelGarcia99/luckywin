use serde::Deserialize;
use serde_valid::Validate;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Validate, Deserialize, ToSchema, Default)]
pub struct CreatePurchaseDto {
    #[validate(
        min_length = 3,
        message = "El nombre de usuario debe tener al menos 3 caracteres"
    )]
    #[validate(
        max_length = 255,
        message = "El nombre de usuario debe tener menos de 255 caracteres"
    )]
    pub client_name: String,
    #[validate(
        min_length = 3,
        message = "El apellido de usuario debe tener al menos 3 caracteres"
    )]
    #[validate(
        max_length = 255,
        message = "El apellido de usuario debe tener menos de 255 caracteres"
    )]
    pub client_last_name: Option<String>,
    #[validate(
        pattern = r#"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4}"#,
        message = "El correo electrónico no es válido"
    )]
    pub client_email: Option<String>,
    /// mobile phone must start with +593 or 09. If it belongs with +593 it must be 13 chars long. If it belongs with 09 it must be 10 chars long
    #[validate(
        pattern = r#"^(?:\+593\s?9\d{8}|09\d{8})$"#,
        message = "El número de celular no es válido"
    )]
    pub client_cellphone: String,
    pub location: Option<String>,
    /// Purchased tickets for lottery. Each ticket has 5 numbers
    pub tickets: Vec<CreateTicketDto>,
}

#[derive(Debug, Deserialize, ToSchema, Default)]
pub struct CreateTicketDto {
    /// Lottery numbers (must be 5 numbers)
    pub numbers: Vec<u8>,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct MarkPurchaseAsPaidQueryParams {
    /// ID of the purchase to mark as paid
    pub purchase_id: Uuid,
}
