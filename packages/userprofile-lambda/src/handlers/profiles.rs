use axum::{http::StatusCode, Json};

use crate::models::{convert_profiles, UserProfile, UserProfilesBatchRequest};
use common::dynamodb::DynamoDbClient;

// POST: /user/profiles/batch
pub async fn get_batch(
    Json(payload): Json<UserProfilesBatchRequest>,
) -> Result<Json<Vec<UserProfile>>, StatusCode> {
    let db = DynamoDbClient::new()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profiles_db = db
        .get_user_profiles(payload.user_ids)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let respoonse = convert_profiles(profiles_db);

    Ok(Json(respoonse))
}
