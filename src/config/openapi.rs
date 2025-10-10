use std::sync::Arc;

use axum::{response::Html, routing::get};
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
            client::ClientOutputDto, game::GameOutputDto, lottery::LotteryOutputDto, users::CurrentUserDto,
        },
    },
    config::state::AppState,
    shared::errors::{Rfc7807Problem, ValidatorError, ValidatorErrorFields},
};

pub fn build_docs() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/docs", get(build_scalar_html))
        .merge(SwaggerUi::new("/swagger-docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
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
    // Intercept fetch to auto-save login tokens
    const originalFetch = window.fetch;
    window.fetch = async function(...args) {
      const response = await originalFetch(...args);

      // Clone the response so we can read it without consuming it
      const clonedResponse = response.clone();

      try {
        const url = typeof args[0] === 'string' ? args[0] : args[0].url;

        // Check if this is a login endpoint
        if (url.includes('/auth/login') && response.ok) {
          const data = await clonedResponse.json();

          if (data.data?.access_token) {
            localStorage.setItem('access_token', data.data.access_token);
            localStorage.setItem('refresh_token', data.data.refresh_token);
            console.log('✅ Token auto-saved!');

            // Show a notification
            const notification = document.createElement('div');
            notification.style.cssText = 'position:fixed;top:20px;right:20px;background:#10b981;color:white;padding:16px 24px;border-radius:8px;box-shadow:0 4px 6px rgba(0,0,0,0.1);z-index:9999;font-family:system-ui;';
            notification.textContent = 'Token saved! Reloading...';
            document.body.appendChild(notification);

            // Reload after a short delay
            setTimeout(() => window.location.reload(), 1500);
          }
        }
      } catch (e) {
        // Ignore errors (response might not be JSON)
      }

      return response;
    };

    // Mount Scalar with auth configuration
    const configuration = {
      url: '/api/v1/api-docs/openapi.json',
      layout: 'modern',
      authentication: {
        preferredSecurityScheme: 'bearer_auth',
        http: {
          bearer: {
            token: localStorage.getItem('access_token') || ''
          }
        }
      }
    };

    Scalar.createApiReference('#app', configuration);

    // 4) Debug: surface errors
    window.addEventListener('unhandledrejection', e => console.error('Promise error:', e.reason));
  </script>
</body>
</html>
        "#,
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
        CurrentUserDto,
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
