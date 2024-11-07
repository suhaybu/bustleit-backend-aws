use std::fmt;

#[derive(Debug)]
pub enum DynamoDbError {
    NotFound(String),
    ParseError(String),
    ConnectionError(String),
    Other(String),
}

impl fmt::Display for DynamoDbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DynamoDbError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DynamoDbError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DynamoDbError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            DynamoDbError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for DynamoDbError {}
