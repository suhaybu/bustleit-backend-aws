use axum::{
    body::Body,
    http::{header, HeaderValue, Method, Request, Response, StatusCode},
    middleware::Next,
};

pub async fn cors_middleware(req: Request<Body>, next: Next) -> Response<Body> {
    let origin = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("*");

    let origin_value =
        HeaderValue::from_str(origin).unwrap_or_else(|_| HeaderValue::from_static("*"));

    // Handle preflight requests
    if req.method() == Method::OPTIONS {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value)
            .header(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                "GET, POST, PUT, DELETE, PATCH, OPTIONS",
            )
            .header(header::ACCESS_CONTROL_ALLOW_HEADERS, "*")
            .header(header::ACCESS_CONTROL_MAX_AGE, "86400")
            .header(header::VARY, "Origin, Access-Control-Request-Method")
            .body(Body::empty())
            .unwrap()
    } else {
        let mut response = next.run(req).await;

        // Add CORS headers to the response
        let headers = response.headers_mut();
        headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value);
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, POST, PUT, DELETE, PATCH, OPTIONS"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*"),
        );
        headers.insert(
            header::VARY,
            HeaderValue::from_static("Origin, Access-Control-Request-Method"),
        );

        response
    }
}
