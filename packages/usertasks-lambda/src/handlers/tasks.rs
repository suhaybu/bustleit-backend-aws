use axum::{http::StatusCode, Json};

fn handle_error<E: std::fmt::Display>(err: E) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(serde_json::json!({
            "error": err.to_string()
        })),
    )
}
