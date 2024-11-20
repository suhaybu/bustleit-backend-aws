use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use std::env;

use crate::error::{Error, Result};

pub struct DynamoDbClient {
    pub client: Client,
    pub table_name: String,
}

#[allow(dead_code)]
impl DynamoDbClient {
    pub async fn new() -> Result<Self> {
        // Loads env
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);

        let table_name = env::var("DYNAMODB_TABLE").map_err(|e| {
            Error::db_connection_failed(format!("Missing DYNAMODB_TABLE env var: {}", e))
        })?;

        Ok(Self { client, table_name })
    }

    // Creating Keys
    // pk = Primary Key, sk = Secondary KeyZ
    pub fn create_user_pk(user_id: &str) -> String {
        format!("USER#{}", user_id)
    }

    pub fn create_task_pk(date: &str, task_id: &str) -> String {
        format!("TASK#DATE#{}#{}", date, task_id)
    }

    pub fn create_stats_pk(user_id: &str) -> String {
        format!("STATS#USER#{}", user_id)
    }

    pub fn create_profile_sk() -> String {
        "PROFILE".to_string()
    }

    pub fn create_task_sk(date: &str) -> String {
        format!("TASK#DATE#{}", date)
    }
}
