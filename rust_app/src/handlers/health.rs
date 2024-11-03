use axum::{http::StatusCode, Json};
use serde_json::Value;

pub async fn check() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "healthy",
            "version": env!("CARGO_PKG_VERSION"),
        })),
    )
}
