use serde::Serialize;
use uuid::Uuid;

use common::models::database as DB;

// Used for calling external_api/cluster
#[derive(Serialize)]
pub struct RequestClusterUser {
    user_id: Uuid,
    scores: DB::PersonalityScores,
    preferences: Vec<String>,
}

// Used for calling external_api/recommend_daily
#[derive(Serialize)]
pub struct RequestRecommendDaily {
    user_id: Uuid,
    scores: DB::PersonalityScores,
    cluster: i32,
    work_end_time: i32,
    sleep_time: i32,
}

// Used for calling external_api/recommend_weekly
#[derive(Serialize)]
pub struct RequestRecommendWeekly {
    user: UserInWeeklyRequest,
    work_end_time: i32,
    work_start_time: i32,
    sleep_time: i32,
}

// Helper datatype used ONLY for struct RequestRecommendWeekly
#[derive(Serialize)]
pub struct UserInWeeklyRequest {
    user_id: Uuid,
    scores: DB::PersonalityScores,
    preferences: Vec<String>,
    cluster: i32,
}
