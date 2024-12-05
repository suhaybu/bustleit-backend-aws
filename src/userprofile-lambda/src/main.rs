use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use std::env::set_var;

use common::services::{cors::cors_middleware, mw_auth::auth};
use handlers::{profile, profiles, recommend, register};

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
        .route("/v1/signup", post(register::create_user_profile))
        .route("/v1/user/profile/:id", get(profile::get_profile))
        .route("/v1/user/profiles", get(profiles::get_profiles))
        .route("/v1/user/cluster/update", post(recommend::update_cluser))
        .route("/v1/user/profiles/batch", post(profiles::get_batch))
        .route("/v1/cluster/:user_id", get(recommend::cluster_user))
        .route("/v1/rank/:user_id", get(recommend::rank_user))
        .route("/v1/recommend/:user_id", get(recommend::get_recommendation))
        .route(
            "/v1/recommend/:user_id/week",
            get(recommend::get_recommendation_week),
        )
        .layer(middleware::from_fn(cors_middleware))
        .layer(middleware::from_fn(auth));

    run(app).await
}
