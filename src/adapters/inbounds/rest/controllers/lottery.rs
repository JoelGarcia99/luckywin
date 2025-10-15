use std::sync::Arc;

use axum::{
    Json,
    body::Body,
    extract::{Path, Query, State},
    http::{
        HeaderMap, StatusCode,
        header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    },
    response::Response,
};
use serde_valid::Validate;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    adapters::{
        inbounds::rest::dtos::{
            lottery::{GetLotteriesByGameQueryParams, LotteryByUserQueryParams},
            purchase::{CreatePurchaseDto, MarkPurchaseAsPaidQueryParams},
        },
        outbound::{
            db::{
                clients::ClientsRepoImpl, games::GamesRepoImpl, lotteries::LotteriesRepoImpl,
                purchases::PurchasesRepoImpl, users::UsersRepoImpl,
            },
            rest::dtos::purchases::{PurchaseOutputDto, TicketOutputDto},
        },
    },
    config::state::AppState,
    domain::{
        enums::roles::Roles,
        usecases::{
            create_purchase::CreatePurchaseUseCase,
            get_purchase_ticket_pdf::GetPurchaseTicketPdfUseCase,
            get_purchases_by_game::GetPurchasesByGameUseCase,
            get_tickets_for_user::GetTicketsForUser,
            mark_purchase_as_paid::MarkPurchaseAsPaidUseCase,
            mark_purchase_as_unpaid::MarkPurchaseAsUnpaidUseCase,
        },
    },
    shared::{
        errors::{Rfc7807Problem, StandardOutput},
        payloads::AccessTokenPayload,
        utils::jwt::decode_jwt,
    },
};

#[derive(OpenApi)]
#[openapi(paths(
    purchase_lottery,
    mark_lottery_as_paid,
    mark_lottery_as_unpaid,
    get_lotteries_by_game,
    get_lotteries_by_user,
    get_purchase_ticket_pdf
))]
pub struct ApiDoc;

/// Get all tickets by user
///
/// Gets all the purchased tickets for the current active game (if any) for the given user filtered
/// by phone
#[utoipa::path(
    get,
    path = "/by-user",
    tags = ["lottery"],
    responses(
        (status = 201, description = "Purchase created successfully", body = Vec<TicketOutputDto>),
        (status = 400, description = "Error creating purchase", body = Rfc7807Problem)
    ),
    params(
        ("phone" = String, Query, description = "Phone number of the client")
    )
)]
pub async fn get_lotteries_by_user(
    State(state): State<Arc<AppState>>,
    Query(query): Query<LotteryByUserQueryParams>,
) -> StandardOutput<Vec<TicketOutputDto>> {
    let users_repo = UsersRepoImpl;
    let games_repo = GamesRepoImpl;
    let lotteries_repo = LotteriesRepoImpl;

    query
        .validate()
        .map_err(|r| Rfc7807Problem::from_validator_err(&r.to_string()))?;

    let use_case = GetTicketsForUser::new(
        users_repo,
        games_repo,
        lotteries_repo,
        &query.phone,
        Arc::clone(&state.db),
    );

    use_case.execute().await
}

/// Purchases lottery tickets
///
/// Creates a purchase with multiple lottery tickets for a given client
#[utoipa::path(
    post,
    path = "/purchase",
    tags = ["lottery"],
    request_body(content = CreatePurchaseDto, content_type = "application/json"),
    responses(
        (status = 201, description = "Purchase created successfully", body = PurchaseOutputDto),
        (status = 400, description = "Error creating purchase", body = Rfc7807Problem)
    )
)]
pub async fn purchase_lottery(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreatePurchaseDto>,
) -> StandardOutput<PurchaseOutputDto> {
    tracing::debug!("Creating purchase");
    body.validate()
        .map_err(|r| Rfc7807Problem::from_validator_err(&r.to_string()))?;

    // Instantiate the repository implementations
    let lotteries_repo = LotteriesRepoImpl;
    let games_repo = GamesRepoImpl;
    let clients_repo = ClientsRepoImpl;
    let purchases_repo = PurchasesRepoImpl;

    // Create the use case with the repository implementations
    let use_case = CreatePurchaseUseCase::new(
        lotteries_repo,
        games_repo,
        clients_repo,
        purchases_repo,
        Arc::clone(&state.db),
    );

    // Execute the use case
    use_case.execute(body).await
}

