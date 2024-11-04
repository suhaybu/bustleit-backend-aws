use axum::{
    routing::{get, post},
    Router,
};
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

    let app = Router::new()
        .route("/ai/request/userdata", post(handlers::ai::get_userdata))
        .route("/ai/request/allusers", get(handlers::ai::get_all_users))
        .route(
            "/ai/request/clusteredUsers",
            get(handlers::ai::get_clustered_users),
        )
        .route("/ai/request/schedules", post(handlers::ai::get_schedules))
        .route("/ai/request/tasks", get(handlers::ai::get_tasks))
        .route(
            "/ai/request/taskedUsers",
            get(handlers::ai::get_tasked_users),
        );

    run(app).await
}
