use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Auth(AuthError),
    Database(DbError),
    Validation {
        field: Option<String>,
        reason: String,
    },
    NotFound {
        resource_name: String,
        reason: String,
    },
    Internal(String),
}

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    MissingToken,
    ExpiredToken,
}

#[derive(Debug)]
pub enum DbError {
    ConnectionFailed(String),
    ParseError(String),
    QueryError(String),
    TransactionError(String),
}

impl Error {
    pub fn db_connection_failed(msg: impl Into<String>) -> Self {
        Self::Database(DbError::ConnectionFailed(msg.into()))
    }

    pub fn db_parse_error(msg: impl Into<String>) -> Self {
        Self::Database(DbError::ParseError(msg.into()))
    }

    pub fn db_query_error(msg: impl Into<String>) -> Self {
        Self::Database(DbError::QueryError(msg.into()))
    }

    pub fn db_transaction_error(msg: impl Into<String>) -> Self {
        Self::Database(DbError::TransactionError(msg.into()))
    }

    pub fn validation(reason: impl Into<String>) -> Self {
        Self::Validation {
            field: None,
            reason: reason.into(),
        }
    }

    pub fn field_validation(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::Validation {
            field: Some(field.into()),
            reason: reason.into(),
        }
    }

    pub fn not_found(resource_name: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::NotFound {
            resource_name: resource_name.into(),
            reason: reason.into(),
        }
    }

    pub fn internal(reason: impl Into<String>) -> Self {
        Self::Internal(reason.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Auth(auth_err) => write!(f, "Authentication error {:?}", auth_err),
            Error::Database(db_err) => write!(f, "Database error: {:?}", db_err),
            Error::Validation {
                field: None,
                reason,
            } => {
                write!(f, "Validation error: {}", reason)
            }
            Error::Validation {
                field: Some(field),
                reason,
            } => {
                write!(f, "Validation error on field '{}': {}", field, reason)
            }
            Error::NotFound {
                resource_name: resource_type,
                reason: identifier,
            } => {
                write!(f, "{} not found: {}", resource_type, identifier)
            }
            Error::Internal(reason) => {
                write!(f, "Internal error: {}", reason)
            }
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Log all errors
        tracing::error!(
            error = ?self,
            "API error occurred"
        );

        let (status, error_json) = match self {
            Error::Auth(AuthError::MissingToken) => (
                StatusCode::UNAUTHORIZED,
                json!({
                    "type": "UNAUTHORIZED",
                    "message": "Missing authentication token"
                }),
            ),
            Error::Auth(_) => (
                StatusCode::UNAUTHORIZED,
                json!({
                    "type": "UNAUTHORIZED",
                    "message": "Authentication failed"
                }),
            ),
            Error::Validation { field, reason } => {
                let details = match field {
                    Some(field_name) => json!({
                        "type": "VALIDATION_ERROR",
                        "message": "Validation failed",
                        "field": field_name,
                        "reason": reason
                    }),
                    None => json!({
                        "type": "VALIDATION_ERROR",
                        "message": reason
                    }),
                };
                (StatusCode::BAD_REQUEST, details)
            }
            Error::NotFound {
                resource_name: resource_type,
                reason: identifier,
            } => (
                StatusCode::NOT_FOUND,
                json!({
                    "type": "NOT_FOUND",
                    "message": format!("{} not found", resource_type),
                    "identifier": identifier
                }),
            ),
            Error::Database(db_err) => {
                tracing::error!("Database error: {:?}", db_err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({
                        "type": "DATABASE_ERROR",
                        "message": "Database operation failed"
                    }),
                )
            }
            Error::Internal(reason) => {
                tracing::error!("Internal error: {}", reason);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    json!({
                        "type": "INTERNAL_ERROR",
                        "message": "An internal error occurred"
                    }),
                )
            }
        };

        (status, Json(error_json)).into_response()
    }
}
