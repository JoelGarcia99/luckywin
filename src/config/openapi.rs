use std::sync::Arc;

use axum::{response::Html, routing::get, Json};
use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    adapters::{
        inbounds::rest::{
            controllers,
            dtos::{
                auth::{LoginInputDto, LoginOutputDto, LogoutOutputDto, RefreshTokenInputDto},
                client::ClientsInputDto,
                game::CreateGameDto,
                lottery::{CreateLotteryTicketDto, LotteriesPurchaseDto},
                user::UpdateUserDto,
            },
        },
        outbound::rest::dtos::{
            client::ClientOutputDto, game::GameOutputDto, lottery::LotteryOutputDto, users::UserOutputDto,
        },
    },
    config::state::AppState,
    shared::errors::{Rfc7807Problem, ValidatorError, ValidatorErrorFields},
};

pub fn build_docs() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/docs", get(build_scalar_html))
        .route("/api-docs/openapi.json", get(openapi))
}

async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

async fn build_scalar_html() -> Html<&'static str> {
    Html(
r#"
<!doctype html>
<html>
<head>
  <meta charset="utf-8"/>
  <meta name="viewport" content="width=device-width,initial-scale=1"/>
  <title>Users API</title>
  <style>
    html,body,#app{height:100%;margin:0}
    input{flex:1}
  </style>
</head>
<body>
  <div id="app"></div>

  <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference"></script>
<script>
    // 3) Mount Scalar AFTER the script is loaded and #app exists.
    // Use the KrakenD path that returns 200 in your browser.
    Scalar.createApiReference('#app', {
      url: '/api/v1/api-docs/openapi.json',
      layout: 'modern'
    });

    // 4) Debug: surface errors
    window.addEventListener('unhandledrejection', e => console.error('Promise error:', e.reason));
  </script>
</body>
</html>
"#
    )
}

pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let bearer = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );
        openapi
            .components
            .get_or_insert_with(Default::default)
            .add_security_scheme("bearer_auth", bearer);
    }
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/health", api = controllers::health::ApiDoc),
        (path = "/auth", api = controllers::auth::ApiDoc),
        (path = "/games", api = controllers::game::ApiDoc),
        (path = "/lottery", api = controllers::lottery::ApiDoc),
        (path = "/users", api = controllers::user::ApiDoc),
    ),
    tags(
        (name = "infra", description = "Infra state"),
        (name = "games", description = "Games management"),
        (name = "lottery", description = "Lottery associated endpoints"),
        (name = "auth", description = "Authorization & authentication management"),
        (name = "users", description = "Users management"),
        (name = "locations", description = "Locations management"),
        (name = "professions", description = "Professions management"),
        (name = "specialties", description = "Specialties management"),
        (name = "scheduler", description = "Schedules and time windows management"),
        (name = "appointments", description = "Appointments management"),
        (name = "prescriptions", description = "Prescriptions management"),
    ),
    modifiers(&SecurityAddon),
    components(schemas(
        // Auth DTOs
        LoginInputDto,
        LoginOutputDto,
        RefreshTokenInputDto,
        LogoutOutputDto,
        UserOutputDto,
        UpdateUserDto,
        // Client DTOs
        ClientsInputDto,
        ClientOutputDto,
        // Game DTOs
        CreateGameDto,
        GameOutputDto,
        // Lottery DTOs
        CreateLotteryTicketDto,
        LotteryOutputDto,
        LotteriesPurchaseDto,
        // Error DTOs
        Rfc7807Problem,
        ValidatorError,
        ValidatorErrorFields,
    )),
    servers( (url = "/api/v1", description = "API v1 base") )
)]
pub struct ApiDoc;
