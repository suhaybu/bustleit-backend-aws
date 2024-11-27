use axum::{
    body::Body,
    http::{header, HeaderValue, Method, Request},
    middleware::Next,
    response::Response,
};

pub async fn cors_middleware(req: Request<Body>, next: Next) -> Response {
    let origin = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("*");
    let origin_value =
        HeaderValue::from_str(origin).unwrap_or_else(|_| HeaderValue::from_static("*"));

    if req.method() == Method::OPTIONS {
        Response::builder()
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value)
            .header(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_static("GET, POST, PUT, DELETE, PATCH, OPTIONS"),
            )
            .header(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                HeaderValue::from_static("*"),
            )
            .header(
                header::ACCESS_CONTROL_MAX_AGE,
                HeaderValue::from_static("86400"),
            )
            .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "false")
            .body(Body::empty())
            .unwrap()
    } else {
        let mut response = next.run(req).await;
        let headers = response.headers_mut();
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value);
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("false"),
        );
        response
    }
}
