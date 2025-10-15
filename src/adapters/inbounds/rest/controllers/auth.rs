use std::sync::Arc;

use axum::{Json, extract::State, http::HeaderMap};
use serde_valid::Validate;
use utoipa::OpenApi;

use crate::{
    adapters::{
        inbounds::rest::dtos::auth::{
            LoginInputDto, LoginOutputDto, LogoutOutputDto, RefreshTokenInputDto,
        },
        outbound::db::{sessions::SessionsRepoImpl, users::UsersRepoImpl},
    },
    config::state::AppState,
    domain::usecases::{
        login::LoginUseCase, logout::LogoutUseCase, refresh_token::RefreshTokenUseCase,
    },
    shared::{
        errors::{Rfc7807Problem, StandardOutput},
        payloads::AccessTokenPayload,
        utils::jwt::decode_jwt,
    },
};

#[derive(OpenApi)]
#[openapi(paths(login, refresh_token, logout))]
pub struct ApiDoc;

/// @admin Login endpoint
///
/// Authenticates a user with cellphone and password, returns access and refresh tokens
#[utoipa::path(
    post,
    path = "/login",
    tags = ["auth"],
    request_body = LoginInputDto,
    responses(
        (status = 200, description = "Login successful", body = LoginOutputDto),
        (status = 400, description = "Invalid credentials", body = Rfc7807Problem),
        (status = 401, description = "Unauthorized", body = Rfc7807Problem)
    )
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<LoginInputDto>,
) -> StandardOutput<LoginOutputDto> {
    body.validate()
        .map_err(|r| Rfc7807Problem::from_validator_err(&r.to_string()))?;

    // Extract device fingerprint, IP, and user agent from headers
    let device_fingerprint = headers
        .get("X-Device-Fingerprint")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let ip = headers
        .get("X-Forwarded-For")
        .or_else(|| headers.get("X-Real-IP"))
        .and_then(|v| v.to_str().ok());

    let user_agent = headers.get("User-Agent").and_then(|v| v.to_str().ok());

    // Instantiate repositories
    let users_repo = UsersRepoImpl;
    let sessions_repo = SessionsRepoImpl;

    // Create use case
    let use_case = LoginUseCase::new(
        users_repo,
        sessions_repo,
        Arc::clone(&state.db),
        (*state.secrets).clone(),
    );

    // Execute login
    use_case
        .execute(&body, device_fingerprint, ip, user_agent)
        .await
}

/// @admin Refresh token endpoint
///
/// Uses a valid refresh token to generate new access and refresh tokens (token rotation)
#[utoipa::path(
    post,
    path = "/refresh",
    tags = ["auth"],
    request_body = RefreshTokenInputDto,
    responses(
        (status = 200, description = "Token refreshed successfully", body = LoginOutputDto),
        (status = 401, description = "Invalid or expired token", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn refresh_token(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<RefreshTokenInputDto>,
) -> StandardOutput<LoginOutputDto> {
    body.validate()
        .map_err(|r| Rfc7807Problem::from_validator_err(&r.to_string()))?;

    // Extract access token from Authorization header
    let access_token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            Box::new(Rfc7807Problem::unauthorized(
                Some("Token de acceso requerido"),
                Some("El header Authorization es requerido"),
                Some("MISSING_ACCESS_TOKEN"),
            ))
        })?;

    // Extract device fingerprint, IP, and user agent
    let device_fingerprint = headers
        .get("X-Device-Fingerprint")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    let ip = headers
        .get("X-Forwarded-For")
        .or_else(|| headers.get("X-Real-IP"))
        .and_then(|v| v.to_str().ok());

    let user_agent = headers.get("User-Agent").and_then(|v| v.to_str().ok());

    // Instantiate repositories
    let users_repo = UsersRepoImpl;
    let sessions_repo = SessionsRepoImpl;

    // Create use case
    let use_case = RefreshTokenUseCase::new(
        users_repo,
        sessions_repo,
        Arc::clone(&state.db),
        (*state.secrets).clone(),
    );

    // Execute token refresh
    use_case
        .execute(
            access_token,
            &body.refresh_token,
            device_fingerprint,
            ip,
            user_agent,
        )
        .await
}

/// @admin Logout endpoint
///
/// Invalidates the current session for the authenticated user
#[utoipa::path(
    post,
    path = "/logout",
    tags = ["auth"],
    responses(
        (status = 200, description = "Logout successful", body = LogoutOutputDto),
        (status = 401, description = "Unauthorized", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn logout(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> StandardOutput<LogoutOutputDto> {
    // Extract access token from Authorization header
    let access_token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            Box::new(Rfc7807Problem::unauthorized(
                Some("Token de acceso requerido"),
                Some("El header Authorization es requerido"),
                Some("MISSING_ACCESS_TOKEN"),
            ))
        })?;

    // Decode token to get user ID
    let payload = decode_jwt::<AccessTokenPayload>(access_token, &*state.secrets, false)?;

    // Extract device fingerprint
    let device_fingerprint = headers
        .get("X-Device-Fingerprint")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");

    // Instantiate repository
    let sessions_repo = SessionsRepoImpl;

    // Create use case
    let use_case = LogoutUseCase::new(sessions_repo, Arc::clone(&state.db));

    // Execute logout
    use_case.execute(&payload.sub, device_fingerprint).await
}
