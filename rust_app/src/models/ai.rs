use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserScores {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub id: i32,
    pub scores: UserScores,
    pub preferences: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UserProfileRequest {
    pub ids: Vec<i32>,
    pub req_scores: bool,
    pub req_preferences: bool,
}

#[derive(Debug, Serialize)]
pub struct RawProfile {
    pub id: i32,
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
    pub preferences: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ClusteredUsers {
    pub id: i32,
    pub cluster: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Task {
    pub name: String,
    pub category: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserTask {
    pub id: i32,
    pub tasks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailySchedule {
    pub task_name: String,
    pub start_time: String,
    pub duration: i32,
}

#[derive(Debug, Deserialize)]
pub struct SchedulesRequest {
    pub weeks: i32,
}
