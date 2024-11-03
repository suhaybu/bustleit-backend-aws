use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    status: String,
    token: Option<String>,
}

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
