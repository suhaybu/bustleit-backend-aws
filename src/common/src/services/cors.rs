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
        let mut response: Response<Body> = Response::builder()
            .status(StatusCode::OK)
            .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, origin_value)
            .header(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                "GET, POST, PUT, DELETE, PATCH, OPTIONS",
            )
            .header(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                "Authorization, Content-Type, Accept",
            )
            .header(header::ACCESS_CONTROL_MAX_AGE, "86400")
            .header(header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true")
            .body(Body::empty())
            .unwrap();

        // Ensure the Vary header is set
        response.headers_mut().insert(
            header::VARY,
            HeaderValue::from_static("Origin, Access-Control-Request-Method"),
        );

        return response;
    }

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
        HeaderValue::from_static("Authorization, Content-Type, Accept"),
    );
    headers.insert(
        header::VARY,
        HeaderValue::from_static("Origin, Access-Control-Request-Method"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );

    response
}
