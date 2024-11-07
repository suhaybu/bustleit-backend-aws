use serde::{Deserialize, Serialize};

use crate::models::dynamodb::UserProfile as DbUserProfile;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UserProfileRequest {
    pub user_ids: Vec<String>,
    pub req_scores: bool,
    pub req_preferences: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub user_id: String,
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

// Convert from DynamoDB UserProfile to userdata UserProfile
impl From<&DbUserProfile> for UserProfile {
    fn from(db_profile: &DbUserProfile) -> Self {
        Self {
            user_id: db_profile.pk.trim_start_matches("USER#").to_string(),
            preferences: db_profile.preferences.clone(),
            scores: PersonalityScores {
                introverted: db_profile.scores.introverted,
                extraverted: db_profile.scores.extraverted,
                observant: db_profile.scores.observant,
                intuitive: db_profile.scores.intuitive,
                thinking: db_profile.scores.thinking,
                feeling: db_profile.scores.feeling,
                judging: db_profile.scores.judging,
                prospecting: db_profile.scores.prospecting,
                assertive: db_profile.scores.assertive,
                turbulent: db_profile.scores.turbulent,
            },
        }
    }
}

// Helper function to convert a slice of DynamoDB UserProfiles
pub fn convert_profiles(db_profiles: &[DbUserProfile]) -> Vec<UserProfile> {
    db_profiles.iter().map(UserProfile::from).collect()
}
