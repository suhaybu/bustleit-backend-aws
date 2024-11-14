use axum::{
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use std::env::set_var;

mod handlers;
mod models;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    let app = Router::new()
        .route("/v1/tasks", get(todo!()))
        .route("/v1/users/:user_id/tasks", get(todo!()))
        // Schedule Routes
        .route("/v1/users/:user_id/schedule", get(todo!()))
        .route("/v1/users/:user_id/schedule/month/:month", get(todo!()));

    run(app).await
}
