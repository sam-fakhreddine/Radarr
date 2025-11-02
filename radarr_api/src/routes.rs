//! API routing configuration

use crate::handlers::movie_handler::{
    create_movie, delete_movie, get_all_movies, get_movie_by_id, update_movie,
};
use axum::{routing::get, Router};
use radarr_core::service::movie_service::MovieService;
use std::sync::Arc;

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
