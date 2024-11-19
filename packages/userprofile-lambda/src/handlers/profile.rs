use axum::{extract::Path, http::StatusCode, Json};
use common::{dynamodb::DynamoDbClient, error::DynamoDbError, models::userdata::UserProfile};

/// GET: /v1/user/profile/:id
///
/// Returns a single user profile by ID
///
/// Example:
///   - /v1/user/profile/123e4567-e89b-12d3-a456-426614174000
pub async fn get_profile(Path(user_id): Path<String>) -> Result<Json<UserProfile>, StatusCode> {
    let db = DynamoDbClient::new()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profile_db = db.get_user_profile(user_id).await.map_err(|e| match e {
        DynamoDbError::NotFound(_) => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    let profile = UserProfile::from(&profile_db);

    Ok(Json(profile))
}
