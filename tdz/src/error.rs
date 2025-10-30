use thiserror::Error;
use crate::models::*;
use std::collections::HashMap;
use chrono::Utc;
pub type Result<T> = std::result::Result<T, TodoziError>;
#[derive(Error, Debug)]
pub enum TodoziError {
    #[error("Task not found: {id}")]
    TaskNotFound { id: String },
    #[error("Project not found: {name}")]
    ProjectNotFound { name: String },
    #[error("Feeling not found: {id}")]
    FeelingNotFound { id: String },
    #[error(
        "Invalid priority: {priority}. Must be one of: low, medium, high, critical, urgent"
    )]
    InvalidPriority { priority: String },
    #[error(
        "Invalid status: {status}. Must be one of: todo, in_progress, blocked, review, done, cancelled, deferred"
    )]
    InvalidStatus { status: String },
    #[error("Invalid assignee: {assignee}. Must be one of: ai, human, collaborative")]
    InvalidAssignee { assignee: String },
    #[error("Invalid progress: {progress}. Must be between 0 and 100")]
    InvalidProgress { progress: u8 },
    #[error("Validation error: {message}")]
    ValidationError { message: String },
    #[error("Storage error: {message}")]
    StorageError { message: String },
    #[error("Configuration error: {message}")]
    ConfigError { message: String },
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("UUID error: {0}")]
    UuidError(#[from] uuid::Error),
    #[error("Chrono error: {0}")]
    ChronoError(#[from] chrono::ParseError),
    #[error("Dialoguer error: {0}")]
    DialoguerError(#[from] dialoguer::Error),
    #[error("HLX error: {0}")]
    HlxError(#[from] helix::hel::error::HlxError),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Directory error: {message}")]
    DirError { message: String },
    #[error("Embedding error: {message}")]
    EmbeddingError { message: String },
    #[error("API error: {message}")]
    ApiError { message: String },
    #[error("Candle error: {0}")]
    CandleError(String),
}
impl TodoziError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
        }
    }
    pub fn storage(message: impl Into<String>) -> Self {
        Self::StorageError {
            message: message.into(),
        }
    }
    pub fn config(message: impl Into<String>) -> Self {
        Self::ConfigError {
            message: message.into(),
        }
    }
    pub fn api(message: impl Into<String>) -> Self {
        Self::ApiError {
            message: message.into(),
        }
    }
    
    pub fn io(message: impl Into<String>) -> Self {
        Self::StorageError {
            message: message.into(),
        }
    }
    
    pub fn serialization(message: impl Into<String>) -> Self {
        Self::ValidationError {
            message: message.into(),
        }
    }
}
pub struct ErrorManager {
    pub errors: HashMap<String, Error>,
}
impl ErrorManager {
    pub fn new() -> Self {
        Self { errors: HashMap::new() }
    }
    pub async fn create_error(&mut self, mut error: Error) -> Result<String> {
        error.id = uuid::Uuid::new_v4().to_string();
        error.created_at = Utc::now();
        error.updated_at = Utc::now();
        self.errors.insert(error.id.clone(), error.clone());
        Ok(error.id)
    }
    pub fn get_unresolved_errors(&self) -> Vec<&Error> {
        self.errors.values().filter(|error| !error.resolved).collect()
    }
    pub async fn resolve_error(
        &mut self,
        error_id: &str,
        resolution: String,
    ) -> Result<()> {
        if let Some(error) = self.errors.get_mut(error_id) {
            error.resolved = true;
            error.resolution = Some(resolution);
            error.resolved_at = Some(Utc::now());
            error.updated_at = Utc::now();
            Ok(())
        } else {
            Err(TodoziError::ValidationError {
                message: format!("Error {} not found", error_id),
            })
        }
    }
}
pub fn parse_error_format(error_text: &str) -> Result<Error> {
    let start_tag = "<error>";
    let end_tag = "</error>";
    let start = error_text
        .find(start_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing <error> start tag".to_string(),
        })?;
    let end = error_text
        .find(end_tag)
        .ok_or_else(|| TodoziError::ValidationError {
            message: "Missing </error> end tag".to_string(),
        })?;
    let content = &error_text[start + start_tag.len()..end];
    let parts: Vec<&str> = content.split(';').map(|s| s.trim()).collect();
    if parts.len() < 5 {
        return Err(TodoziError::ValidationError {
            message: "Invalid error format: need at least 5 parts (title; description; severity; category; source)"
                .to_string(),
        });
    }
    let tags = if parts.len() > 6 && !parts[6].is_empty() {
        parts[6].split(',').map(|s| s.trim().to_string()).collect()
    } else {
        Vec::new()
    };
    Ok(Error {
        id: uuid::Uuid::new_v4().to_string(),
        title: parts[0].to_string(),
        description: parts[1].to_string(),
        severity: parts[2]
            .parse::<ErrorSeverity>()
            .map_err(|_| TodoziError::ValidationError {
                message: "Invalid error severity".to_string(),
            })?,
        category: parts[3]
            .parse::<ErrorCategory>()
            .map_err(|_| TodoziError::ValidationError {
                message: "Invalid error category".to_string(),
            })?,
        source: parts[4].to_string(),
        context: if parts.len() > 5 && !parts[5].is_empty() {
            Some(parts[5].to_string())
        } else {
            None
        },
        tags,
        resolved: false,
        resolution: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        resolved_at: None,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_error_format() {
        let error_text = "<error>Database connection failed; Unable to connect to PostgreSQL database; critical; network; database-service; Connection timeout after 30 seconds; database,postgres,connection</error>";
        let error = parse_error_format(error_text).unwrap();
        assert_eq!(error.title, "Database connection failed");
        assert_eq!(error.description, "Unable to connect to PostgreSQL database");
        assert_eq!(error.severity, ErrorSeverity::Critical);
        assert_eq!(error.category, ErrorCategory::Network);
        assert_eq!(error.source, "database-service");
        assert_eq!(
            error.context, Some("Connection timeout after 30 seconds".to_string())
        );
        assert_eq!(error.tags, vec!["database", "postgres", "connection"]);
        assert_eq!(error.resolved, false);
    }
}

impl From<candle_core::Error> for TodoziError {
    fn from(err: candle_core::Error) -> Self {
        TodoziError::CandleError(err.to_string())
    }
}