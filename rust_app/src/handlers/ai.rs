use axum::{http::StatusCode, Json};
use tracing::info;

use crate::models::ai::*;
use crate::services::mock_db::MockDb;

// POST /ai/request/userdata/
pub async fn get_userdata(
    Json(payload): Json<UserProfileRequest>,
) -> Result<Json<Vec<UserProfile>>, (StatusCode, Json<serde_json::Value>)> {
    info!("Fetching user data for ids: {:?}", payload.ids);

    let users = MockDb::get_userdata(payload).await;
    Ok(Json(users))
}

// GET /ai/request/allusers/
pub async fn get_all_users() -> Result<Json<Vec<RawProfile>>, (StatusCode, Json<serde_json::Value>)>
{
    info!("Fetching all users raw profiles");

    let users = MockDb::get_all_userdata().await;
    Ok(Json(users))
}

// GET /ai/request/clusteredUsers
pub async fn get_clustered_users(
) -> Result<Json<Vec<ClusteredUsers>>, (StatusCode, Json<serde_json::Value>)> {
    info!("Fetching clustered users");

    let clusters = MockDb::get_clustered_users().await;
    Ok(Json(clusters))
}

// POST /ai/request/schedules
pub async fn get_schedules(
    Json(payload): Json<SchedulesRequest>,
) -> Result<Json<Vec<serde_json::Value>>, (StatusCode, Json<serde_json::Value>)> {
    info!("Fetching schedules for {} weeks", payload.weeks);

    let schedules = MockDb::get_weekly_schedules(payload.weeks).await;
    Ok(Json(schedules))
}

// GET /ai/request/tasks
pub async fn get_tasks() -> Result<Json<Vec<Task>>, (StatusCode, Json<serde_json::Value>)> {
    info!("Fetching all tasks");

    let tasks = MockDb::get_tasks().await;
    Ok(Json(tasks))
}

// GET /ai/request/taskedUsers
pub async fn get_tasked_users() -> Result<Json<Vec<UserTask>>, (StatusCode, Json<serde_json::Value>)>
{
    info!("Fetching users with tasks");

    let users = MockDb::get_users_with_tasks().await;
    Ok(Json(users))
}
