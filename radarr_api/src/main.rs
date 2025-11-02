//! Radarr API Server

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// HTTP handlers
mod handlers;

/// API resources (DTOs)
mod resources;

/// Request validation
mod validation;

/// API error handling
mod error;

/// API routing
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "radarr_api=debug,radarr_core=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Radarr API server starting...");

    // Load configuration
    let config = Arc::new(routes::Config::default());
    tracing::info!(
        "Configuration loaded: host={}, port={}, database={}",
        config.host,
        config.port,
        config.database_url
    );

    // Initialize database connection pool (SQLite for now)
    tracing::info!("Connecting to database: {}", config.database_url);

    // Create database file if it doesn't exist
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(config.database_url.trim_start_matches("sqlite:"))
                .create_if_missing(true),
        )
        .await?;

    // Run migrations
    tracing::info!("Running database migrations...");
    sqlx::migrate!("../migrations").run(&pool).await?;
    tracing::info!("Migrations completed successfully");

    // Create repository instance
    let repository_impl =
        Arc::new(radarr_core::repository::movie_repository::SqlxMovieRepository::new(pool));

    let movie_repository: Arc<dyn radarr_core::repository::traits::MovieRepository> =
        repository_impl.clone();
    let metadata_repository: Arc<dyn radarr_core::repository::traits::MovieMetadataRepository> =
        repository_impl;

    tracing::debug!("Movie and metadata repositories created");

    // Create service configuration
    let service_config = Arc::new(radarr_core::service::movie_service::Config {
        availability_delay: config.availability_delay,
    });

    // Create service instance
    let movie_service = Arc::new(radarr_core::service::movie_service::MovieService::new(
        movie_repository,
        metadata_repository,
        service_config,
    ));
    tracing::debug!("Movie service created");

    // Build router with state
    let app_state = routes::AppState::new(movie_service, config.clone());
    let app = routes::movie_routes().with_state(app_state);
    tracing::debug!("Router configured");

    // Start HTTP server
    let addr = format!("{}:{}", config.host, config.port);
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
