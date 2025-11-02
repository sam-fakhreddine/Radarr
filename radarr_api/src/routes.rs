//! API routing configuration

use crate::handlers::config_handler::get_initialize_config;
use crate::handlers::movie_handler::{
    create_movie, delete_movie, get_all_movies, get_movie_by_id, update_movie,
};
use crate::handlers::stub_handler::{
    get_collections, get_custom_filters, get_import_lists, get_indexer_flags, get_languages,
    get_localization, get_localization_language, get_quality_profiles, get_system_status, get_tags,
    get_ui_config,
};
use axum::{
    body::Body,
    http::{header, StatusCode},
    response::Response,
    routing::get,
    Router,
};
use radarr_core::service::movie_service::MovieService;
use std::sync::Arc;
use tower_http::services::ServeDir;

/// Application configuration
#[derive(Clone, Debug)]
pub struct Config {
    /// Database URL
    pub database_url: String,
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Availability delay in days
    pub availability_delay: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite:radarr.db".to_string()),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(7878),
            availability_delay: std::env::var("AVAILABILITY_DELAY")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(0),
        }
    }
}

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    /// Movie service for business logic
    pub movie_service: Arc<MovieService>,
    /// Application configuration
    #[allow(dead_code)] // Will be used for availability delay and other features
    pub config: Arc<Config>,
}

impl AppState {
    /// Creates a new `AppState`
    ///
    /// # Arguments
    ///
    /// * `movie_service` - The movie service instance
    /// * `config` - The application configuration
    ///
    /// # Returns
    ///
    /// Returns a new `AppState`
    #[must_use]
    pub const fn new(movie_service: Arc<MovieService>, config: Arc<Config>) -> Self {
        Self {
            movie_service,
            config,
        }
    }
}

/// Creates the movie routes
///
/// Defines all movie-related API endpoints:
/// - GET /api/v3/movie - Get all movies or filter by TMDB ID
/// - POST /api/v3/movie - Create a new movie
/// - GET /api/v3/movie/:id - Get a movie by ID
/// - PUT /api/v3/movie/:id - Update a movie
/// - DELETE /api/v3/movie/:id - Delete a movie
///
/// # Returns
///
/// Returns a Router configured with movie endpoints
pub fn movie_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v3/movie", get(get_all_movies).post(create_movie))
        .route(
            "/api/v3/movie/:id",
            get(get_movie_by_id).put(update_movie).delete(delete_movie),
        )
}

/// Serves index.html with URL_BASE replaced
async fn serve_index_html() -> Result<Response, StatusCode> {
    let url_base = std::env::var("URL_BASE").unwrap_or_default();
    let index_path = "legacy/_output/UI/index.html";

    let content = tokio::fs::read_to_string(index_path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let content = content.replace("__URL_BASE__", &url_base);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8")
        .body(Body::from(content))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}

/// Creates stub API routes for unimplemented endpoints
///
/// These routes return minimal valid responses to allow the frontend to load
fn stub_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v3/system/status", get(get_system_status))
        .route("/api/v3/config/ui", get(get_ui_config))
        .route("/api/v3/qualityprofile", get(get_quality_profiles))
        .route("/api/v3/language", get(get_languages))
        .route("/api/v3/localization", get(get_localization))
        .route("/api/v3/localization/language", get(get_localization_language))
        .route("/api/v3/tag", get(get_tags))
        .route("/api/v3/collection", get(get_collections))
        .route("/api/v3/customFilter", get(get_custom_filters))
        .route("/api/v3/indexerFlag", get(get_indexer_flags))
        .route("/api/v3/importlist", get(get_import_lists))
}

/// Creates the complete application router with API and static file serving
///
/// This router includes:
/// - API routes under /api/v3/*
/// - Initialize configuration endpoint
/// - Static file serving from legacy/_output/UI
/// - SPA fallback to index.html for client-side routing
///
/// # Arguments
///
/// * `state` - The application state
///
/// # Returns
///
/// Returns a Router configured with all routes
pub fn app_router(state: AppState) -> Router {
    // Serve static files from the legacy frontend build output
    let serve_dir = ServeDir::new("legacy/_output/UI")
        .precompressed_gzip()
        .precompressed_br();

    Router::new()
        // Initialize configuration endpoint
        .route("/initialize.json", get(get_initialize_config))
        // API routes
        .merge(movie_routes())
        .merge(stub_routes())
        // Custom index.html handler for root and explicit index.html
        .route("/", get(serve_index_html))
        .route("/index.html", get(serve_index_html))
        // Static files for everything else
        .fallback_service(serve_dir)
        // State for API routes
        .with_state(state)
}
