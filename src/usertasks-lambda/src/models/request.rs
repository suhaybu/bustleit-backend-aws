use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct TasksRequest {
    pub user_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub name: String,
    pub category: String,
    pub start_time: String,
    pub end_time: String,
    pub date: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub name: Option<String>,
    pub category: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub completed: Option<bool>,
    pub date: Option<String>,
}
