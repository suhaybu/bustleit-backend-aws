use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub user_id: Uuid,
    pub cluster: i32,
    pub preferences: Vec<String>,
    pub personality_scores: JsonValue, // this is a single JSONB in Postgres
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalityScores {
    pub turbulent: f32,
    pub introverted: f32,
    pub prospecting: f32,
    pub judging: f32,
    pub assertive: f32,
    pub feeling: f32,
    pub extraverted: f32,
    pub observant: f32,
    pub thinking: f32,
    pub intuitive: f32,
}

impl Profile {
    // Helper function to give us types and access to personality_scores
    pub fn get_typed_scores(&self) -> Option<PersonalityScores> {
        serde_json::from_value(self.personality_scores.clone()).ok()
    }
}

impl Default for PersonalityScores {
    // If PersonalityScores does not exist, backtrack to this
    fn default() -> Self {
        Self {
            turbulent: 0.0,
            introverted: 0.0,
            prospecting: 0.0,
            judging: 0.0,
            assertive: 0.0,
            feeling: 0.0,
            extraverted: 0.0,
            observant: 0.0,
            thinking: 0.0,
            intuitive: 0.0,
        }
    }
}
