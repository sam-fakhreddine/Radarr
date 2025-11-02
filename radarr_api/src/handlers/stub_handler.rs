//! Stub handlers for endpoints not yet implemented
//!
//! These handlers return minimal valid responses to allow the frontend to load
//! while we implement the full functionality.

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

/// Get system status
///
/// Returns basic system information
pub async fn get_system_status() -> impl IntoResponse {
    Json(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "buildTime": "2024-01-01T00:00:00Z",
        "isDebug": cfg!(debug_assertions),
        "isProduction": cfg!(not(debug_assertions)),
        "isAdmin": true,
        "isUserInteractive": true,
        "startupPath": "/app",
        "appData": "/app/data",
        "osName": std::env::consts::OS,
        "osVersion": "",
        "isMonoRuntime": false,
        "isMono": false,
        "isLinux": cfg!(target_os = "linux"),
        "isOsx": cfg!(target_os = "macos"),
        "isWindows": cfg!(target_os = "windows"),
        "isDocker": false,
        "mode": "console",
        "branch": "rust",
        "authentication": "none",
        "sqliteVersion": "3.0.0",
        "urlBase": std::env::var("URL_BASE").unwrap_or_default(),
        "runtimeVersion": "rust",
        "runtimeName": "rust",
        "startTime": "2024-01-01T00:00:00Z",
        "packageVersion": env!("CARGO_PKG_VERSION"),
        "packageAuthor": "Radarr Contributors",
        "packageUpdateMechanism": "docker"
    }))
}

/// Get UI configuration
pub async fn get_ui_config() -> impl IntoResponse {
    Json(json!({
        "firstDayOfWeek": 0,
        "calendarWeekColumnHeader": "ddd M/D",
        "movieRuntimeFormat": "hoursMinutes",
        "shortDateFormat": "MMM D YYYY",
        "longDateFormat": "dddd, MMMM D YYYY",
        "timeFormat": "h:mm A",
        "showRelativeDates": true,
        "enableColorImpairedMode": false,
        "movieInfoLanguage": 1,
        "uiLanguage": 1,
        "theme": "auto",
        "id": 1
    }))
}

/// Get quality profiles
pub async fn get_quality_profiles() -> impl IntoResponse {
    Json(json!([
        {
            "id": 1,
            "name": "Any",
            "upgradeAllowed": true,
            "cutoff": 20,
            "items": [],
            "minFormatScore": 0,
            "cutoffFormatScore": 0,
            "formatItems": [],
            "language": {
                "id": 1,
                "name": "English"
            }
        }
    ]))
}

/// Get languages
pub async fn get_languages() -> impl IntoResponse {
    Json(json!([
        {
            "id": 1,
            "name": "English"
        }
    ]))
}

/// Get localization
pub async fn get_localization() -> impl IntoResponse {
    // Load English translations from the legacy localization file
    let translations_path = "legacy/src/NzbDrone.Core/Localization/Core/en.json";
    
    match tokio::fs::read_to_string(translations_path).await {
        Ok(content) => {
            match serde_json::from_str::<serde_json::Value>(&content) {
                Ok(strings) => Json(json!({
                    "Strings": strings
                })),
                Err(_) => Json(json!({
                    "Strings": {}
                })),
            }
        }
        Err(_) => Json(json!({
            "Strings": {}
        })),
    }
}

/// Get localization language
pub async fn get_localization_language() -> impl IntoResponse {
    Json(json!("en"))
}

/// Get tags
pub async fn get_tags() -> impl IntoResponse {
    Json(json!([]))
}

/// Get collections
pub async fn get_collections() -> impl IntoResponse {
    Json(json!([]))
}

/// Get custom filters
pub async fn get_custom_filters() -> impl IntoResponse {
    Json(json!([]))
}

/// Get indexer flags
pub async fn get_indexer_flags() -> impl IntoResponse {
    Json(json!([]))
}

/// Get import lists
pub async fn get_import_lists() -> impl IntoResponse {
    Json(json!([]))
}

/// Catch-all for unimplemented endpoints
#[allow(dead_code)]
pub async fn not_implemented() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "message": "This endpoint is not yet implemented in the Rust version"
        })),
    )
}
