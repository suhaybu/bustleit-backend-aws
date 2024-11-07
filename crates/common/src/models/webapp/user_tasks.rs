use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::dynamodb::user_tasks::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontendScheduleResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub data: HashMap<String, Vec<FrontendTask>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontendTask {
    #[serde(rename = "taskId")]
    pub task_id: String,
    pub name: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    pub completed: bool,
}

impl From<&Task> for FrontendTask {
    fn from(task: &Task) -> Self {
        Self {
            task_id: task.task_id.clone(),
            name: task.name.clone(),
            start_time: task.start_time.clone(),
            end_time: task.end_time.clone(),
            completed: task.completed,
        }
    }
}

impl FrontendScheduleResponse {
    pub fn from_user_tasks(user_tasks: &crate::models::dynamodb::user_tasks::UserTasks) -> Self {
        let mut data = HashMap::new();
        let tasks: Vec<FrontendTask> = user_tasks.tasks.iter().map(FrontendTask::from).collect();

        data.insert(user_tasks.date.clone(), tasks);

        Self {
            user_id: user_tasks.pk.trim_start_matches("USER#").to_string(),
            data,
        }
    }

    pub fn merge(&mut self, other: &crate::models::dynamodb::user_tasks::UserTasks) {
        let tasks: Vec<FrontendTask> = other.tasks.iter().map(FrontendTask::from).collect();
        self.data.insert(other.date.clone(), tasks);
    }
}

// Helper function to convert multiple UserTasks into a single FrontendScheduleResponse
pub fn combine_user_tasks(
    tasks: Vec<crate::models::dynamodb::user_tasks::UserTasks>,
) -> FrontendScheduleResponse {
    if let Some(first) = tasks.first() {
        let mut response = FrontendScheduleResponse::from_user_tasks(first);

        // Merge the rest of the tasks
        for task in tasks.iter().skip(1) {
            response.merge(task);
        }

        response
    } else {
        // Return empty response if no tasks
        FrontendScheduleResponse {
            user_id: String::new(),
            data: HashMap::new(),
        }
    }
}

// Example usage in handler:
/*
pub async fn get_user_schedule(
    uuid: &str,
    dates: Vec<String>,
) -> Result<FrontendScheduleResponse, DynamoDbError> {
    let db = DynamoDbClient::new().await?;

    let mut all_tasks = Vec::new();
    for date in dates {
        let tasks = db.get_user_tasks(uuid, &date).await?;
        all_tasks.push(tasks);
    }

    Ok(combine_user_tasks(all_tasks))
}
*/
