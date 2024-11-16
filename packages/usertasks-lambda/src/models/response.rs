use std::collections::HashMap;

use common::models::dynamodb as DB;
use serde::Serialize;

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

// Converts DynamoDB Task format to Response format (move)
impl From<DB::Task> for Task {
    fn from(db_task: DB::Task) -> Self {
        Self {
            task_id: db_task.task_id,
            name: db_task.name,
            category: db_task.category,
            start_time: db_task.start_time,
            end_time: db_task.end_time,
            completed: db_task.completed,
            created_at: db_task.created_at,
            updated_at: db_task.updated_at,
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

    // Used for creating structure of every day
    pub fn add_day(&mut self, date: String, user_tasks: Option<DB::UserTasks>) {
        let day_tasks = match user_tasks {
            Some(ut) => DayTasks {
                total_tasks: ut.total_tasks,
                completed_tasks: ut.completed_tasks,
                tasks: ut.tasks.into_iter().map(Task::from).collect(),
            },
            None => DayTasks {
                total_tasks: 0,
                completed_tasks: 0,
                tasks: Vec::new(),
            },
        };
        self.data.insert(date, day_tasks);
    }
}
