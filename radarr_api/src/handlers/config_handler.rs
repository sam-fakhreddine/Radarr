//! Configuration and initialization handlers

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use crate::routes::AppState;

/// Initialize configuration response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeConfig {
    /// API key for authentication
    pub api_key: String,
    /// API root path
    pub api_root: String,
    /// Instance name
    pub instance_name: String,
    /// UI theme
    pub theme: String,
    /// URL base path
    pub url_base: String,
    /// Application version
    pub version: String,
    /// Production mode flag
    pub is_production: bool,
}

/// Get initialization configuration
///
/// Returns the configuration needed by the frontend to initialize.
///
/// # Errors
///
/// Returns an error if configuration cannot be serialized
pub async fn get_initialize_config(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let config = InitializeConfig {
        api_key: std::env::var("API_KEY").unwrap_or_default(),
        api_root: "/api/v3".to_string(),
        instance_name: std::env::var("INSTANCE_NAME").unwrap_or_else(|_| "Radarr".to_string()),
        theme: std::env::var("THEME").unwrap_or_else(|_| "auto".to_string()),
        url_base: std::env::var("URL_BASE").unwrap_or_default(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        is_production: cfg!(not(debug_assertions)),
    };

    Ok(Json(config))
}
