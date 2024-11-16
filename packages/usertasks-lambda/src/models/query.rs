use chrono::NaiveDate;
use serde::Deserialize;
use validator::{Validate, ValidationError};

pub const DATE_FMT: &str = "%Y-%m-%d"; // YYYY-MM-DD
const ERR_INVALID_DATE_FORMAT: &str = "Invalid date format. Expected YYYY-MM-DD";
const ERR_INVALID_DATE_RANGE: &str = "End date must be after start date";
const ERR_CONFLICTING_PARAMS: &str = "Cannot specify both 'until' and 'range'";

#[derive(Deserialize, Validate)]
pub struct DateRangeQuery {
    #[validate(custom(function = "validate_date_format", message = "Invalid date format"))]
    pub date: Option<String>, // Single/Start date: YYYY-MM-DD

    #[validate(custom(function = "validate_date_format", message = "Invalid date format"))]
    pub until: Option<String>, // End date: YYYY-MM-DD

    #[validate(range(min = 1, max = 31, message = "Range must be between 1 and 31 days"))]
    pub range: Option<i32>, // No. of days from date
}

/// Checks if date compiles with YYYY-MM-DD format
fn validate_date_format(date: &str) -> Result<(), ValidationError> {
    NaiveDate::parse_from_str(date, DATE_FMT)
        .map_err(|_| ValidationError::new(ERR_INVALID_DATE_FORMAT))?;
    Ok(())
}

impl DateRangeQuery {
    /// Validates formats, and ensures date ranges are logically validated
    pub fn validate_all(&self) -> Result<(), ValidationError> {
        self.validate().unwrap_err();
        self.validate_date_range().unwrap_err();

        Ok(())
    }

    /// Ensures the date ranges are valid
    fn validate_date_range(&self) -> Result<(), ValidationError> {
        // Prevents defining both 'until' & 'range' in the same request
        if self.until.is_some() && self.range.is_some() {
            return Err(ValidationError::new(ERR_CONFLICTING_PARAMS));
        }

        // Ensures until date is later than start date
        if let (Some(date), Some(until)) = (&self.date, &self.until) {
            let start_date = NaiveDate::parse_from_str(date, DATE_FMT).unwrap();
            let end_date = NaiveDate::parse_from_str(until, DATE_FMT).unwrap();

            if end_date < start_date {
                return Err(ValidationError::new(ERR_INVALID_DATE_RANGE));
            }
        }

        Ok(())
    }

    // pub fn parse_date(&self) -> Option<NaiveDate> {
    //     self.date
    //         .as_ref()
    //         .and_then(|d| NaiveDate::parse_from_str(d, DATE_FMT).ok())
    // }
}
