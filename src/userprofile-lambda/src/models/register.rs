use serde::Deserialize;

use common::models::database::PersonalityScores;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RegisterUserPayload {
    personal: PersonalData,
    routine: RoutineData,
    scores: PersonalityScores,
    preferences: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PersonalData {
    name: String,
    email: String,
    date_of_birth: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RoutineData {
    work_time_start: String,
    work_time_end: String,
    sleep_time: String,
}
