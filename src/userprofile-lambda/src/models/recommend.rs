use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
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
    work_start_time: i32,
    work_end_time: i32,
    sleep_time: i32,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseRecommendDaily {
    date: String,
    day: String,
    tasks: Vec<Task>,
}

#[derive(Deserialize, Serialize)]
pub struct Task {
    #[serde(rename = "endTime")]
    end_time: String,
    name: String,
    #[serde(rename = "startTime")]
    start_time: String,
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

impl RequestRecommendDaily {
    pub fn new(
        user_id: Uuid,
        scores: DB::PersonalityScores,
        cluster: i32,
        work_start_time: DateTime<Utc>,
        work_end_time: DateTime<Utc>,
        sleep_time: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            scores,
            cluster,
            work_start_time: work_start_time
                .format("%H%M")
                .to_string()
                .parse::<i32>()
                .unwrap(),
            work_end_time: work_end_time
                .format("%H%M")
                .to_string()
                .parse::<i32>()
                .unwrap(),
            sleep_time: sleep_time
                .format("%H%M")
                .to_string()
                .parse::<i32>()
                .unwrap(),
        }
    }
}
