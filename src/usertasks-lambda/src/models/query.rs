use chrono::NaiveDate;
use serde::Deserialize;
use validator::{Validate, ValidationError};

use common::error::{Error, Result};

pub const DATE_FMT: &str = "%Y-%m-%d"; // YYYY-MM-DD

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
fn validate_date_format(date: &str) -> std::result::Result<(), ValidationError> {
    NaiveDate::parse_from_str(date, DATE_FMT)
        .map_err(|_| ValidationError::new("Invalid date format. Expected YYYY-MM-DD"))?;
    Ok(())
}

impl DateRangeQuery {
    pub fn validate_all(&self) -> Result<()> {
        if let Err(validation_errors) = self.validate() {
            return Err(Error::validation(validation_errors.to_string()));
        }
        self.validate_date_range()?;
        Ok(())
    }

    /// Ensures the date ranges are valid
    fn validate_date_range(&self) -> Result<()> {
        // Prevents defining both 'until' & 'range' in the same request
        if self.until.is_some() && self.range.is_some() {
            return Err(Error::validation("Cannot specify both 'until' and 'range'"));
        }

        // Ensures until date is later than start date
        if let (Some(date), Some(until)) = (&self.date, &self.until) {
            let start_date = NaiveDate::parse_from_str(date, DATE_FMT)
                .map_err(|_| Error::validation("Invalid start date format"))?;

            let end_date = NaiveDate::parse_from_str(until, DATE_FMT)
                .map_err(|_| Error::validation("Invalid end date format"))?;

            if end_date < start_date {
                return Err(Error::validation("End date must be after start date"));
            }
        }

        Ok(())
    }
}
