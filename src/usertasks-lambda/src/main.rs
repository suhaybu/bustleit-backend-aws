use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use std::env::set_var;

use common::services::mw_auth::auth;
use handlers::{get_all_tasks, get_tasks_batch, get_user_schedule};

mod db;
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
        .route("/v1/user/:user_id/schedule", get(get_user_schedule))
        .route("/v1/tasks", get(get_all_tasks))
        .route("/v1/tasks/batch", post(get_tasks_batch))
        .layer(middleware::from_fn(auth));
    // .route("/v1/users/:user_id/schedule/month/:month", get(todo!()));

    run(app).await
}
