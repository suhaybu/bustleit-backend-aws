use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use common::models::database as DB;

#[derive(Serialize)]
pub struct TasksResponse {
    pub user_id: Uuid,
    pub all_tasks: Vec<Task>,
}

#[derive(Serialize)]
pub struct ScheduleResponse {
    pub user_id: String,
    pub data: HashMap<String, DayTasks>,
}

#[derive(Serialize)]
pub struct DayTasks {
    pub total_tasks: i32,
    pub completed_tasks: i32,
    pub tasks: Vec<Task>,
}

#[derive(Serialize)]
pub struct Task {
    pub task_id: String,
    pub name: String,
    pub category: String,

    pub start_time: String,
    pub end_time: String,

    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
}

// Converts DB::Task format to Response format (move)
impl From<DB::Task> for Task {
    fn from(db_task: DB::Task) -> Self {
        Self {
            task_id: db_task.id.to_string(),
            name: db_task.name,
            category: db_task.category,
            start_time: db_task.start_time.to_string(),
            end_time: db_task.end_time.to_string(),
            completed: db_task.completed,
            created_at: db_task.created_at.to_string(),
            updated_at: db_task.updated_at.to_string(),
        }
    }
}

impl ScheduleResponse {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            data: HashMap::new(),
        }
    }
}
