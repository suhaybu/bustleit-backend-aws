use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub user_id: i32,
    pub time_slots: Vec<TimeSlot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start_time: String,
    pub end_time: String,
    pub task: Task,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Task {
    pub name: String,
    pub category: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct UserTask {
    pub id: i32,
    pub tasks: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct DailySchedule {
    pub task_name: String,
    pub start_time: String,
    pub duration: i32,
}

#[derive(Debug, Deserialize)]
pub struct SchedulesRequest {
    pub weeks: i32,
}
