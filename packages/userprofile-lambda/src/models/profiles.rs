use common::models::dynamodb::UserProfileDB;
use serde::{Deserialize, Serialize};

// POST Request structure for /user/profiles/batch
#[derive(Deserialize)]
pub struct UserProfilesBatchRequest {
    pub user_ids: Vec<String>,
}

// GET Query structure for /user/profiles
#[derive(Deserialize)]
pub struct UserProfilesQuery {
    pub cluster: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub scores: PersonalityScores,
    pub preferences: Vec<String>,
}

#[derive(Serialize, Deserialize)]
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

// Convert from DynamoDB UserProfileDB to UserProfile
impl From<&UserProfileDB> for UserProfile {
    fn from(db_profile: &UserProfileDB) -> Self {
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
pub fn convert_profiles(profiles_db: Vec<UserProfileDB>) -> Vec<UserProfile> {
    profiles_db.iter().map(UserProfile::from).collect()
}