/// @admin Mark purchase as paid
///
/// Marks a purchase's status from pending to paid (Admin only)
#[utoipa::path(
    post,
    path = "/mark-as-paid",
    tags = ["lottery"],
    params(MarkPurchaseAsPaidQueryParams),
    responses(
        (status = 200, description = "Purchase marked as paid successfully", body = PurchaseOutputDto),
        (status = 401, description = "Unauthorized - Admin access required", body = Rfc7807Problem),
        (status = 404, description = "Purchase not found", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn mark_lottery_as_paid(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<MarkPurchaseAsPaidQueryParams>,
) -> StandardOutput<PurchaseOutputDto> {
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
            Some("Solo los administradores pueden marcar compras como pagadas"),
            Some("ADMIN_ACCESS_REQUIRED"),
        )));
    }

    // Instantiate repository
    let purchases_repo = PurchasesRepoImpl;

    // Create and execute use case
    let use_case = MarkPurchaseAsPaidUseCase::new(purchases_repo, Arc::clone(&state.db));

    use_case.execute(&params.purchase_id).await
}

/// @admin Mark purchase as unpaid
///
/// Marks a purchase's status from paid back to pending (Admin only)
#[utoipa::path(
    post,
    path = "/mark-as-unpaid",
    tags = ["lottery"],
    params(MarkPurchaseAsPaidQueryParams),
    responses(
        (status = 200, description = "Purchase marked as unpaid successfully", body = PurchaseOutputDto),
        (status = 400, description = "Purchase already unpaid", body = Rfc7807Problem),
        (status = 401, description = "Unauthorized - Admin access required", body = Rfc7807Problem),
        (status = 404, description = "Purchase not found", body = Rfc7807Problem)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn mark_lottery_as_unpaid(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<MarkPurchaseAsPaidQueryParams>,
) -> StandardOutput<PurchaseOutputDto> {
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
            Some("Solo los administradores pueden marcar compras como no pagadas"),
            Some("ADMIN_ACCESS_REQUIRED"),
        )));
    }

    // Instantiate repository
    let purchases_repo = PurchasesRepoImpl;

    // Create and execute use case
    let use_case = MarkPurchaseAsUnpaidUseCase::new(purchases_repo, Arc::clone(&state.db));

    use_case.execute(&params.purchase_id).await
}

/// Obtener ticket de compra en PDF
///
/// Genera un PDF con la información de la compra y números ganadores
#[utoipa::path(
    get,
    path = "/purchase/{purchase_id}/ticket",
    tags = ["lottery"],
    params(("purchase_id" = Uuid, Path, description = "Identificador de la compra")),
    responses(
        (status = 200, description = "Ticket generado exitosamente", content_type = "application/pdf", body = Vec<u8>),
        (status = 400, description = "Datos inválidos", body = Rfc7807Problem),
        (status = 404, description = "Compra no encontrada", body = Rfc7807Problem)
    )
)]
pub async fn get_purchase_ticket_pdf(
    State(state): State<Arc<AppState>>,
    Path(purchase_id): Path<Uuid>,
) -> Result<Response, Box<Rfc7807Problem>> {
    let purchases_repo = PurchasesRepoImpl;
    let games_repo = GamesRepoImpl;

    let use_case =
        GetPurchaseTicketPdfUseCase::new(purchases_repo, games_repo, Arc::clone(&state.db));

    let pdf_bytes = use_case.execute(&purchase_id).await?;
    let filename = format!("ticket-{}.pdf", purchase_id);

    Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/pdf")
        .header(
            CONTENT_DISPOSITION,
            format!("inline; filename=\"{}\"", filename),
        )
        .body(Body::from(pdf_bytes))
        .map_err(|_| {
            Box::new(Rfc7807Problem::server_error(
                Some("No se pudo generar la respuesta"),
                Some("Ocurrió un error al enviar el archivo"),
                Some("PDF_RESPONSE_ERROR"),
            ))
        })
}

/// @admin Get purchases by game ID
///
/// Fetches paginated purchases with their lottery tickets for a specific game with optional serial filter (Admin only)
#[utoipa::path(
    get,
    path = "/by-game/{game_id}",
    tags = ["lottery"],
    params(
        ("game_id" = Uuid, Path, description = "ID of the game to fetch purchases for"),
        GetLotteriesByGameQueryParams
    ),
    responses(
        (status = 200, description = "Purchases fetched successfully", body = Vec<PurchaseOutputDto>),
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
) -> StandardOutput<Vec<PurchaseOutputDto>> {
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
            Some("Solo los administradores pueden ver las compras de un juego"),
            Some("ADMIN_ACCESS_REQUIRED"),
        )));
    }

    // Instantiate repository
    let purchases_repo = PurchasesRepoImpl;

    // Create and execute use case
    let use_case = GetPurchasesByGameUseCase::new(purchases_repo, Arc::clone(&state.db));

    use_case
        .execute(game_id, params.page, params.per_page, params.serial)
        .await
}
