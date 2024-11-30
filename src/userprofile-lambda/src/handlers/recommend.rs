use axum::{extract::Path, Json};
use chrono::NaiveTime;
use uuid::Uuid;

use crate::{
    db::ProfileDb,
    models::{RequestRecommend, ResponseRecommendDaily, ResponseRecommendWeekly},
};
use common::error::{Error, Result};

// GET: /v1/recommend/:user_id
pub async fn get_recommendation(Path(user_id): Path<Uuid>) -> Result<Json<ResponseRecommendDaily>> {
    let url = get_external_endpoint("/recommend_daily")?;

    let db = ProfileDb::new().await?;
    let profile_data = db.get_profile(user_id).await?;
    let times = UserTimes::default(); // TEMP: Hardcoded missing User Data for now

    let request_body = RequestRecommend::new(
        user_id,
        profile_data.get_typed_scores().unwrap_or_default(),
        profile_data.preferences,
        profile_data.cluster,
        times.work_start,
        times.work_end,
        times.sleep,
    );

    let response = make_api_request::<_, ResponseRecommendDaily>(url, &request_body).await?;

    Ok(Json(response))
}

// GET /v1/recommend/:user_id/week
pub async fn get_recommendation_week(
    Path(user_id): Path<Uuid>,
) -> Result<Json<ResponseRecommendWeekly>> {
    let url = get_external_endpoint("/recommend_weekly")?;

    let db = ProfileDb::new().await?;
    let profile_data = db.get_profile(user_id).await?;
    let times = UserTimes::default(); // TEMP: Hardcoded missing User Data for now

    let request_body = RequestRecommend::new(
        user_id,
        profile_data.get_typed_scores().unwrap_or_default(),
        profile_data.preferences,
        profile_data.cluster,
        times.work_start,
        times.work_end,
        times.sleep,
    );

    let response = make_api_request::<_, ResponseRecommendWeekly>(url, &request_body).await?;

    Ok(Json(response))
}

// This function gets the url of the external api from env and arg
fn get_external_endpoint(endpoint: &str) -> Result<String> {
    let uri_base =
        std::env::var("EXTERNAL_API").map_err(|_| Error::validation("EXTERNAL_API must be set"))?;

    Ok(format!("{uri_base}{endpoint}"))
}

// This function calls the external API
async fn make_api_request<T, R>(url: String, body: &T) -> Result<R>
where
    T: serde::Serialize,
    R: for<'de> serde::Deserialize<'de>,
{
    let recommendation = reqwest::Client::new()
        .post(url)
        .json(body)
        .send()
        .await
        .map_err(|e| Error::validation(format!("API request failed: {}", e)))?;

    if !recommendation.status().is_success() {
        return Err(Error::InternalServerError(format!(
            "External API error: Status {}",
            recommendation.status()
        )));
    }

    recommendation
        .json::<R>()
        .await
        .map_err(|_| Error::InternalServerError("External API error".to_string()))
}

// TEMP: Until we have real user data
struct UserTimes {
    work_start: NaiveTime,
    work_end: NaiveTime,
    sleep: NaiveTime,
}

impl UserTimes {
    // Convert time to numeric format (HHMM)
    // fn to_numeric(&self, time: NaiveTime) -> i32 {
    //     (time.hour() as i32 * 100) + time.minute() as i32
    // }

    fn default() -> Self {
        Self {
            work_start: NaiveTime::parse_from_str("09:00", "%H:%M").unwrap(),
            work_end: NaiveTime::parse_from_str("16:30", "%H:%M").unwrap(),
            sleep: NaiveTime::parse_from_str("22:00", "%H:%M").unwrap(),
        }
    }
}
