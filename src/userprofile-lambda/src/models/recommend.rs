use chrono::{NaiveTime, Timelike};
use serde::{Deserialize, Serialize};

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
    preferences: Vec<String>,
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
        preferences: Vec<String>,
        cluster: i32,
        work_start_time: NaiveTime,
        work_end_time: NaiveTime,
        sleep_time: NaiveTime,
    ) -> Self {
        let time_to_numeric = |t: NaiveTime| -> i32 {
            let hour = t.hour() as i32;
            let minute = t.minute() as i32;
            hour * 100 + minute
        };

        Self {
            user_id,
            scores,
            preferences,
            cluster,
            work_start_time: time_to_numeric(work_start_time),
            work_end_time: time_to_numeric(work_end_time),
            sleep_time: time_to_numeric(sleep_time),
        }
    }
}
