use axum::{extract::Query, http::StatusCode, Json};

use crate::models::{convert_profiles, UserProfile, UserProfilesBatchRequest, UserProfilesQuery};
use common::{dynamodb::DynamoDbClient, utils::error::DynamoDbError};

/// POST: /user/profiles/batch
/// Retrieves multiple user profiles in a single request
///
/// Request Body:
/// ```json
/// {
///     "user_ids": ["123e4567-e89b-12d3", "987fcdeb-51d3-a456"]
/// }
/// ```
///
/// Returns:
/// - 200: Array of user profiles
/// - 400: If request body is invalid
/// - 500: For server errors
pub async fn get_batch(
    Json(payload): Json<UserProfilesBatchRequest>,
) -> Result<Json<Vec<UserProfile>>, (StatusCode, Json<serde_json::Value>)> {
    if payload.user_ids.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "At least one user ID must be provided"
            })),
        ));
    }

    let db = DynamoDbClient::new().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            })),
        )
    })?;

    let profiles_db = db
        .get_user_profiles(payload.user_ids)
        .await
        .map_err(|e| match e {
            DynamoDbError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": msg
                })),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": format!("Failed to fetch profiles: {}", e)
                })),
            ),
        })?;

    let respoonse = convert_profiles(profiles_db);

    Ok(Json(respoonse))
}

/// GET: /user/profiles[?cluster={cluster_id}]
///
/// Returns all user profiles or filters by cluster if cluster query parameter is provided
///
/// Query Parameters:
///   - cluster (optional): Integer value representing the cluster ID
///
/// Returns:
///   - 200: Array of user profiles
///   - 400: If cluster parameter is invalid
///   - 404: If no profiles found
///   - 500: For server errors
///
/// Examples:
///   - /user/profiles            ->  Returns all user profiles
///   - /user/profiles?cluster=3  ->  Returns profiles in cluster 3
pub async fn get_profiles(
    Query(query): Query<UserProfilesQuery>,
) -> Result<Json<Vec<UserProfile>>, (StatusCode, Json<serde_json::Value>)> {
    let db = DynamoDbClient::new().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Database connection error: {}", e)
            })),
        )
    })?;

    let profiles_db = match query.cluster {
        Some(cluster) => {
            if cluster < 0 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "Cluster ID must be a non-negative integer"
                    })),
                ));
            }
            db.get_users_by_cluster(cluster).await
        }
        None => db.get_all_users().await,
    }
    .map_err(|e| match e {
        DynamoDbError::NotFound(msg) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": msg
            })),
        ),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to fetch profiles: {}", e)
            })),
        ),
    })?;

    let profiles = convert_profiles(profiles_db);

    Ok(Json(profiles))
}
