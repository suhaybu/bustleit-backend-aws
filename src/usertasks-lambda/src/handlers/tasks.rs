use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::db::UserTasksDb;
use crate::models::{
    request::{CreateTaskRequest, TasksRequest},
    response::{Task, TasksResponse},
};
use common::error::{Error, Result};
use common::models::dynamodb as DB;

// GET /v1/tasks - Get all tasks
pub async fn get_all_tasks() -> Result<Json<Vec<TasksResponse>>> {
    let db = UserTasksDb::new().await?;
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

    let db = UserTasksDb::new().await?;
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

// DELETE /v1/user/:user_id/tasks/:task_id
pub async fn delete_task(Path((user_id, task_id)): Path<(String, String)>) -> Result<()> {
    let db = UserTasksDb::new().await?;
    db.delete_task(&user_id, &task_id).await?;
    Ok(())
}

// POST /v1/user/:user_id/tasks
pub async fn create_task(
    Path(user_id): Path<String>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<DB::Task>> {
    let db = UserTasksDb::new().await?;
    let task_id = Uuid::new_v4().to_string();

    let task = DB::Task::new(
        payload.name,
        payload.category,
        payload.start_time,
        payload.end_time,
        task_id,
    );

    db.add_task(&user_id, &payload.date, task.clone()).await?;

    Ok(Json(task))
}

// PATCH /v1/user/:user_id/tasks/:task_id
