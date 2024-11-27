use serde::Deserialize;

use common::models::database::PersonalityScores;

#[derive(Deserialize, Debug)]
pub struct RegisterUserPayload {
    personal: PersonalData,
    routine: RoutineData,
    scores: PersonalityScores,
    preferences: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct PersonalData {
    name: String,
    email: String,
    date_of_birth: String,
}

#[derive(Deserialize, Debug)]
pub struct RoutineData {
    work_time_start: String,
    work_time_end: String,
    sleep_time: String,
}
