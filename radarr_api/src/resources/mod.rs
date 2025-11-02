//! API resource types (DTOs)

/// Movie resource
pub mod movie_resource;

// Re-export commonly used types
#[allow(unused_imports)] // Will be used by handlers
pub use movie_resource::{
    DeleteMovieParams, MediaCover, MovieCollection, MovieQueryParams, MovieResource,
    MovieStatistics, UpdateMovieParams,
};
