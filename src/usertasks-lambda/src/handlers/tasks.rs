use axum::{extract::Path, Json};
use chrono::{NaiveDate, NaiveTime};
use uuid::Uuid;

use crate::db::TasksDb;
use crate::models::{
    query::DATE_FMT,
    request::{CreateTaskRequest, TasksRequest},
    response::{Task, TasksResponse},
};
use common::error::{Error, Result};

// GET /v1/tasks - Get all tasks
pub async fn get_all_tasks() -> Result<Json<Vec<TasksResponse>>> {
    let db = TasksDb::new().await?;
    let tasks = db.get_all_tasks().await?;

    let mut user_tasks_map = std::collections::HashMap::new();
    for task in tasks {
        user_tasks_map
            .entry(task.user_id)
            .or_insert_with(Vec::new)
            .push(Task::from(task));
    }

    let response = user_tasks_map
        .into_iter()
        .map(|(user_id, all_tasks)| TasksResponse { user_id, all_tasks })
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

    let db = TasksDb::new().await?;
    let tasks = db.get_users_tasks(&payload.user_ids).await?;

    let mut user_tasks_map = std::collections::HashMap::new();
    for task in tasks {
        user_tasks_map
            .entry(task.user_id)
            .or_insert_with(Vec::new)
            .push(Task::from(task));
    }

    let response = user_tasks_map
        .into_iter()
        .map(|(user_id, all_tasks)| TasksResponse { user_id, all_tasks })
        .collect();

    Ok(Json(response))
}

// DELETE /v1/user/:user_id/tasks/:task_id
pub async fn delete_task(Path((user_id, task_id)): Path<(Uuid, Uuid)>) -> Result<()> {
    let db = TasksDb::new().await?;
    db.delete_task(user_id, task_id).await?;
    Ok(())
}

// POST /v1/user/:user_id/tasks - Create task
pub async fn create_task(
    Path(user_id): Path<Uuid>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<Task>> {
    // Parse date
    let date = NaiveDate::parse_from_str(&payload.date, DATE_FMT)
        .map_err(|_| Error::validation("Invalid date format"))?;

    if !payload.start_time.contains(':') || !payload.end_time.contains(':') {
        return Err(Error::validation("Time must be in HH:MM format"));
    }

    let start_time = NaiveTime::parse_from_str(&payload.start_time, "%H:%M")
        .map_err(|_| Error::validation("Start time must be in HH:MM format"))?;
    let end_time = NaiveTime::parse_from_str(&payload.end_time, "%H:%M")
        .map_err(|_| Error::validation("End time must be in HH:MM format"))?;

    // Validate times
    if end_time <= start_time {
        return Err(Error::validation("End time must be after start time"));
    }

    let db = TasksDb::new().await?;
    let task = db
        .add_task(
            user_id,
            date,
            payload.name,
            payload.category,
            &payload.start_time,
            &payload.end_time,
        )
        .await?;

    Ok(Json(Task::from(task)))
}
