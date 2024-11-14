use axum::{extract::Query, http::StatusCode, Json};

use crate::models::{convert_profiles, UserProfile, UserProfilesBatchRequest, UserProfilesQuery};
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

/// GET: /user/profiles[?cluster={cluster_id}]
///
/// Returns all user profiles or filters by cluster if cluster query parameter is provided
///
/// Examples:
///   - /user/profiles            ->  Returns all user profiles
///   - /user/profiles?cluster=3  ->  Returns profiles in cluster 3
pub async fn get_profiles(
    Query(query): Query<UserProfilesQuery>,
) -> Result<Json<Vec<UserProfile>>, StatusCode> {
    let db = DynamoDbClient::new()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profiles_db = match query.cluster {
        Some(cluster) => db.get_users_by_cluster(cluster).await,
        None => db.get_all_users().await,
    }
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let profiles = convert_profiles(profiles_db);

    Ok(Json(profiles))
}
