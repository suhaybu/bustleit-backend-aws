use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use std::env::set_var;

use common::services::mw_auth::auth;
use handlers::{profile, profiles};

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
        .route("/v1/user/profile/:id", get(profile::get_profile))
        .route("/v1/user/profiles", get(profiles::get_profiles))
        .route("/v1/user/profiles/batch", post(profiles::get_batch))
        .layer(middleware::from_fn(auth));

    run(app).await
}
