use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UserProfileRequest {
    pub ids: Vec<String>,
    pub req_scores: bool,
    pub req_preferences: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: String,
    pub scores: PersonalityScores,
    pub preferences: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonalityScores {
    pub introverted: f32,
    pub extraverted: f32,
    pub observant: f32,
    pub intuitive: f32,
    pub thinking: f32,
    pub feeling: f32,
    pub judging: f32,
    pub prospecting: f32,
    pub assertive: f32,
    pub turbulent: f32,
}
