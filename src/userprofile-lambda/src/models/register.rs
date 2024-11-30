use serde::Deserialize;

use common::models::database as DB;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RegisterUserPayload {
    routine: RoutineData,
    scores: DB::PersonalityScores,
    preferences: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RoutineData {
    work_time_start: String,
    work_time_end: String,
    sleep_time: String,
}
