use axum::{http::StatusCode, Json};
use chrono::NaiveDate;
use serde::Deserialize;
use validator::{Validate, ValidationError};

pub const DATE_FMT: &str = "%Y-%m-%d"; // YYYY-MM-DD
const ERR_INVALID_DATE_FORMAT: &str = "Invalid date format. Expected YYYY-MM-DD";
const ERR_INVALID_DATE_RANGE: &str = "End date must be after start date";
const ERR_CONFLICTING_PARAMS: &str = "Cannot specify both 'until' and 'range'";

pub type ApiError = (StatusCode, Json<serde_json::Value>);

#[derive(Deserialize, Validate)]
pub struct DateRangeQuery {
    #[validate(custom(function = "validate_date_format", message = "Invalid date format"))]
    pub date: Option<String>, // Single/Start date: YYYY-MM-DD

    #[validate(custom(function = "validate_date_format", message = "Invalid date format"))]
    pub until: Option<String>, // End date: YYYY-MM-DD

    #[validate(range(min = 1, max = 31, message = "Range must be between 1 and 31 days"))]
    pub range: Option<i32>, // No. of days from date

    #[serde(default)]
    pub skip_empty: bool,
}

/// Checks if date compiles with YYYY-MM-DD format
fn validate_date_format(date: &str) -> Result<(), ValidationError> {
    NaiveDate::parse_from_str(date, DATE_FMT)
        .map_err(|_| ValidationError::new(ERR_INVALID_DATE_FORMAT))?;
    Ok(())
}

impl DateRangeQuery {
    pub fn validate_all(&self) -> Result<(), ApiError> {
        if let Err(validation_errors) = self.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "Validation failed",
                    "details": validation_errors.to_string()
                })),
            ));
        }

        self.validate_date_range()?;

        Ok(())
    }

    /// Ensures the date ranges are valid
    fn validate_date_range(&self) -> Result<(), ApiError> {
        // Prevents defining both 'until' & 'range' in the same request
        if self.until.is_some() && self.range.is_some() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": ERR_CONFLICTING_PARAMS
                })),
            ));
        }

        // Ensures until date is later than start date
        if let (Some(date), Some(until)) = (&self.date, &self.until) {
            let start_date = NaiveDate::parse_from_str(date, DATE_FMT).map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": ERR_INVALID_DATE_FORMAT
                    })),
                )
            })?;

            let end_date = NaiveDate::parse_from_str(until, DATE_FMT).map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": ERR_INVALID_DATE_FORMAT
                    })),
                )
            })?;

            if end_date < start_date {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": ERR_INVALID_DATE_RANGE
                    })),
                ));
            }
        }

        Ok(())
    }
}
