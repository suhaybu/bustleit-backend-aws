use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RecommendationInput {
    pub scores: Vec<f32>,
    pub preferences: Vec<String>,
    pub work_end_time: i32,
}

#[derive(Debug, Serialize, Clone)]
pub struct TaskRecommendation {
    pub task_name: String,
    pub score: f32,
    pub category: String,
    pub suggested_time: String,
}

#[derive(Debug, Serialize)]
pub struct RecommendationResponse {
    pub recommendations: Vec<TaskRecommendation>,
    pub total_tasks: usize,
}
