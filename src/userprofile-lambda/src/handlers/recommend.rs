use axum::{extract::Path, Json};
use sqlx::types::chrono::{self, Utc};
use uuid::Uuid;

use crate::{
    db::ProfileDb,
    models::{RequestRecommendDaily, ResponseRecommendDaily},
};
use common::error::{Error, Result};

// GET: /v1/recommend/:user_id
pub async fn get_recommendation(Path(user_id): Path<Uuid>) -> Result<Json<ResponseRecommendDaily>> {
    let url = {
        let uri_base = std::env::var("EXTERNAL_API")
            .map_err(|_| Error::validation("EXTERNAL_API must be set"))?;
        format!("{}/recommend_daily", uri_base)
    };

    let db = ProfileDb::new().await?;
    let profile_data = db.get_profile(user_id).await?;

    // TEMP: Hardcoded missing User Data for now
    let work_start_time = chrono::DateTime::parse_from_str("09:00 +0000", "%H:%M %z")
        .unwrap()
        .with_timezone(&Utc);
    let work_end_time = chrono::DateTime::parse_from_str("16:30 +0000", "%H:%M %z")
        .unwrap()
        .with_timezone(&Utc);

    let sleep_time = chrono::DateTime::parse_from_str("22:00 +0000", "%H:%M %z")
        .unwrap()
        .with_timezone(&Utc);

    let request_body = RequestRecommendDaily::new(
        user_id,
        profile_data.get_typed_scores().unwrap_or_default(),
        profile_data.cluster,
        work_start_time,
        work_end_time,
        sleep_time,
    );

    let recommendation = reqwest::Client::new()
        .post(url)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| Error::validation(format!("API request failed: {}", e)))?;

    let response = recommendation
        .json::<ResponseRecommendDaily>()
        .await
        .map_err(|_| Error::InternalServerError("External API error".to_string()))?;

    Ok(Json(response))
}
