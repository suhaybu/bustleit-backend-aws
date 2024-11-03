use axum::{
    routing::{get, post},
    Router,
};
use lambda_http::{run, tracing, Error};
use std::env::set_var;

mod handlers;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // If you use API Gateway stages, the Rust Runtime will include the stage name
    // as part of the path that your application receives.
    // Setting the following environment variable, you can remove the stage from the path.
    // This variable only applies to API Gateway stages,
    // you can remove it if you don't use them.
    // i.e with: `GET /test-stage/todo/id/123` without: `GET /todo/id/123`
    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    // Build the router
    let app = Router::new()
        // Auth routes
        .route("/auth/login", post(handlers::auth::login))
        // AI routes
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
        )
        // Recommend routes
        .route("/recommend", post(handlers::recommend::get_recommendation))
        // .route(
        //     "/recommend/test",
        //     get(handlers::recommend::get_test_recommendation),
        // )
        // Profile routes
        // .route(
        //     "/profile",
        //     get(handlers::profile::get_profile).post(handlers::profile::update_profile),
        // )
        // Health check
        .route("/health", get(handlers::health::check));

    // Run it with Lambda runtime
    run(app).await
}
