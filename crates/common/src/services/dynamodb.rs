use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use std::collections::HashMap;
use std::env;

use crate::models::userdata::{PersonalityScores, UserProfile, UserProfileRequest};
use crate::utils::error::DynamoDbError;

pub struct DynamoDbClient {
    client: Client,
    table_name: String,
}

impl DynamoDbClient {
    pub async fn new() -> Result<Self, DynamoDbError> {
        // Loads env
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);

        let table_name = env::var("DYNAMODB_TABLE")
            .map_err(|e| DynamoDbError::Other(format!("Missing DYNAMODB_TABLE env var: {}", e)))?;

        Ok(Self { client, table_name })
    }

    // Creating Keys
    // pk = Primary Key, sk = Secondary KeyZ
    fn create_user_pk(user_id: &str) -> String {
        format!("USER#{}", user_id)
    }

    fn create_profile_sk() -> String {
        "PROFILE".to_string()
    }

    fn create_task_pk(date: &str, task_id: &str) -> String {
        format!("TASK#DATE#{}#{}", date, task_id)
    }

    fn create_stats_pk(user_id: &str) -> String {
        format!("STATS#USER#{}", user_id)
    }

    pub async fn get_user_profiles(
        &self,
        req: &UserProfileRequest,
    ) -> Result<Vec<UserProfile>, DynamoDbError> {
        let mut profiles = Vec::new();

        for id in &req.ids {
            let pk = Self::create_user_pk(id);
            let sk = Self::create_profile_sk();

            match self
                .client
                .get_item()
                .table_name(&self.table_name)
                .key("PK", AttributeValue::S(pk))
                .key("SK", AttributeValue::S(sk))
                .send()
                .await
            {
                Ok(output) => {
                    if let Some(item) = output.item {
                        match self.convert_to_user_profile(&item, id) {
                            Ok(profile) => profiles.push(profile),
                            Err(e) => {
                                tracing::error!("Failed to convert profile for user {}: {}", id, e);
                                continue;
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("DynamoDB error for user {}: {}", id, e);
                    return Err(DynamoDbError::ConnectionError(e.to_string()));
                }
            }
        }

        if profiles.is_empty() {
            return Err(DynamoDbError::NotFound(
                "No user profiles found".to_string(),
            ));
        }

        Ok(profiles)
    }

    fn convert_to_user_profile(
        &self,
        item: &HashMap<String, AttributeValue>,
        id: &str,
    ) -> Result<UserProfile, DynamoDbError> {
        let preferences = if let Some(AttributeValue::L(prefs)) = item.get("preferences") {
            prefs
                .iter()
                .filter_map(|av| {
                    if let AttributeValue::S(s) = av {
                        Some(s.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        };

        let scores = if let Some(AttributeValue::M(scores_map)) = item.get("scores") {
            PersonalityScores {
                turbulent: Self::extract_number(scores_map.get("turbulent"))?,
                introverted: Self::extract_number(scores_map.get("introverted"))?,
                prospecting: Self::extract_number(scores_map.get("prospecting"))?,
                judging: Self::extract_number(scores_map.get("judging"))?,
                assertive: Self::extract_number(scores_map.get("assertive"))?,
                feeling: Self::extract_number(scores_map.get("feeling"))?,
                extraverted: Self::extract_number(scores_map.get("extraverted"))?,
                observant: Self::extract_number(scores_map.get("observant"))?,
                thinking: Self::extract_number(scores_map.get("thinking"))?,
                intuitive: Self::extract_number(scores_map.get("intuitive"))?,
            }
        } else {
            return Err(DynamoDbError::NotFound(format!(
                "Missing scores map for user {}",
                id
            )));
        };

        Ok(UserProfile {
            id: id.to_string(),
            scores,
            preferences,
        })
    }

    fn extract_number(attr: Option<&AttributeValue>) -> Result<f32, DynamoDbError> {
        match attr {
            Some(AttributeValue::N(num_str)) => num_str
                .parse::<f32>()
                .map_err(|e| DynamoDbError::ParseError(format!("Failed to parse number: {}", e))),
            _ => Ok(0.0),
        }
    }
}
