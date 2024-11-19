use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfileDB {
    #[serde(rename = "PK")]
    pub pk: String, // USER#uuid
    #[serde(rename = "SK")]
    pub sk: String, // PROFILE
    #[serde(rename = "GSI1PK")]
    pub gsi1pk: String, // CLUSTER#number
    #[serde(rename = "GSI1SK")]
    pub gsi1sk: String, // USER#uuid
    pub cluster: i32,
    pub preferences: Vec<String>,
    pub scores: Scores,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Scores {
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

impl UserProfileDB {
    pub fn new(uuid: &str, cluster: i32, preferences: Vec<String>, scores: Scores) -> Self {
        Self {
            pk: format!("USER#{}", uuid),
            sk: "PROFILE".to_string(),
            gsi1pk: format!("CLUSTER#{}", cluster),
            gsi1sk: format!("USER#{}", uuid),
            cluster,
            preferences,
            scores,
        }
    }
}
