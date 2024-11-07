use axum::{routing::post, Router};
use lambda_http::{run, Error};
use std::env::set_var;

mod handlers;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    let app = Router::new().route("/auth/login", post(handlers::auth::login));

    run(app).await
}
