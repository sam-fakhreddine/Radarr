//! Repository trait definitions

use crate::domain::movie::Movie;
use crate::error::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Repository trait for movie data access operations
#[async_trait]
pub trait MovieRepository: Send + Sync {
    /// Gets a movie by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The movie ID
    ///
    /// # Returns
    ///
    /// Returns the movie with loaded metadata
    ///
    /// # Errors
    ///
    /// Returns `Error::NotFound` if the movie doesn't exist
    /// Returns `Error::Database` for database errors
    async fn get(&self, id: i32) -> Result<Movie>;

    /// Finds a movie by ID, returning None if not found
    ///
    /// # Arguments
    ///
    /// * `id` - The movie ID
    ///
    /// # Returns
    ///
    /// Returns `Some(Movie)` if found, `None` otherwise
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn find(&self, id: i32) -> Result<Option<Movie>>;

    /// Gets all movies
    ///
    /// # Returns
    ///
    /// Returns all movies with loaded metadata
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn all(&self) -> Result<Vec<Movie>>;

    /// Inserts a new movie
    ///
    /// # Arguments
    ///
    /// * `movie` - The movie to insert
    ///
    /// # Returns
    ///
    /// Returns the inserted movie with generated ID
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    /// Returns `Error::Validation` for constraint violations
    async fn insert(&self, movie: &Movie) -> Result<Movie>;

    /// Updates an existing movie
    ///
    /// # Arguments
    ///
    /// * `movie` - The movie to update
    ///
    /// # Returns
    ///
    /// Returns the updated movie
    ///
    /// # Errors
    ///
    /// Returns `Error::NotFound` if the movie doesn't exist
    /// Returns `Error::Database` for database errors
    async fn update(&self, movie: &Movie) -> Result<Movie>;

    /// Deletes a movie by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The movie ID to delete
    ///
    /// # Errors
    ///
    /// Returns `Error::NotFound` if the movie doesn't exist
    /// Returns `Error::Database` for database errors
    async fn delete(&self, id: i32) -> Result<()>;

    /// Finds a movie by TMDB ID
    ///
    /// # Arguments
    ///
    /// * `tmdb_id` - The TMDB ID
    ///
    /// # Returns
    ///
    /// Returns `Some(Movie)` if found, `None` otherwise
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<Movie>>;

    /// Finds a movie by IMDB ID
    ///
    /// # Arguments
    ///
    /// * `imdb_id` - The IMDB ID
    ///
    /// # Returns
    ///
    /// Returns `Some(Movie)` if found, `None` otherwise
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn find_by_imdb_id(&self, imdb_id: &str) -> Result<Option<Movie>>;

    /// Finds a movie by path
    ///
    /// # Arguments
    ///
    /// * `path` - The file system path
    ///
    /// # Returns
    ///
    /// Returns `Some(Movie)` if found, `None` otherwise
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn find_by_path(&self, path: &str) -> Result<Option<Movie>>;

    /// Finds movies by titles (clean title matching)
    ///
    /// # Arguments
    ///
    /// * `titles` - List of clean titles to search for
    ///
    /// # Returns
    ///
    /// Returns all movies matching any of the titles
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn find_by_titles(&self, titles: &[String]) -> Result<Vec<Movie>>;

    /// Inserts multiple movies in a transaction
    ///
    /// # Arguments
    ///
    /// * `movies` - The movies to insert
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    /// Rolls back all inserts on error
    async fn insert_many(&self, movies: &[Movie]) -> Result<()>;

    /// Updates multiple movies in a transaction
    ///
    /// # Arguments
    ///
    /// * `movies` - The movies to update
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    /// Rolls back all updates on error
    async fn update_many(&self, movies: &[Movie]) -> Result<()>;

    /// Deletes multiple movies by IDs in a transaction
    ///
    /// # Arguments
    ///
    /// * `ids` - The movie IDs to delete
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    /// Rolls back all deletes on error
    async fn delete_many(&self, ids: &[i32]) -> Result<()>;

    /// Gets all movie paths as a map of ID to path
    ///
    /// # Returns
    ///
    /// Returns a `HashMap` of movie ID to path
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn all_movie_paths(&self) -> Result<HashMap<i32, String>>;

    /// Gets all movie TMDB IDs
    ///
    /// # Returns
    ///
    /// Returns a list of all TMDB IDs
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn all_movie_tmdb_ids(&self) -> Result<Vec<i32>>;

    /// Gets movies between dates
    ///
    /// # Arguments
    ///
    /// * `start` - Start date
    /// * `end` - End date
    /// * `include_unmonitored` - Whether to include unmonitored movies
    ///
    /// # Returns
    ///
    /// Returns movies added between the specified dates
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn movies_between_dates(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        include_unmonitored: bool,
    ) -> Result<Vec<Movie>>;
}

/// Repository trait for movie metadata data access operations
#[async_trait]
pub trait MovieMetadataRepository: Send + Sync {
    /// Finds movie metadata by TMDB ID
    ///
    /// # Arguments
    ///
    /// * `tmdb_id` - The TMDB ID
    ///
    /// # Returns
    ///
    /// Returns `Some(MovieMetadata)` if found, `None` otherwise
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn find_by_tmdb_id(
        &self,
        tmdb_id: i32,
    ) -> Result<Option<crate::domain::movie_metadata::MovieMetadata>>;

    /// Inserts new movie metadata
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata to insert
    ///
    /// # Returns
    ///
    /// Returns the inserted metadata with generated ID
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    /// Returns `Error::Validation` for constraint violations
    async fn insert(
        &self,
        metadata: &crate::domain::movie_metadata::MovieMetadata,
    ) -> Result<crate::domain::movie_metadata::MovieMetadata>;
}
