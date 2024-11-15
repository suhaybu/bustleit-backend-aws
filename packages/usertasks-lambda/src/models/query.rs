use serde::Deserialize;

#[derive(Deserialize)]
pub struct DateRangeQuery {
    pub date: Option<String>,  // Single/Start date: YYYY-MM-DD
    pub until: Option<String>, // End date: YYYY-MM-DD
    pub range: Option<i32>,    // No. of days from date
}
