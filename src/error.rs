use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use std::env;
use std::error::Error as StdError;
use thiserror::Error;
use validator::ValidationErrors;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] env::VarError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("SeaORM error: {0}")]
    SeaOrm(#[from] sea_orm::DbErr),

    #[error("Axum HTTP error: {0}")]
    AxumHttp(#[from] axum::http::Error),

    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheus::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Validation error")]
    Validation {
        #[source]
        errors: ValidationErrors,
    },

    #[error("Internal server error")]
    InternalServer {
        #[from]
        source: Box<dyn StdError + Send + Sync>,
    },
}

impl From<ValidationErrors> for Error {
    fn from(errors: ValidationErrors) -> Self {
        Error::Validation { errors }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Vec<ValidationErrorDetail>>,
}

#[derive(Serialize)]
struct ValidationErrorDetail {
    field: String,
    message: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_response) = match &self {
            Error::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    error: "Bad Request".into(),
                    message: msg.clone(),
                    details: None,
                },
            ),

            Error::Io(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "Internal Server Error".into(),
                    message: "I/O error".into(),
                    details: None,
                },
            ),

            Error::Validation { errors } => {
                let details = errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, errors)| ValidationErrorDetail {
                        field: field.to_string(),
                        message: errors
                            .iter()
                            .filter_map(|e| e.message.as_ref())
                            .map(|m| m.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    })
                    .collect();

                (
                    StatusCode::BAD_REQUEST,
                    ErrorResponse {
                        error: "Validation Error".into(),
                        message: "Invalid input data".into(),
                        details: Some(details),
                    },
                )
            }

            Error::InternalServer { source } => {
                tracing::error!(%source, "Internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        error: "Internal Server Error".into(),
                        message: "Something went wrong".into(),
                        details: None,
                    },
                )
            }
            Error::EnvVar(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "Internal Server Error".into(),
                    message: "Environment variable error".into(),
                    details: None,
                },
            ),
            Error::Sqlx(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "Internal Server Error".into(),
                    message: "Database error".into(),
                    details: None,
                },
            ),
            Error::SeaOrm(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "Internal Server Error".into(),
                    message: "SeaORM error".into(),
                    details: None,
                },
            ),
            Error::AxumHttp(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "Internal Server Error".into(),
                    message: "Axum HTTP error".into(),
                    details: None,
                },
            ),
            Error::Prometheus(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error: "Internal Server Error".into(),
                    message: "Prometheus error".into(),
                    details: None,
                },
            ),
            Error::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    error: "Unauthorized".into(),
                    message: msg.clone(),
                    details: None,
                },
            ),
        };

        // NOTE: Don't expose any details about the error to the client
        if status.is_server_error() {
            tracing::error!(error = ?self);
        }

        (status, Json(error_response)).into_response()
    }
}

impl From<JsonRejection> for Error {
    fn from(rejection: JsonRejection) -> Self {
        let msg = match rejection {
            JsonRejection::MissingJsonContentType(_) => "Missing JSON content type".into(),
            JsonRejection::JsonSyntaxError(_) => "Invalid JSON syntax".into(),
            _ => "Invalid request".into(),
        };
        Error::BadRequest(msg)
    }
}
