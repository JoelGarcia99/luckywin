use std::sync::Arc;

use axum::{extract::{Path, Query, State}, http::HeaderMap, Json};
use serde_valid::Validate;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    adapters::{
        inbounds::rest::dtos::lottery::{GetLotteriesByGameQueryParams, LotteriesPurchaseDto, MarkLotteryAsPaidQueryParams},
        outbound::{
            db::{clients::ClientsRepoImpl, games::GamesRepoImpl, lotteries::LotteriesRepoImpl},
            rest::dtos::lottery::LotteryOutputDto,
        },
    },
    config::state::AppState,
    domain::{
        enums::roles::Roles,
        usecases::{get_lotteries_by_game::GetLotteriesByGameUseCase, mark_lottery_as_paid::MarkLotteryAsPaidUseCase, purchase_lottery::PurchaseLotteryUseCase},
    },
    shared::{
        errors::{Rfc7807Problem, StandardOutput},
        payloads::AccessTokenPayload,
        utils::jwt::decode_jwt,
    },
};

#[derive(OpenApi)]
#[openapi(paths(purchase_lottery, mark_lottery_as_paid, get_lotteries_by_game))]
pub struct ApiDoc;

/// Purchases lottery tickets
///
/// Purchases many tickets for a given client
#[utoipa::path(
    post,
    path = "/purchase",
    tags = ["lottery"],
    request_body = LotteriesPurchaseDto,
    responses(
        (status = 201, description = "Lotteries purchased", body = Vec<LotteryOutputDto>),
        (status = 400, description = "Error purchasing lotteries", body = Rfc7807Problem)
    )
)]
pub async fn purchase_lottery(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LotteriesPurchaseDto>,
) -> StandardOutput<Vec<LotteryOutputDto>> {
    body.validate()
        .map_err(|r| Rfc7807Problem::from_validator_err(&r.to_string()))?;

    // Instantiate the repository implementations
    let lotteries_repo = LotteriesRepoImpl;
    let games_repo = GamesRepoImpl;
    let clients_repo = ClientsRepoImpl;

    // Create the use case with the repository implementations
    let use_case = PurchaseLotteryUseCase::new(
        lotteries_repo,
        games_repo,
        clients_repo,
        Arc::clone(&state.db),
    );

    // Execute the use case
    use_case.execute(body).await
}

/// @admin Mark lottery as paid
///
/// Marks a lottery ticket's status from pending to paid (Admin only)
#[utoipa::path(
    post,
    path = "/mark-as-paid",
    tags = ["lottery"],
    params(MarkLotteryAsPaidQueryParams),
    responses(
        (status = 200, description = "Lottery marked as paid successfully", body = LotteryOutputDto),
        (status = 401, description = "Unauthorized - Admin access required", body = Rfc7807Problem),
        (status = 404, description = "Lottery not found", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn mark_lottery_as_paid(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<MarkLotteryAsPaidQueryParams>,
) -> StandardOutput<LotteryOutputDto> {
    // Extract and validate access token
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

    // Decode JWT and verify admin role
    let payload = decode_jwt::<AccessTokenPayload>(access_token, &state.secrets, false)?;

    if payload.role != Roles::Admin.to_string() {
        return Err(Box::new(Rfc7807Problem::unauthorized(
            Some("Acceso denegado"),
            Some("Solo los administradores pueden marcar loterías como pagadas"),
            Some("ADMIN_ACCESS_REQUIRED"),
        )));
    }

    // Instantiate repository
    let lotteries_repo = LotteriesRepoImpl;

    // Create and execute use case
    let use_case = MarkLotteryAsPaidUseCase::new(lotteries_repo, Arc::clone(&state.db));

    use_case.execute(params.lottery_id).await
}

/// @admin Get lottery tickets by game ID
///
/// Fetches paginated lottery tickets for a specific game with optional serial filter (Admin only)
#[utoipa::path(
    get,
    path = "/by-game/{game_id}",
    tags = ["lottery"],
    params(
        ("game_id" = Uuid, Path, description = "ID of the game to fetch lottery tickets for"),
        GetLotteriesByGameQueryParams
    ),
    responses(
        (status = 200, description = "Lotteries fetched successfully", body = Vec<LotteryOutputDto>),
        (status = 401, description = "Unauthorized - Admin access required", body = Rfc7807Problem),
        (status = 404, description = "Game not found", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_lotteries_by_game(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Path(game_id): Path<Uuid>,
    Query(params): Query<GetLotteriesByGameQueryParams>,
) -> StandardOutput<Vec<LotteryOutputDto>> {
    // Extract and validate access token
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

    // Decode JWT and verify admin role
    let payload = decode_jwt::<AccessTokenPayload>(access_token, &state.secrets, false)?;

    if payload.role != Roles::Admin.to_string() {
        return Err(Box::new(Rfc7807Problem::unauthorized(
            Some("Acceso denegado"),
            Some("Solo los administradores pueden ver las loterías de un juego"),
            Some("ADMIN_ACCESS_REQUIRED"),
        )));
    }

    // Instantiate repository
    let lotteries_repo = LotteriesRepoImpl;

    // Create and execute use case
    let use_case = GetLotteriesByGameUseCase::new(lotteries_repo, Arc::clone(&state.db));

    use_case.execute(game_id, params.page, params.per_page, params.serial).await
}
