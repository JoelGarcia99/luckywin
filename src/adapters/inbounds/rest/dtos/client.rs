use serde::Deserialize;
use serde_valid::Validate;
use utoipa::ToSchema;

#[derive(Debug, Validate, Deserialize, ToSchema, Default)]
pub struct ClientsInputDto {
    #[validate(
        min_length = 3,
        message = "El nombre de usuario debe tener al menos 3 caracteres"
    )]
    #[validate(
        max_length = 255,
        message = "El nombre de usuario debe tener menos de 255 caracteres"
    )]
    pub first_name: String,
    #[validate(
        min_length = 3,
        message = "El apellido de usuario debe tener al menos 3 caracteres"
    )]
    #[validate(
        max_length = 255,
        message = "El apellido de usuario debe tener menos de 255 caracteres"
    )]
    pub last_name: Option<String>,
    #[validate(
        pattern = r#"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4}"#,
        message = "El correo electrónico no es válido"
    )]
    pub email: String,
    /// mobile phone must start with +593 or 09. If it belongs with +593 it must be 13 chars long. If it belongs with 09 it must be 10 chars long
    #[validate(
        pattern = r#"^(\+593|09)\d{9}$"#,
        message = "El número de celular no es válido"
    )]
    pub cellphone: String,
    pub location: Option<String>,
}
