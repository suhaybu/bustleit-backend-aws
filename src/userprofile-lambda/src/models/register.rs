use serde::Deserialize;

use common::models::database::PersonalityScores;

#[derive(Deserialize)]
pub struct RegisterUserPayload {
    personal: PersonalData,
    routine: RoutineData,
    scores: PersonalityScores,
    preferences: Vec<String>,
}

#[derive(Deserialize)]
pub struct PersonalData {
    name: String,
    email: String,
    date_of_birth: String,
}

#[derive(Deserialize)]
pub struct RoutineData {
    work_time_start: String,
    work_time_end: String,
    sleep_time: String,
}
