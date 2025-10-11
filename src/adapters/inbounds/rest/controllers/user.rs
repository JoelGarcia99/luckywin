use std::sync::Arc;

use axum::{Json, extract::{Query, State}, http::HeaderMap};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};

use crate::{
    adapters::{
        inbounds::rest::dtos::user::UpdateUserDto,
        outbound::{db::users::UsersRepoImpl, rest::dtos::users::UserOutputDto},
    },
    config::state::AppState,
    domain::{
        enums::roles::Roles,
        usecases::{
            get_current_user::GetCurrentUserUseCase, get_user_by_email::GetUserByEmailUseCase, update_current_user::UpdateCurrentUserUseCase,
        },
    },
    shared::{
        errors::{Rfc7807Problem, StandardOutput},
        payloads::AccessTokenPayload,
        utils::jwt::decode_jwt,
    },
};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in=Query)]
pub struct GetUserByEmailQueryParams {
    /// User email to search for
    #[param(inline)]
    pub email: String,
}

#[derive(OpenApi)]
#[openapi(paths(get_current_user, update_current_user, get_user_by_email))]
pub struct ApiDoc;

/// @admin Get current user in session
///
/// Get current user in session from JWT payload
#[utoipa::path(
    get,
    path = "/me",
    tags = ["users"],
    responses(
        (status = 200, description = "Current user retrieved successfully", body = UserOutputDto),
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
) -> StandardOutput<UserOutputDto> {
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

    let payload = decode_jwt::<AccessTokenPayload>(access_token, &state.secrets, false)?;

    let users_repo = UsersRepoImpl;
    let use_case = GetCurrentUserUseCase::new(users_repo, Arc::clone(&state.db));

    use_case.execute(&payload.sub).await
}

/// @admin Update current user
///
/// Update current user if it's an admin
#[utoipa::path(
    patch,
    path = "/me",
    tags = ["users"],
    request_body = UpdateUserDto,
    responses(
        (status = 200, description = "User updated successfully", body = UserOutputDto),
        (status = 400, description = "Invalid data", body = Rfc7807Problem),
        (status = 401, description = "Unauthorized", body = Rfc7807Problem),
        (status = 404, description = "User not found", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_current_user(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(dto): Json<UpdateUserDto>,
) -> StandardOutput<UserOutputDto> {
    tracing::debug!("Seeing what's going on {:?}", headers.get("Authorization"));
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

    tracing::debug!("passed first check");
    let payload = decode_jwt::<AccessTokenPayload>(access_token, &state.secrets, false)?;

    if payload.role != Roles::Admin.to_string() {
        return Err(Box::new(Rfc7807Problem::unauthorized(
            Some("Acceso denegado"),
            Some("No tienes permisos para realizar esta accion"),
            Some("UNAUTHORIZED"),
        )));
    }

    let users_repo = UsersRepoImpl;
    let use_case = UpdateCurrentUserUseCase::new(users_repo, Arc::clone(&state.db));

    use_case.execute(&payload.sub, &dto).await
}

/// Get user by email
///
/// Retrieves user information by email address (public endpoint - no authentication required)
#[utoipa::path(
    get,
    path = "/by-email",
    tags = ["users"],
    params(GetUserByEmailQueryParams),
    responses(
        (status = 200, description = "User retrieved successfully", body = UserOutputDto),
        (status = 404, description = "User not found", body = Rfc7807Problem)
    )
)]
pub async fn get_user_by_email(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GetUserByEmailQueryParams>,
) -> StandardOutput<UserOutputDto> {
    let users_repo = UsersRepoImpl;
    let use_case = GetUserByEmailUseCase::new(users_repo, Arc::clone(&state.db));

    use_case.execute(&params.email).await
}
