
use std::sync::Arc;

use axum::{response::Html, routing::get};
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use crate::{adapters::inbounds::rest::health_controller, config::{routes, state::AppState}, shared::errors::{Rfc7807Problem, ValidatorError, ValidatorErrorFields}};

pub fn build_docs() -> axum::Router<Arc<AppState>> {
    axum::Router::new()
        .route("/scalar", get(build_scalar_html))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
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
        // (path = "/users", api = crate::controllers::users::ApiDoc),
        // (path = "/auth", api = crate::controllers::session::ApiDoc),
        // (path = "/locations", api = crate::controllers::locations::ApiDoc),
        // (path = "/professions", api = crate::controllers::professions::ApiDoc),
        // (path = "/specialties", api = crate::controllers::specialties::ApiDoc),
        // (path = "/schedules", api = crate::controllers::scheduler::ApiDoc),
        // (path = "/appointments", api = crate::controllers::appointments::ApiDoc),
        (path = "/health", api = health_controller::ApiDoc),
    ),
    tags(
        (name = "infra", description = "Infra state"),
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
        // CreateUserDto,
        Rfc7807Problem,
        ValidatorError,
        ValidatorErrorFields
    )),
    servers( (url = "/api/v1", description = "API v1 base") )
)]
pub struct ApiDoc;
