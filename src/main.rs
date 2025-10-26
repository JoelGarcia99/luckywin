use std::sync::Arc;

use axum::Router;
use reqwest::Client;
use tower_http::{cors::CorsLayer, normalize_path::NormalizePathLayer};

use crate::config::{db::connect_db, tracing::build_tracing};

pub mod adapters;
pub mod config;
pub mod domain;
pub mod shared;

#[tokio::main]
async fn main() {
    // loading secrets
    let secrets = Arc::new(config::env::SecretsVariables::default());

    // initializing tracing for logs
    build_tracing(&secrets);
    tracing::debug!("Tracings defined");

    let http_client = Arc::new(Client::new());

    let state = Arc::new(config::state::AppState {
        secrets: Arc::clone(&secrets),
        db: connect_db(&secrets).await,
        http_client,
    });

    // Configure CORS based on environment
    let cors_layer = if state.secrets.environment.to_lowercase() == "prod" {
        // Production: only allow juegalofacil.com
        CorsLayer::new()
            .allow_origin([
                "https://juegalofacil.com"
                    .parse::<axum::http::HeaderValue>()
                    .unwrap(),
                "https://www.juegalofacil.com"
                    .parse::<axum::http::HeaderValue>()
                    .unwrap(),
            ])
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::PUT,
                axum::http::Method::DELETE,
                axum::http::Method::PATCH,
                axum::http::Method::OPTIONS,
            ])
            .allow_headers([
                axum::http::header::CONTENT_TYPE,
                axum::http::header::AUTHORIZATION,
                axum::http::header::ACCEPT,
            ])
            .allow_credentials(true)
    } else {
        // Development/other: permissive CORS
        CorsLayer::permissive()
    };

    // defining app router
    let app = Router::new()
        .nest(
            "/api/v1",
            config::routes::build_routes_v1().merge(config::openapi::build_docs()),
        )
        .with_state(Arc::clone(&state))
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(cors_layer);

    // listening on target port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("listening on http://0.0.0.0:8080");
    axum::serve(listener, app).await.unwrap();
}
