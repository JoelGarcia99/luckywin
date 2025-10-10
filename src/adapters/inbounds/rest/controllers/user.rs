
use std::sync::Arc;

use axum::{extract::State, http::HeaderMap};
use utoipa::OpenApi;

use crate::{
    config::state::AppState,
    shared::{
        errors::{Rfc7807Problem, StandardOutput},
        payloads::{AccessTokenPayload, CurrentUserDto},
        utils::jwt::decode_jwt,
    },
    adapters::outbound::db::users::UsersRepoImpl,
    domain::usecases::get_current_user::GetCurrentUserUseCase,
};

#[derive(OpenApi)]
#[openapi(paths(get_current_user))]
pub struct ApiDoc;

/// Get the current user in session
///
/// Get the full object of the curent user in session via access token payload
#[utoipa::path(
    get,
    path = "/me",
    tags = ["users"],
    responses(
        (status = 200, description = "Current user retrieved successfully", body = CurrentUserDto),
        (status = 401, description = "Unauthorized", body = Rfc7807Problem),
        (status = 404, description = "User not found", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_current_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> StandardOutput<CurrentUserDto> {
    let access_token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            Box::new(Rfc7807Problem::unauthorized(
                Some("Access token required"),
                Some("Authorization header is required"),
                Some("MISSING_ACCESS_TOKEN"),
            ))
        })?;

    let payload = decode_jwt::<AccessTokenPayload>(access_token, &*state.secrets, false)?;

    let users_repo = UsersRepoImpl;
    let use_case = GetCurrentUserUseCase::new(users_repo, Arc::clone(&state.db));

    use_case.execute(&payload.sub).await
}
