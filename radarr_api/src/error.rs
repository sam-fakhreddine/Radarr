//! API error handling

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use radarr_core::error::Error as CoreError;
use serde::Serialize;

/// API error response
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// Error message
    pub message: String,
    /// Optional validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

impl IntoResponse for CoreError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            CoreError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            CoreError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            CoreError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string(),
            ),
            CoreError::Serialization(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Serialization error occurred".to_string(),
            ),
            CoreError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(ApiError {
            message,
            errors: None,
        });

        (status, body).into_response()
    }
}
