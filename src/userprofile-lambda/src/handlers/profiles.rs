use axum::{extract::Query, Json};

use crate::db::ProfileDb;
use crate::models::{convert_profiles, UserProfile, UserProfilesBatchRequest, UserProfilesQuery};
use common::error::{Error, Result};

/// POST: /v1/user/profiles/batch
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
) -> Result<Json<Vec<UserProfile>>> {
    if payload.user_ids.is_empty() {
        return Err(Error::validation("At least one user ID must be provided"));
    }

    let db = ProfileDb::new().await?;
    let profiles_db = db.get_profiles(&payload.user_ids).await?;
    let respoonse = convert_profiles(profiles_db);

    Ok(Json(respoonse))
}

/// GET: /v1/user/profiles[?cluster={cluster_id}]
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
///   - /v1/user/profiles            ->  Returns all user profiles
///   - /v1/user/profiles?cluster=3  ->  Returns profiles in cluster 3
pub async fn get_profiles(
    Query(query): Query<UserProfilesQuery>,
) -> Result<Json<Vec<UserProfile>>> {
    let db = ProfileDb::new().await?;

    let profiles_db = match query.cluster {
        Some(cluster) => {
            if cluster < 0 {
                return Err(Error::validation(
                    "Cluster ID must be a non-negative integer",
                ));
            }
            db.get_profiles_by_cluster(cluster).await
        }
        None => db.get_all_users().await,
    }?;

    let profiles = convert_profiles(profiles_db);

    Ok(Json(profiles))
}
