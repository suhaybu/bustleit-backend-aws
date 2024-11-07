use axum::{http::StatusCode, Json};
use rand::Rng;
use tracing::info;

use common::models::ai::{RecommendationInput, RecommendationResponse};

// POST /recommend
pub async fn get_recommendation(
    Json(payload): Json<RecommendationInput>,
) -> Result<Json<RecommendationResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!(
        "Received recommendation request for work_end_time: {}",
        payload.work_end_time
    );

    // Get recommendations from mock service
    let recommendations = MockDb::get_recommendations(
        payload.scores,
        payload.preferences,
        payload.work_end_time,
        payload.sleep_time,
    )
    .await;

    let total_tasks = recommendations.len();

    Ok(Json(RecommendationResponse {
        recommendations,
        total_tasks,
    }))
}

// GET /recommend (test endpoint)
pub async fn get_test_recommendation(
) -> Result<Json<RecommendationResponse>, (StatusCode, Json<serde_json::Value>)> {
    info!("Generating test recommendation");

    // Generate random test data
    let test_scores: Vec<f32> = {
        let mut rng = rand::thread_rng();
        (0..10).map(|_| rng.gen_range(0.0..100.0)).collect()
    };

    let test_preferences = vec![
        "Health".to_string(),
        "Learning".to_string(),
        "Exercise".to_string(),
        "Finance".to_string(),
    ];
    let test_work_end_time = 18; // 6 PM
    let test_sleep_time = 23; // 11 PM

    // Get recommendations using test data
    let recommendations = MockDb::get_recommendations(
        test_scores,
        test_preferences,
        test_work_end_time,
        test_sleep_time,
    )
    .await;

    let total_tasks = recommendations.len();

    Ok(Json(RecommendationResponse {
        recommendations,
        total_tasks,
    }))
}
