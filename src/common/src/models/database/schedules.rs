use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

// This is for internal use to represent how it's stored in DB
#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub user_id: Uuid,
    pub schedule_date: NaiveDate,
    pub completed_tasks: i32,
    pub total_tasks: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// This is used for responding to API calls
#[derive(Debug, Serialize)]
pub struct ScheduleResponse {
    pub date: String, // YYYY-MM-DD
    pub total_tasks: i32,
    pub completed_tasks: i32,
    pub tasks: Vec<super::tasks::TaskResponse>,
}

impl Schedule {
    // Helper function to make Schedule API Response ready
    pub fn to_response(self, tasks: Vec<super::tasks::Task>) -> ScheduleResponse {
        ScheduleResponse {
            date: self.schedule_date.format("%Y-%m-%d").to_string(),
            total_tasks: self.total_tasks,
            completed_tasks: self.completed_tasks,
            tasks: tasks.into_iter().map(|t| t.into()).collect(),
        }
    }
}
