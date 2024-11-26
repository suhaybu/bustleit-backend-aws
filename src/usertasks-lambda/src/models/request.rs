use chrono::NaiveTime;
use serde::Deserialize;
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::models::query::DATE_FMT;
use common::error::{Error, Result};

#[derive(Deserialize)]
pub struct TasksRequest {
    pub user_ids: Vec<Uuid>,
}

#[derive(Deserialize, Validate)]
pub struct CreateTaskRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Task name must be between 1 and 255 characters"
    ))]
    pub name: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Category must be between 1 and 100 characters"
    ))]
    pub category: String,

    #[validate(custom(function = "validate_time_format"))]
    pub start_time: String,

    #[validate(custom(function = "validate_time_format"))]
    pub end_time: String,

    #[validate(custom(function = "validate_date_format"))]
    pub date: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateTaskRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Task name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Category must be between 1 and 100 characters"
    ))]
    pub category: Option<String>,

    #[validate(custom(function = "validate_time_format"))]
    pub start_time: Option<String>,

    #[validate(custom(function = "validate_time_format"))]
    pub end_time: Option<String>,

    pub completed: Option<bool>,

    #[validate(custom(function = "validate_date_format"))]
    pub date: Option<String>,
}

// Validates HH:MM time format
fn validate_time_format(time: &str) -> std::result::Result<(), ValidationError> {
    if !time.contains(':') {
        return Err(ValidationError::new("Time must be in HH:MM format"));
    }

    NaiveTime::parse_from_str(time, "%H:%M")
        .map_err(|_| ValidationError::new("Invalid time format"))?;

    Ok(())
}

// Validates YYYY-MM-DD date format
fn validate_date_format(date: &str) -> std::result::Result<(), ValidationError> {
    chrono::NaiveDate::parse_from_str(date, DATE_FMT)
        .map_err(|_| ValidationError::new("Invalid date format. Expected YYYY-MM-DD"))?;
    Ok(())
}

impl CreateTaskRequest {
    /// Validates the entire request including inter-field validations
    pub fn validate_all(&self) -> Result<()> {
        // Run validator derive validations
        if let Err(validation_errors) = self.validate() {
            return Err(Error::validation(validation_errors.to_string()));
        }

        // Parse and validate times
        let start_time = NaiveTime::parse_from_str(&self.start_time, "%H:%M")
            .expect("Time format already validated");
        let end_time = NaiveTime::parse_from_str(&self.end_time, "%H:%M")
            .expect("Time format already validated");

        // Check time order
        if end_time <= start_time {
            return Err(Error::validation("End time must be after start time"));
        }

        Ok(())
    }
}

impl UpdateTaskRequest {
    /// Validates the entire request including inter-field validations
    pub fn validate_all(&self) -> Result<()> {
        // Run validator derive validations
        if let Err(validation_errors) = self.validate() {
            return Err(Error::validation(validation_errors.to_string()));
        }

        // Check time order if both times are provided
        if let (Some(start), Some(end)) = (&self.start_time, &self.end_time) {
            let start_time =
                NaiveTime::parse_from_str(start, "%H:%M").expect("Time format already validated");
            let end_time =
                NaiveTime::parse_from_str(end, "%H:%M").expect("Time format already validated");

            if end_time <= start_time {
                return Err(Error::validation("End time must be after start time"));
            }
        }

        Ok(())
    }

    /// Returns true if the request contains no updates
    pub fn is_empty(&self) -> bool {
        self.name.is_none()
            && self.category.is_none()
            && self.start_time.is_none()
            && self.end_time.is_none()
            && self.completed.is_none()
            && self.date.is_none()
    }
}
