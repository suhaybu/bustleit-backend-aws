use std::env;

use axum::{
    extract::Request,
    http::{header, Method, StatusCode},
    middleware::Next,
    response::Response,
};

const TEST_API_TOKEN: &str = "TEST_API_TOKEN";

pub async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    // Skip auth for OPTIONS requests
    if req.method() == Method::OPTIONS {
        return Ok(next.run(req).await);
    }

    let valid_token = env::var(TEST_API_TOKEN).map_err(|_| {
        tracing::error!("API_AUTH_TOKEN not configuired");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    // Makes sure auth header isn't missing
    let auth_header = if let Some(header) = auth_header {
        header
    } else {
        tracing::warn!("Missing Authorization header");
        return Err(StatusCode::UNAUTHORIZED);
    };

    // Makes sure auth header has the correct bearer token format
    if let Some(token) = auth_header.strip_prefix("Bearer ") {
        if token == valid_token {
            Ok(next.run(req).await)
        } else {
            tracing::warn!("Invalid token provided");
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        tracing::warn!("Invalid Authorization header format");
        Err(StatusCode::UNAUTHORIZED)
    }
}
