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
        // User Profile Routes
        .route("/v1/users", get(handlers::ai::list_users))
        .route("/v1/users/:id/profile", get(handlers::ai::get_user_profile))
        .route(
            "/v1/users/profiles/batch",
            post(handlers::ai::get_user_profiles_batch),
        )
        .route("/v1/users/clusters", get(handlers::ai::get_user_clusters))
        // Task Routes
        .route("/v1/tasks", get(handlers::ai::list_tasks))
        .route(
            "/v1/users/:user_id/tasks",
            get(handlers::ai::get_user_tasks),
        )
        // Schedule Routes
        .route(
            "/v1/users/:user_id/schedule",
            get(handlers::schedule::get_user_schedule),
        )
        .route(
            "/v1/users/:user_id/schedule/month/:month",
            get(handlers::schedule::get_user_schedule_month),
        );

    run(app).await
}
