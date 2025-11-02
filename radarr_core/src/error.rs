//! Error types for the core library

use thiserror::Error;

/// Result type alias for core operations
pub type Result<T> = std::result::Result<T, Error>;

/// Core error types
#[derive(Debug, Error)]
pub enum Error {
    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Internal error
    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Error::NotFound("Resource not found".to_string()),
            _ => Error::Database(err.to_string()),
        }
    }
}
