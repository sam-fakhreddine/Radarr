//! Radarr API Server

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// HTTP handlers
mod handlers;

/// API resources (DTOs)
mod resources;

/// Request validation
mod validation;

/// API error handling
mod error;

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

    // TODO: Initialize database connection pool
    // TODO: Run migrations
    // TODO: Create repository instances
    // TODO: Create service instances
    // TODO: Build router with state
    // TODO: Start HTTP server

    Ok(())
}
