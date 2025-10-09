use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use utoipa::OpenApi;

use crate::{
    adapters::inbounds::rest::{dtos::lottery::LotteriesPurchaseDto}, config::state::AppState,
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
    responses(
        (status = 201, description = "Lotteries purchased", body = String),
        (status = 400, description = "Error purchasing lotteries", body = Rfc7807Problem)
    )
)]
pub async fn purchase_lottery(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LotteriesPurchaseDto>,
) -> StandardOutput<String>{
    todo!()
}
