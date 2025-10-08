use std::sync::Arc;

use axum::Router;

use crate::config::{db::connect_db, tracing::build_tracing};

pub mod adapters;
pub mod shared;
pub mod config;

#[tokio::main]
async fn main() {
    // loading secrets
    let secrets = Arc::new(config::env::SecretsVariables::default());

    // initializing tracing for logs
    build_tracing(&secrets);

    let state = Arc::new(config::state::AppState {
        secrets: Arc::clone(&secrets),
        db: connect_db(&secrets).await
    });

    // defining app router
    let app = Router::new()
        .nest(
            "/api/v1",
            config::routes::build_routes_v1().merge(config::openapi::build_docs()),
        )
        .with_state(Arc::clone(&state));

    // listening on target port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
