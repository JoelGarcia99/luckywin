use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use serde_valid::Validate;
use utoipa::OpenApi;

use crate::{
    adapters::{
        inbounds::rest::dtos::lottery::LotteriesPurchaseDto,
        outbound::{
            db::{clients::ClientsRepoImpl, games::GamesRepoImpl, lotteries::LotteriesRepoImpl},
            rest::dtos::lottery::LotteryOutputDto,
        },
    },
    config::state::AppState,
    domain::usecases::purchase_lottery::PurchaseLotteryUseCase,
    shared::errors::{Rfc7807Problem, StandardOutput},
};

#[derive(OpenApi)]
#[openapi(paths(purchase_lottery))]
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
