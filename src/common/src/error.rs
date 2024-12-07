use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::{error, warn};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("{0} not found")]
    NotFound(String),

    #[error("Internal Server error: {0}")]
    InternalServerError(String),
}

impl Error {
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            Error::Database(e) => {
                error!(error = %e, "Database error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
            Error::Validation(msg) => {
                warn!(message = %msg, "Validation error");
                (StatusCode::BAD_REQUEST, msg.clone())
            }
            Error::NotFound(msg) => {
                warn!(message = %msg, "Resource not found");
                (StatusCode::NOT_FOUND, msg.clone())
            }
            Error::InternalServerError(msg) => {
                error!(error = %msg, "Internal server error occurred");
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}
