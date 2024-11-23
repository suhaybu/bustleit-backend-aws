use axum::{extract::Path, Json};
use uuid::Uuid;

use crate::{db::ProfileDb, models::UserProfile};
use common::error::Result;

/// GET: /v1/user/profile/:id
///
/// Returns a single user profile by ID
///
/// Example:
///   - /v1/user/profile/123e4567-e89b-12d3-a456-426614174000
pub async fn get_profile(Path(user_id): Path<Uuid>) -> Result<Json<UserProfile>> {
    let db = ProfileDb::new().await?;

    let profile_db = db.get_profile(&user_id).await?;
    let profile = UserProfile::from(&profile_db);

    Ok(Json(profile))
}
