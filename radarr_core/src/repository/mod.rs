//! Repository layer for data access

/// Repository trait definitions
pub mod traits;

/// Movie repository implementation
pub mod movie_repository;

// Re-export repository implementations
pub use movie_repository::{PostgresMovieRepository, SqlxMovieRepository};
pub use traits::MovieRepository;
