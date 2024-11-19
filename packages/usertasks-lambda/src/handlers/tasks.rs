use axum::Json;

use crate::models::{Task, TasksRequest, TasksResponse};
use common::{
    dynamodb::DynamoDbClient,
    error::{Error, Result},
};

// GET /v1/tasks - Get all tasks
pub async fn get_all_tasks() -> Result<Json<Vec<TasksResponse>>> {
    let db = DynamoDbClient::new().await?;
    let all_tasks = db.get_all_users_tasks().await?;

    let response = all_tasks
        .into_iter()
        .map(|tasks| {
            let user_id = tasks
                .pk
                .strip_prefix("USER#")
                .unwrap_or(&tasks.pk) // Fallback: if no USER# found, return as is
                .to_string();

            TasksResponse {
                user_id,
                all_tasks: tasks.tasks.into_iter().map(Task::from).collect(),
            }
        })
        .collect();

    Ok(Json(response))
}

// POST /v1/tasks/batch - Get tasks for specific users
pub async fn get_tasks_batch(
    Json(payload): Json<TasksRequest>,
) -> Result<Json<Vec<TasksResponse>>> {
    if payload.user_ids.is_empty() {
        return Err(Error::validation("At least one user ID must be provided"));
    }

    let db = DynamoDbClient::new().await?;
    let user_tasks = db.get_users_tasks(&payload.user_ids).await?;

    let response = user_tasks
        .into_iter()
        .map(|tasks| {
            let user_id = tasks
                .pk
                .strip_prefix("USER#")
                .unwrap_or(&tasks.pk)
                .to_string();

            TasksResponse {
                user_id,
                all_tasks: tasks.tasks.into_iter().map(Task::from).collect(),
            }
        })
        .collect();

    Ok(Json(response))
}
