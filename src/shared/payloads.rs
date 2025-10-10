use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// This will be the claim to encode within an access token and give it to the user
/// to perform actions within the system
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessTokenPayload {
    /// Expiration time measured in UTC format (Unix timestamp)
    pub exp: i64,
    /// Issued at time (Unix timestamp)
    pub iat: i64,
    /// Audience
    pub aud: String,
    /// Session ID (refresh token's ID)
    pub sid: Uuid,
    /// Device fingerprint - identifier for the device (not biometric information)
    pub dfp: String,
    /// User's ID (subject)
    pub sub: Uuid,
    /// User's role
    pub role: String,
}

/// This will be the claim to encode within a refresh token and store it into auth DB
/// so the user can renew their session at any time as long as this token is valid
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshTokenPayload {
    /// Expiration time measured in UTC format (Unix timestamp)
    pub exp: i64,
}
