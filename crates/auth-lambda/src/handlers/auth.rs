use axum::{http::StatusCode, Json};

use crate::models::auth::{LoginRequest, LoginResponse};

pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, Json<LoginResponse>)> {
    // Here you would normally validate against DynamoDB
    if payload.username == "test" && payload.password == "test" {
        Ok((
            StatusCode::OK,
            Json(LoginResponse {
                status: "success".to_string(),
                token: Some("dummy-token".to_string()),
            }),
        ))
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(LoginResponse {
                status: "error".to_string(),
                token: None,
            }),
        ))
    }
}
