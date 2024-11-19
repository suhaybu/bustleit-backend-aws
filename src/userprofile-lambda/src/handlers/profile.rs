use axum::{extract::Path, Json};

use crate::models::UserProfile;
use common::dynamodb::DynamoDbClient;
use common::error::Result;

/// GET: /v1/user/profile/:id
///
/// Returns a single user profile by ID
///
/// Example:
///   - /v1/user/profile/123e4567-e89b-12d3-a456-426614174000
pub async fn get_profile(Path(user_id): Path<String>) -> Result<Json<UserProfile>> {
    let db = DynamoDbClient::new().await?;

    let profile_db = db.get_user_profile(user_id).await?;
    let profile = UserProfile::from(&profile_db);

    Ok(Json(profile))
}
