use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use utoipa::ToSchema;

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginInputDto {
    /// Email address used for login
    #[schema(example = "dev@devallsas.com")]
    #[validate(min_length = 5)]
    #[validate(max_length = 255)]
    pub email: String,

    /// User password
    #[schema(example = "MySecurePass123!")]
    #[validate(min_length = 8)]
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginOutputDto {
    /// Access token containing user information in its payload
    /// Used for API authentication
    pub access_token: String,

    /// Refresh token used to obtain a new access token
    /// This is a one-shot token - once used it is invalidated
    pub refresh_token: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct RefreshTokenInputDto {
    /// The refresh token to use for obtaining a new access token
    #[validate(min_length = 1)]
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
pub struct LogoutOutputDto {
    pub message: String,
}
