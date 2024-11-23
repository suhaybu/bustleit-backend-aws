use serde::{Deserialize, Serialize};
use uuid::Uuid;

use common::models::database as DB;

// POST Request structure for /user/profiles/batch
#[derive(Deserialize)]
pub struct UserProfilesBatchRequest {
    pub user_ids: Vec<Uuid>,
}

// GET Query structure for /user/profiles
#[derive(Deserialize)]
pub struct UserProfilesQuery {
    pub cluster: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: Uuid,
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
impl From<&DB::Profile> for UserProfile {
    fn from(db_profile: &DB::Profile) -> Self {
        let scores = db_profile.get_typed_scores().unwrap_or_default();

        Self {
            user_id: db_profile.user_id,
            preferences: db_profile.preferences.clone(),
            scores: PersonalityScores {
                introverted: scores.introverted,
                extraverted: scores.extraverted,
                observant: scores.observant,
                intuitive: scores.intuitive,
                thinking: scores.thinking,
                feeling: scores.feeling,
                judging: scores.judging,
                prospecting: scores.prospecting,
                assertive: scores.assertive,
                turbulent: scores.turbulent,
            },
        }
    }
}

// Helper function to convert a slice of DB UserProfiles
pub fn convert_profiles(profiles_db: Vec<DB::Profile>) -> Vec<UserProfile> {
    profiles_db.iter().map(UserProfile::from).collect()
}
