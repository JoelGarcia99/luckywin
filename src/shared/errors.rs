use std::collections::HashMap;

use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ValidatorErrorFields {
    errors: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ValidatorError {
    properties: HashMap<String, ValidatorErrorFields>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct Rfc7807Problem {
    /// URI identifier (RFC 7807). This URI is not callable, it's just an identifier. No HTML
    /// template is hosted here
    #[schema(example = "https://my-api.com/v1/validation-error")]
    pub r#type: String,

    /// Short description of the error in human readable form
    #[schema(example = "Validation error")]
    pub title: String,

    /// It must match the HTTP status code on headers
    #[schema(example = 400)]
    pub status: u16,

    /// More detailed message about the problem
    #[schema(example = "There is a problem with one of your fields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    /// It's a code for the message. IT'S NOT THE STATUS CODE. It's a code indicating the exact
    /// message that must be rendered. Useful for internalization later.
    #[schema(example = "BAD_REQUEST")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,

    /// More detailed message about the problem for specific fields
    #[schema(example = json!({"email": ["Invalid email"]}))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<HashMap<String, Vec<String>>>,
}

impl Rfc7807Problem {
    pub fn status(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn simple_bad_request(
        title: Option<&str>,
        details: Option<&str>,
        custom_err_code: Option<&str>,
    ) -> Self {
        Self {
            r#type: "/v1/bad-body".into(),
            title: title
                .unwrap_or("Ha ocurrido un error procesando sus datos")
                .into(),
            status: StatusCode::BAD_REQUEST.as_u16(),
            detail: details.map(|d| d.to_string()),
            error_code: custom_err_code
                .map(|c| c.to_string())
                .or(Some("BAD_BODY".into())),
            errors: None,
        }
    }

    pub fn bad_request(
        title: Option<&str>,
        details: Option<&str>,
        errors: Option<HashMap<String, Vec<String>>>,
    ) -> Self {
        Self {
            r#type: "/v1/bad-body".into(),
            title: title
                .unwrap_or("Los datos ingresados contienen errores")
                .into(),
            status: StatusCode::BAD_REQUEST.as_u16(),
            detail: details.map(|d| d.to_string()),
            error_code: Some("FORM_ERROR".into()),
            errors,
        }
    }

    pub fn server_error(
        title: Option<&str>,
        details: Option<&str>,
        custom_err_code: Option<&str>,
    ) -> Self {
        Self {
            r#type: "/v1/server-error".into(),
            title: title.unwrap_or("Server error").into(),
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            detail: details.map(|d| d.to_string()),
            error_code: custom_err_code
                .map(|c| c.to_string())
                .or(Some("INTERNAL_SERVER_ERROR".into())),
            errors: None,
        }
    }

    pub fn unauthorized(
        title: Option<&str>,
        details: Option<&str>,
        custom_err_code: Option<&str>,
    ) -> Self {
        Self {
            r#type: "/v1/unauthorized".into(),
            title: title.unwrap_or("Unauthorized").into(),
            status: StatusCode::UNAUTHORIZED.as_u16(),
            detail: details.map(|d| d.to_string()),
            error_code: custom_err_code
                .map(|c| c.to_string())
                .or(Some("UNAUTHORIZED".into())),
            errors: None,
        }
    }

    pub fn missing_header(header_name: &str) -> Self {
        Self {
            r#type: "/v1/missing-header".to_string(),
            title: "Missing header".to_string(),
            status: StatusCode::BAD_REQUEST.as_u16(),
            detail: Some(format!("Missing header: {}", header_name)),
            error_code: Some("MISSING_HEADER".to_string()),
            errors: None,
        }
    }

    pub fn wrong_path_parameter(parameter_name: &str, details: Option<&str>) -> Self {
        Self {
            r#type: "/v1/wrong-path-parameter".into(),
            title: format!("El parámetro {} es inválido", parameter_name),
            status: StatusCode::BAD_REQUEST.as_u16(),
            detail: details.map(|d| d.to_string()),
            error_code: Some("INVALID_PARAMETER".into()),
            errors: None,
        }
    }

    pub fn forbidden_action(title: Option<&str>, details: Option<&str>) -> Self {
        Self {
            r#type: "/v1/forbidden-action".into(),
            title: title
                .unwrap_or("No tiene permisos para realizar esta acción")
                .into(),
            status: StatusCode::FORBIDDEN.as_u16(),
            detail: details.map(|d| d.to_string()),
            error_code: Some("FORBIDDEN".into()),
            errors: None,
        }
    }

    pub fn not_found(title: Option<&str>, details: Option<&str>) -> Self {
        Self {
            r#type: "/v1/not-found".into(),
            title: title.unwrap_or("Not found").into(),
            status: StatusCode::NOT_FOUND.as_u16(),
            detail: details.map(|d| d.to_string()),
            error_code: Some("RESOURCE_NOT_FOUND".into()),
            errors: None,
        }
    }

    pub fn from_validator_err(errors: &str) -> Self {
        let structured = match serde_json::from_str::<ValidatorError>(errors) {
            Ok(e) => e,
            Err(e) => {
                return Rfc7807Problem::server_error(
                    Some("Ha ocurrido un error procesando sus datos"),
                    Some(&e.to_string()),
                    None,
                )
            }
        };

        let mut err_hm = HashMap::new();

        for (error, messages) in structured.properties {
            err_hm.insert(error, messages.errors);
        }

        Self {
            r#type: "/v1/bad-body".into(),
            title: "Los datos ingresados contienen errores".into(),
            status: StatusCode::BAD_REQUEST.as_u16(),
            detail: Some(errors.to_string()),
            error_code: Some("FORM_ERROR".into()),
            errors: Some(err_hm),
        }
    }
}

impl IntoResponse for Rfc7807Problem {
    fn into_response(self) -> Response {
        let status = self.status();
        let mut resp = Json(self).into_response();
        resp.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            // official IETF standard error format for HTTP APIs (RFC 7807)
            "application/problem+json".parse().unwrap(),
        );
        *resp.status_mut() = status;
        resp
    }
}

impl IntoResponse for Box<Rfc7807Problem> {
    fn into_response(self) -> Response {
        (*self).into_response()
    }
}
