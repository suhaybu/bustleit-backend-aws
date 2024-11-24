use axum::{
    middleware,
    routing::{delete, get, patch, post},
    Router,
};
use lambda_http::{run, Error};
use std::env::set_var;

use common::services::mw_auth::auth;
use handlers::schedule::get_user_schedule;
use handlers::tasks::{create_task, delete_task, get_all_tasks, get_tasks_batch, update_task};

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
        .route("/v1/user/:user_id/tasks", post(create_task))
        .route("/v1/user/:user_id/tasks/:task_id", patch(update_task))
        .route("/v1/user/:user_id/tasks/:task_id", delete(delete_task))
        .layer(middleware::from_fn(auth));

    run(app).await
}
