use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Uuid};

// This is for internal use to represent how it's stored in DB
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub user_id: Uuid,
    pub schedule_date: NaiveDate,
    pub name: String,
    pub category: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// This is used for responding to API calls
#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub task_id: Uuid,
    pub name: String,
    pub category: String,
    pub start_time: String, // HH:MM format
    pub end_time: String,   // HH:MM format
    pub completed: bool,
    pub created_at: String, // ISO 8601
    pub updated_at: String, // ISO 8601
}

impl From<Task> for TaskResponse {
    // Helper function to make Tasks API Response ready
    fn from(task: Task) -> Self {
        Self {
            task_id: task.id,
            name: task.name,
            category: task.category,
            // Format times as HH:MM
            start_time: task.start_time.format("%H:%M").to_string(),
            end_time: task.end_time.format("%H:%M").to_string(),
            completed: task.completed,
            // Format timestamps as ISO 8601
            created_at: task.created_at.to_rfc3339(),
            updated_at: task.updated_at.to_rfc3339(),
        }
    }
}
