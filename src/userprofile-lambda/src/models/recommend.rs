use chrono::{NaiveTime, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use common::models::database as DB;

// Used for calling external_api/cluster
#[derive(Serialize)]
pub struct RequestClusterUser {
    user_id: Uuid,
    scores: DB::PersonalityScores,
    preferences: Vec<String>,
}

// Used for calling external_api/rank
pub struct RequestRankUser {
    user_id: Uuid,
    scores: DB::PersonalityScores,
    preferences: Vec<String>,
    cluster: i32,
}

// Used for calling external_api
#[derive(Serialize)]
pub struct RequestRecommend {
    user_id: Uuid,
    scores: DB::PersonalityScores,
    preferences: Vec<String>,
    cluster: i32,
    work_end_time: i32,
    work_start_time: i32,
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

#[derive(Serialize, Deserialize)]
pub struct ResponseRecommendWeekly {
    #[serde(flatten)]
    pub days: HashMap<String, DaySchedule>,
}

#[derive(Serialize, Deserialize)]
pub struct DaySchedule {
    pub date: String,
    pub day: String,
    pub tasks: Vec<Task>,
}

impl RequestRecommend {
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
