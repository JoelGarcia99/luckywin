use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    config::env::SecretsVariables,
    shared::errors::{CANNOT_DECODE_TOKEN, CANNOT_GENERATE_TOKEN, Rfc7807Problem},
};

const CORRUPTED_SESSION_MSG: &str = "La sesión es inválida o ha expirado";

/// Encodes a given claim `T` into a JWT using the password defined on ENV variables.
/// Uses HS256 algorithm by default
pub fn encode_jwt<T>(claim: &T, state: &SecretsVariables) -> Result<String, Box<Rfc7807Problem>>
where
    T: Serialize,
{
    let encoding_key = EncodingKey::from_secret(state.jwt_password.as_ref());

    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some(state.jwt_kid.clone());

    match encode(&header, claim, &encoding_key) {
        Ok(token) => Ok(token),
        Err(err) => {
            tracing::error!("Failed to encode JWT: {}", err);
            Err(Box::new(Rfc7807Problem::server_error(
                Some("No se pudo generar el token de sesión"),
                Some("Este es un error crítico, por favor contacte a soporte técnico"),
                Some(CANNOT_GENERATE_TOKEN),
            )))
        }
    }
}

/// Decodes a JWT using the password defined on ENV variables. Uses HS256 algorithm by default.
/// Returns the decoded token if everything is ok or an error otherwise.
/// `bypass_expiration` is used to allow the token to be decoded even if it's expired
pub fn decode_jwt<T>(
    token: &str,
    secrets: &SecretsVariables,
    bypass_expiration: bool,
) -> Result<T, Box<Rfc7807Problem>>
where
    T: DeserializeOwned,
{
    let mut validation = Validation::new(Algorithm::HS256);

    if !bypass_expiration {
        validation.validate_exp = true;
    }

    validation.set_audience(&[&secrets.jwt_aud]);
    validation.leeway = 60; // 1 minute of tolerance for expirations (due to servers' delay)

    let decoding_key = DecodingKey::from_secret(secrets.jwt_password.as_ref());
    match decode::<T>(token, &decoding_key, &validation) {
        Ok(decoded_token) => Ok(decoded_token.claims),
        Err(err) => {
            tracing::error!("Failed to decode JWT: {}", err);
            Err(Box::new(Rfc7807Problem::unauthorized(
                Some(CORRUPTED_SESSION_MSG),
                None,
                Some(CANNOT_DECODE_TOKEN),
            )))
        }
    }
}
