use axum::{extract::Path, http::StatusCode, Json};
use tracing::info;

use common::{
    dynamodb::DynamoDbClient,
    models::{
        ai::ClusteredUsers,
        userdata::{self, Task, UserProfile, UserProfileRequest, UserTask},
    },
};

// GET /v1/users
pub async fn list_users() -> Result<Json<Vec<UserProfile>>, (StatusCode, Json<serde_json::Value>)> {
    info!("Fetching all users");

    let db = DynamoDbClient::new().await.map_err(handle_error)?;
    let db_profiles = db
        .get_user_profiles_ai(&UserProfileRequest {
            user_ids: vec![], // Empty vector to get all users
            req_scores: true,
            req_preferences: true,
        })
        .await
        .map_err(handle_error)?;

    // Convert DynamoDB profiles to userdata profiles using reference conversion
    let profiles = userdata::profile::convert_profiles(&db_profiles);
    Ok(Json(profiles))
}

// GET /v1/users/{id}/profile
pub async fn get_user_profile(
    Path(user_id): Path<String>,
) -> Result<Json<UserProfile>, (StatusCode, Json<serde_json::Value>)> {
    info!("Fetching profile for user {}", user_id);

    let db = DynamoDbClient::new().await.map_err(handle_error)?;
    let db_profile = db.get_user_profile(&user_id).await.map_err(handle_error)?;

    let profile = UserProfile::from(&db_profile);
    Ok(Json(profile))
}

// POST /v1/users/profiles/batch
pub async fn get_user_profiles_batch(
    Json(payload): Json<UserProfileRequest>,
) -> Result<Json<Vec<UserProfile>>, (StatusCode, Json<serde_json::Value>)> {
    info!(
        "Fetching profiles for multiple users: {:?}",
        payload.user_ids
    );

    let db = DynamoDbClient::new().await.map_err(handle_error)?;
    let db_profiles = db
        .get_user_profiles_ai(&payload)
        .await
        .map_err(handle_error)?;

    let profiles = userdata::profile::convert_profiles(&db_profiles);
    Ok(Json(profiles))
}

// GET /v1/users/clusters
pub async fn get_user_clusters(
) -> Result<Json<Vec<ClusteredUsers>>, (StatusCode, Json<serde_json::Value>)> {
    info!("Fetching user clusters");

    let db = DynamoDbClient::new().await.map_err(handle_error)?;
    // Note: You might need to implement this in DynamoDbClient
    // For now, returning example data
    let clusters = vec![
        ClusteredUsers {
            user_id: "1".to_string(),
            cluster: 1,
        },
        // Add more example data as needed
    ];

    Ok(Json(clusters))
}

// GET /v1/tasks TODO
pub async fn list_tasks() -> Result<Json<Vec<Task>>, (StatusCode, Json<serde_json::Value>)> {
    info!("Fetching all tasks");

    let db = DynamoDbClient::new().await.map_err(handle_error)?;
    // Note: You might need to implement this in DynamoDbClient
    // For now, returning example data

    let tasks = vec![
        Task::new(
            // TODO
            "Example Task".to_string(),
            "Example Category".to_string(),
            // "09:00".to_string(),
            // "10:00".to_string(),
            // "task-id-1".to_string(),
        ),
        // Add more example data as needed
    ];

    Ok(Json(tasks))
}

// GET /v1/users/{id}/tasks
pub async fn get_user_tasks(
    Path(user_id): Path<String>,
) -> Result<Json<UserTask>, (StatusCode, Json<serde_json::Value>)> {
    info!("Fetching tasks for user {}", user_id);

    let db = DynamoDbClient::new().await.map_err(handle_error)?;
    // Note: The current date will need to be passed to get_user_tasks
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let user_tasks = db
        .get_user_tasks(&user_id, &today)
        .await
        .map_err(handle_error)?;

    Ok(Json(UserTask {
        user_id: user_id.clone(),
        tasks: user_tasks.tasks.iter().map(|t| t.name.clone()).collect(),
    }))
}

fn handle_error<E: std::fmt::Display>(err: E) -> (StatusCode, Json<serde_json::Value>) {
    let status = if err.to_string().contains("not found") {
        StatusCode::NOT_FOUND
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    (
        status,
        Json(serde_json::json!({
            "error": err.to_string()
        })),
    )
}
