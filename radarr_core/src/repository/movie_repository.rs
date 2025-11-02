//! Movie repository implementation

use crate::domain::movie::Movie;
use crate::domain::movie_metadata::MovieMetadata;
use crate::error::{Error, Result};
use crate::repository::traits::MovieRepository;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Pool, Postgres, Row, Sqlite};
use std::collections::HashMap;

/// `SQLite` implementation of the `MovieRepository` trait
pub struct SqlxMovieRepository {
    pool: Pool<Sqlite>,
}

/// `PostgreSQL` implementation of the `MovieRepository` trait
pub struct PostgresMovieRepository {
    pool: Pool<Postgres>,
}

impl SqlxMovieRepository {
    /// Creates a new `SqlxMovieRepository`
    ///
    /// # Arguments
    ///
    /// * `pool` - The `SQLite` connection pool
    #[must_use]
    pub const fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Gets the connection pool
    #[must_use]
    pub const fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }

    /// Loads metadata for a slice of movies
    ///
    /// # Arguments
    ///
    /// * `movies` - Mutable slice of movies to load metadata for
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn load_metadata(&self, movies: &mut [Movie]) -> Result<()> {
        if movies.is_empty() {
            return Ok(());
        }

        // Collect unique metadata IDs
        let metadata_ids: Vec<i32> = movies
            .iter()
            .map(|m| m.movie_metadata_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        // Build query with placeholders
        let placeholders = metadata_ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");
        let query = format!("SELECT * FROM MovieMetadata WHERE Id IN ({placeholders})");

        // Fetch metadata
        let mut query_builder = sqlx::query_as::<_, MovieMetadata>(&query);
        for id in &metadata_ids {
            query_builder = query_builder.bind(id);
        }

        let metadata_list = query_builder.fetch_all(&self.pool).await?;

        // Create a map for quick lookup
        let metadata_map: HashMap<i32, MovieMetadata> =
            metadata_list.into_iter().map(|m| (m.id, m)).collect();

        // Assign metadata to movies
        for movie in movies {
            movie.movie_metadata = metadata_map.get(&movie.movie_metadata_id).cloned();
        }

        Ok(())
    }
}

#[async_trait]
impl MovieRepository for SqlxMovieRepository {
    async fn get(&self, id: i32) -> Result<Movie> {
        let mut movie = sqlx::query_as::<_, Movie>("SELECT * FROM Movies WHERE Id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    Error::NotFound(format!("Movie with id {id} not found"))
                }
                _ => Error::Database(e.to_string()),
            })?;

        // Load metadata
        self.load_metadata(std::slice::from_mut(&mut movie)).await?;

        Ok(movie)
    }

    async fn find(&self, id: i32) -> Result<Option<Movie>> {
        let movie_result = sqlx::query_as::<_, Movie>("SELECT * FROM Movies WHERE Id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(mut movie) = movie_result {
            self.load_metadata(std::slice::from_mut(&mut movie)).await?;
            Ok(Some(movie))
        } else {
            Ok(None)
        }
    }

    async fn all(&self) -> Result<Vec<Movie>> {
        let mut movies = sqlx::query_as::<_, Movie>("SELECT * FROM Movies")
            .fetch_all(&self.pool)
            .await?;

        // Load metadata for all movies efficiently
        self.load_metadata(&mut movies).await?;

        Ok(movies)
    }

    async fn insert(&self, movie: &Movie) -> Result<Movie> {
        let result = sqlx::query(
            r"
            INSERT INTO Movies (
                MovieMetadataId, Monitored, MinimumAvailability,
                QualityProfileId, Path, RootFolderPath, Added,
                Tags, AddOptions, LastSearchTime, MovieFileId
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ",
        )
        .bind(movie.movie_metadata_id)
        .bind(movie.monitored)
        .bind(movie.minimum_availability)
        .bind(movie.quality_profile_id)
        .bind(&movie.path)
        .bind(&movie.root_folder_path)
        .bind(movie.added)
        .bind(&movie.tags)
        .bind(&movie.add_options)
        .bind(movie.last_search_time)
        .bind(movie.movie_file_id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                db_err.code().map_or_else(
                    || Error::Database(db_err.to_string()),
                    |code| {
                        if code == "2067" || code == "1555" {
                            // SQLite UNIQUE constraint violation
                            Error::Validation("Movie already exists".to_string())
                        } else if code == "787" || code == "1811" {
                            // SQLite FOREIGN KEY constraint violation
                            Error::Validation("Invalid foreign key reference".to_string())
                        } else {
                            Error::Database(db_err.to_string())
                        }
                    },
                )
            }
            _ => Error::Database(e.to_string()),
        })?;

        let id = i32::try_from(result.last_insert_rowid())
            .map_err(|e| Error::Internal(format!("ID overflow: {e}")))?;
        self.get(id).await
    }

    async fn update(&self, movie: &Movie) -> Result<Movie> {
        let result = sqlx::query(
            r"
            UPDATE Movies SET
                MovieMetadataId = ?,
                Monitored = ?,
                MinimumAvailability = ?,
                QualityProfileId = ?,
                Path = ?,
                RootFolderPath = ?,
                Added = ?,
                Tags = ?,
                AddOptions = ?,
                LastSearchTime = ?,
                MovieFileId = ?
            WHERE Id = ?
            ",
        )
        .bind(movie.movie_metadata_id)
        .bind(movie.monitored)
        .bind(movie.minimum_availability)
        .bind(movie.quality_profile_id)
        .bind(&movie.path)
        .bind(&movie.root_folder_path)
        .bind(movie.added)
        .bind(&movie.tags)
        .bind(&movie.add_options)
        .bind(movie.last_search_time)
        .bind(movie.movie_file_id)
        .bind(movie.id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                db_err.code().map_or_else(
                    || Error::Database(db_err.to_string()),
                    |code| {
                        if code == "2067" || code == "1555" {
                            // SQLite UNIQUE constraint violation
                            Error::Validation("Movie path already exists".to_string())
                        } else if code == "787" || code == "1811" {
                            // SQLite FOREIGN KEY constraint violation
                            Error::Validation("Invalid foreign key reference".to_string())
                        } else {
                            Error::Database(db_err.to_string())
                        }
                    },
                )
            }
            _ => Error::Database(e.to_string()),
        })?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!(
                "Movie with id {} not found",
                movie.id
            )));
        }

        self.get(movie.id).await
    }

    async fn delete(&self, id: i32) -> Result<()> {
        let result = sqlx::query("DELETE FROM Movies WHERE Id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!("Movie with id {id} not found")));
        }

        Ok(())
    }

    async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<Movie>> {
        let movie_result = sqlx::query_as::<_, Movie>(
            "SELECT m.* FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id 
             WHERE mm.TmdbId = ?",
        )
        .bind(tmdb_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(mut movie) = movie_result {
            self.load_metadata(std::slice::from_mut(&mut movie)).await?;
            Ok(Some(movie))
        } else {
            Ok(None)
        }
    }

    async fn find_by_imdb_id(&self, imdb_id: &str) -> Result<Option<Movie>> {
        let movie_result = sqlx::query_as::<_, Movie>(
            "SELECT m.* FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id 
             WHERE mm.ImdbId = ?",
        )
        .bind(imdb_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(mut movie) = movie_result {
            self.load_metadata(std::slice::from_mut(&mut movie)).await?;
            Ok(Some(movie))
        } else {
            Ok(None)
        }
    }

    async fn find_by_path(&self, path: &str) -> Result<Option<Movie>> {
        let movie_result = sqlx::query_as::<_, Movie>("SELECT * FROM Movies WHERE Path = ?")
            .bind(path)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(mut movie) = movie_result {
            self.load_metadata(std::slice::from_mut(&mut movie)).await?;
            Ok(Some(movie))
        } else {
            Ok(None)
        }
    }

    async fn find_by_titles(&self, titles: &[String]) -> Result<Vec<Movie>> {
        if titles.is_empty() {
            return Ok(Vec::new());
        }

        // Build query with placeholders
        let placeholders = titles.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT m.* FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id 
             WHERE mm.CleanTitle IN ({placeholders})"
        );

        // Fetch movies
        let mut query_builder = sqlx::query_as::<_, Movie>(&query);
        for title in titles {
            query_builder = query_builder.bind(title);
        }

        let mut movies = query_builder.fetch_all(&self.pool).await?;

        self.load_metadata(&mut movies).await?;

        Ok(movies)
    }

    async fn insert_many(&self, movies: &[Movie]) -> Result<()> {
        if movies.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        for movie in movies {
            sqlx::query(
                r"
                INSERT INTO Movies (
                    MovieMetadataId, Monitored, MinimumAvailability,
                    QualityProfileId, Path, RootFolderPath, Added,
                    Tags, AddOptions, LastSearchTime, MovieFileId
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ",
            )
            .bind(movie.movie_metadata_id)
            .bind(movie.monitored)
            .bind(movie.minimum_availability)
            .bind(movie.quality_profile_id)
            .bind(&movie.path)
            .bind(&movie.root_folder_path)
            .bind(movie.added)
            .bind(&movie.tags)
            .bind(&movie.add_options)
            .bind(movie.last_search_time)
            .bind(movie.movie_file_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn update_many(&self, movies: &[Movie]) -> Result<()> {
        if movies.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        for movie in movies {
            sqlx::query(
                r"
                UPDATE Movies SET
                    MovieMetadataId = ?,
                    Monitored = ?,
                    MinimumAvailability = ?,
                    QualityProfileId = ?,
                    Path = ?,
                    RootFolderPath = ?,
                    Added = ?,
                    Tags = ?,
                    AddOptions = ?,
                    LastSearchTime = ?,
                    MovieFileId = ?
                WHERE Id = ?
                ",
            )
            .bind(movie.movie_metadata_id)
            .bind(movie.monitored)
            .bind(movie.minimum_availability)
            .bind(movie.quality_profile_id)
            .bind(&movie.path)
            .bind(&movie.root_folder_path)
            .bind(movie.added)
            .bind(&movie.tags)
            .bind(&movie.add_options)
            .bind(movie.last_search_time)
            .bind(movie.movie_file_id)
            .bind(movie.id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn delete_many(&self, ids: &[i32]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        // Build query with placeholders
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!("DELETE FROM Movies WHERE Id IN ({placeholders})");

        // Execute delete
        let mut query_builder = sqlx::query(&query);
        for id in ids {
            query_builder = query_builder.bind(id);
        }

        query_builder.execute(&mut *tx).await?;

        tx.commit().await?;
        Ok(())
    }

    async fn all_movie_paths(&self) -> Result<HashMap<i32, String>> {
        #[derive(sqlx::FromRow)]
        struct MoviePath {
            #[sqlx(rename = "Id")]
            id: i32,
            #[sqlx(rename = "Path")]
            path: String,
        }

        let rows = sqlx::query_as::<_, MoviePath>("SELECT Id, Path FROM Movies")
            .fetch_all(&self.pool)
            .await?;

        let map = rows.into_iter().map(|row| (row.id, row.path)).collect();

        Ok(map)
    }

    async fn all_movie_tmdb_ids(&self) -> Result<Vec<i32>> {
        #[derive(sqlx::FromRow)]
        struct TmdbIdRow {
            #[sqlx(rename = "TmdbId")]
            tmdb_id: i32,
        }

        let rows = sqlx::query_as::<_, TmdbIdRow>(
            "SELECT mm.TmdbId FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id",
        )
        .fetch_all(&self.pool)
        .await?;

        let ids = rows.into_iter().map(|row| row.tmdb_id).collect();

        Ok(ids)
    }

    async fn movies_between_dates(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        include_unmonitored: bool,
    ) -> Result<Vec<Movie>> {
        let mut movies = if include_unmonitored {
            sqlx::query_as::<_, Movie>("SELECT * FROM Movies WHERE Added >= ? AND Added <= ?")
                .bind(start)
                .bind(end)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as::<_, Movie>(
                "SELECT * FROM Movies WHERE Added >= ? AND Added <= ? AND Monitored = 1",
            )
            .bind(start)
            .bind(end)
            .fetch_all(&self.pool)
            .await?
        };

        self.load_metadata(&mut movies).await?;

        Ok(movies)
    }
}

impl PostgresMovieRepository {
    /// Creates a new `PostgresMovieRepository`
    ///
    /// # Arguments
    ///
    /// * `pool` - The `PostgreSQL` connection pool
    #[must_use]
    pub const fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Loads metadata for a slice of movies
    ///
    /// # Arguments
    ///
    /// * `movies` - Mutable slice of movies to load metadata for
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    async fn load_metadata(&self, movies: &mut [Movie]) -> Result<()> {
        if movies.is_empty() {
            return Ok(());
        }

        // Collect unique metadata IDs
        let metadata_ids: Vec<i32> = movies
            .iter()
            .map(|m| m.movie_metadata_id)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        // Fetch metadata using ANY for PostgreSQL
        let metadata_list =
            sqlx::query_as::<_, MovieMetadata>("SELECT * FROM MovieMetadata WHERE Id = ANY($1)")
                .bind(&metadata_ids)
                .fetch_all(&self.pool)
                .await?;

        // Create a map for quick lookup
        let metadata_map: HashMap<i32, MovieMetadata> =
            metadata_list.into_iter().map(|m| (m.id, m)).collect();

        // Assign metadata to movies
        for movie in movies {
            movie.movie_metadata = metadata_map.get(&movie.movie_metadata_id).cloned();
        }

        Ok(())
    }
}

#[async_trait]
impl MovieRepository for PostgresMovieRepository {
    async fn get(&self, id: i32) -> Result<Movie> {
        let mut movie = sqlx::query_as::<_, Movie>("SELECT * FROM Movies WHERE Id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    Error::NotFound(format!("Movie with id {id} not found"))
                }
                _ => Error::Database(e.to_string()),
            })?;

        // Load metadata
        self.load_metadata(std::slice::from_mut(&mut movie)).await?;

        Ok(movie)
    }

    async fn find(&self, id: i32) -> Result<Option<Movie>> {
        let movie_result = sqlx::query_as::<_, Movie>("SELECT * FROM Movies WHERE Id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(mut movie) = movie_result {
            self.load_metadata(std::slice::from_mut(&mut movie)).await?;
            Ok(Some(movie))
        } else {
            Ok(None)
        }
    }

    async fn all(&self) -> Result<Vec<Movie>> {
        let mut movies = sqlx::query_as::<_, Movie>("SELECT * FROM Movies")
            .fetch_all(&self.pool)
            .await?;

        // Load metadata for all movies efficiently
        self.load_metadata(&mut movies).await?;

        Ok(movies)
    }

    async fn insert(&self, movie: &Movie) -> Result<Movie> {
        let result = sqlx::query(
            r"
            INSERT INTO Movies (
                MovieMetadataId, Monitored, MinimumAvailability,
                QualityProfileId, Path, RootFolderPath, Added,
                Tags, AddOptions, LastSearchTime, MovieFileId
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING Id
            ",
        )
        .bind(movie.movie_metadata_id)
        .bind(movie.monitored)
        .bind(movie.minimum_availability)
        .bind(movie.quality_profile_id)
        .bind(&movie.path)
        .bind(&movie.root_folder_path)
        .bind(movie.added)
        .bind(&movie.tags)
        .bind(&movie.add_options)
        .bind(movie.last_search_time)
        .bind(movie.movie_file_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                db_err.code().map_or_else(
                    || Error::Database(db_err.to_string()),
                    |code| {
                        if code == "23505" {
                            // PostgreSQL UNIQUE constraint violation
                            Error::Validation("Movie already exists".to_string())
                        } else if code == "23503" {
                            // PostgreSQL FOREIGN KEY constraint violation
                            Error::Validation("Invalid foreign key reference".to_string())
                        } else {
                            Error::Database(db_err.to_string())
                        }
                    },
                )
            }
            _ => Error::Database(e.to_string()),
        })?;

        let id: i32 = result.try_get("Id")?;
        self.get(id).await
    }

    async fn update(&self, movie: &Movie) -> Result<Movie> {
        let result = sqlx::query(
            r"
            UPDATE Movies SET
                MovieMetadataId = $1,
                Monitored = $2,
                MinimumAvailability = $3,
                QualityProfileId = $4,
                Path = $5,
                RootFolderPath = $6,
                Added = $7,
                Tags = $8,
                AddOptions = $9,
                LastSearchTime = $10,
                MovieFileId = $11
            WHERE Id = $12
            ",
        )
        .bind(movie.movie_metadata_id)
        .bind(movie.monitored)
        .bind(movie.minimum_availability)
        .bind(movie.quality_profile_id)
        .bind(&movie.path)
        .bind(&movie.root_folder_path)
        .bind(movie.added)
        .bind(&movie.tags)
        .bind(&movie.add_options)
        .bind(movie.last_search_time)
        .bind(movie.movie_file_id)
        .bind(movie.id)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                db_err.code().map_or_else(
                    || Error::Database(db_err.to_string()),
                    |code| {
                        if code == "23505" {
                            // PostgreSQL UNIQUE constraint violation
                            Error::Validation("Movie path already exists".to_string())
                        } else if code == "23503" {
                            // PostgreSQL FOREIGN KEY constraint violation
                            Error::Validation("Invalid foreign key reference".to_string())
                        } else {
                            Error::Database(db_err.to_string())
                        }
                    },
                )
            }
            _ => Error::Database(e.to_string()),
        })?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!(
                "Movie with id {} not found",
                movie.id
            )));
        }

        self.get(movie.id).await
    }

    async fn delete(&self, id: i32) -> Result<()> {
        let result = sqlx::query("DELETE FROM Movies WHERE Id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!("Movie with id {id} not found")));
        }

        Ok(())
    }

    async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<Movie>> {
        let movie_result = sqlx::query_as::<_, Movie>(
            "SELECT m.* FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id 
             WHERE mm.TmdbId = $1",
        )
        .bind(tmdb_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(mut movie) = movie_result {
            self.load_metadata(std::slice::from_mut(&mut movie)).await?;
            Ok(Some(movie))
        } else {
            Ok(None)
        }
    }

    async fn find_by_imdb_id(&self, imdb_id: &str) -> Result<Option<Movie>> {
        let movie_result = sqlx::query_as::<_, Movie>(
            "SELECT m.* FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id 
             WHERE mm.ImdbId = $1",
        )
        .bind(imdb_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(mut movie) = movie_result {
            self.load_metadata(std::slice::from_mut(&mut movie)).await?;
            Ok(Some(movie))
        } else {
            Ok(None)
        }
    }

    async fn find_by_path(&self, path: &str) -> Result<Option<Movie>> {
        let movie_result = sqlx::query_as::<_, Movie>("SELECT * FROM Movies WHERE Path = $1")
            .bind(path)
            .fetch_optional(&self.pool)
            .await?;

        if let Some(mut movie) = movie_result {
            self.load_metadata(std::slice::from_mut(&mut movie)).await?;
            Ok(Some(movie))
        } else {
            Ok(None)
        }
    }

    async fn find_by_titles(&self, titles: &[String]) -> Result<Vec<Movie>> {
        if titles.is_empty() {
            return Ok(Vec::new());
        }

        // Use ANY for PostgreSQL
        let mut movies = sqlx::query_as::<_, Movie>(
            "SELECT m.* FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id 
             WHERE mm.CleanTitle = ANY($1)",
        )
        .bind(titles)
        .fetch_all(&self.pool)
        .await?;

        self.load_metadata(&mut movies).await?;

        Ok(movies)
    }

    async fn insert_many(&self, movies: &[Movie]) -> Result<()> {
        if movies.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        for movie in movies {
            sqlx::query(
                r"
                INSERT INTO Movies (
                    MovieMetadataId, Monitored, MinimumAvailability,
                    QualityProfileId, Path, RootFolderPath, Added,
                    Tags, AddOptions, LastSearchTime, MovieFileId
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ",
            )
            .bind(movie.movie_metadata_id)
            .bind(movie.monitored)
            .bind(movie.minimum_availability)
            .bind(movie.quality_profile_id)
            .bind(&movie.path)
            .bind(&movie.root_folder_path)
            .bind(movie.added)
            .bind(&movie.tags)
            .bind(&movie.add_options)
            .bind(movie.last_search_time)
            .bind(movie.movie_file_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn update_many(&self, movies: &[Movie]) -> Result<()> {
        if movies.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        for movie in movies {
            sqlx::query(
                r"
                UPDATE Movies SET
                    MovieMetadataId = $1,
                    Monitored = $2,
                    MinimumAvailability = $3,
                    QualityProfileId = $4,
                    Path = $5,
                    RootFolderPath = $6,
                    Added = $7,
                    Tags = $8,
                    AddOptions = $9,
                    LastSearchTime = $10,
                    MovieFileId = $11
                WHERE Id = $12
                ",
            )
            .bind(movie.movie_metadata_id)
            .bind(movie.monitored)
            .bind(movie.minimum_availability)
            .bind(movie.quality_profile_id)
            .bind(&movie.path)
            .bind(&movie.root_folder_path)
            .bind(movie.added)
            .bind(&movie.tags)
            .bind(&movie.add_options)
            .bind(movie.last_search_time)
            .bind(movie.movie_file_id)
            .bind(movie.id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn delete_many(&self, ids: &[i32]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        // Use ANY for PostgreSQL
        sqlx::query("DELETE FROM Movies WHERE Id = ANY($1)")
            .bind(ids)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn all_movie_paths(&self) -> Result<HashMap<i32, String>> {
        #[derive(sqlx::FromRow)]
        struct MoviePath {
            #[sqlx(rename = "Id")]
            id: i32,
            #[sqlx(rename = "Path")]
            path: String,
        }

        let rows = sqlx::query_as::<_, MoviePath>("SELECT Id, Path FROM Movies")
            .fetch_all(&self.pool)
            .await?;

        let map = rows.into_iter().map(|row| (row.id, row.path)).collect();

        Ok(map)
    }

    async fn all_movie_tmdb_ids(&self) -> Result<Vec<i32>> {
        #[derive(sqlx::FromRow)]
        struct TmdbIdRow {
            #[sqlx(rename = "TmdbId")]
            tmdb_id: i32,
        }

        let rows = sqlx::query_as::<_, TmdbIdRow>(
            "SELECT mm.TmdbId FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id",
        )
        .fetch_all(&self.pool)
        .await?;

        let ids = rows.into_iter().map(|row| row.tmdb_id).collect();

        Ok(ids)
    }

    async fn movies_between_dates(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        include_unmonitored: bool,
    ) -> Result<Vec<Movie>> {
        let mut movies = if include_unmonitored {
            sqlx::query_as::<_, Movie>("SELECT * FROM Movies WHERE Added >= $1 AND Added <= $2")
                .bind(start)
                .bind(end)
                .fetch_all(&self.pool)
                .await?
        } else {
            sqlx::query_as::<_, Movie>(
                "SELECT * FROM Movies WHERE Added >= $1 AND Added <= $2 AND Monitored = true",
            )
            .bind(start)
            .bind(end)
            .fetch_all(&self.pool)
            .await?
        };

        self.load_metadata(&mut movies).await?;

        Ok(movies)
    }
}

// MovieMetadataRepository implementations

use crate::repository::traits::MovieMetadataRepository;

#[async_trait]
impl MovieMetadataRepository for SqlxMovieRepository {
    async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<MovieMetadata>> {
        let metadata_result =
            sqlx::query_as::<_, MovieMetadata>("SELECT * FROM MovieMetadata WHERE TmdbId = ?")
                .bind(tmdb_id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(metadata_result)
    }

    async fn insert(&self, metadata: &MovieMetadata) -> Result<MovieMetadata> {
        let result = sqlx::query(
            r"
            INSERT INTO MovieMetadata (
                TmdbId, ImdbId, Title, OriginalTitle, CleanTitle, SortTitle,
                Year, Status, Overview, Images, Genres, Ratings, Runtime,
                InCinemas, PhysicalRelease, DigitalRelease, Certification,
                Website, YouTubeTrailerId, Studio, Popularity,
                CollectionTmdbId, CollectionTitle
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ",
        )
        .bind(metadata.tmdb_id)
        .bind(&metadata.imdb_id)
        .bind(&metadata.title)
        .bind(&metadata.original_title)
        .bind(&metadata.clean_title)
        .bind(&metadata.sort_title)
        .bind(metadata.year)
        .bind(metadata.status)
        .bind(&metadata.overview)
        .bind(&metadata.images)
        .bind(&metadata.genres)
        .bind(&metadata.ratings)
        .bind(metadata.runtime)
        .bind(metadata.in_cinemas)
        .bind(metadata.physical_release)
        .bind(metadata.digital_release)
        .bind(&metadata.certification)
        .bind(&metadata.website)
        .bind(&metadata.youtube_trailer_id)
        .bind(&metadata.studio)
        .bind(metadata.popularity)
        .bind(metadata.collection_tmdb_id)
        .bind(&metadata.collection_title)
        .execute(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                db_err.code().map_or_else(
                    || Error::Database(db_err.to_string()),
                    |code| {
                        if code == "2067" || code == "1555" {
                            // SQLite UNIQUE constraint violation
                            Error::Validation("Movie metadata already exists".to_string())
                        } else {
                            Error::Database(db_err.to_string())
                        }
                    },
                )
            }
            _ => Error::Database(e.to_string()),
        })?;

        let id = i32::try_from(result.last_insert_rowid())
            .map_err(|e| Error::Internal(format!("ID overflow: {e}")))?;

        // Fetch and return the inserted metadata
        sqlx::query_as::<_, MovieMetadata>("SELECT * FROM MovieMetadata WHERE Id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }
}

#[async_trait]
impl MovieMetadataRepository for PostgresMovieRepository {
    async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<MovieMetadata>> {
        let metadata_result =
            sqlx::query_as::<_, MovieMetadata>("SELECT * FROM MovieMetadata WHERE TmdbId = $1")
                .bind(tmdb_id)
                .fetch_optional(&self.pool)
                .await?;

        Ok(metadata_result)
    }

    async fn insert(&self, metadata: &MovieMetadata) -> Result<MovieMetadata> {
        let result = sqlx::query(
            r"
            INSERT INTO MovieMetadata (
                TmdbId, ImdbId, Title, OriginalTitle, CleanTitle, SortTitle,
                Year, Status, Overview, Images, Genres, Ratings, Runtime,
                InCinemas, PhysicalRelease, DigitalRelease, Certification,
                Website, YouTubeTrailerId, Studio, Popularity,
                CollectionTmdbId, CollectionTitle
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23)
            RETURNING Id
            ",
        )
        .bind(metadata.tmdb_id)
        .bind(&metadata.imdb_id)
        .bind(&metadata.title)
        .bind(&metadata.original_title)
        .bind(&metadata.clean_title)
        .bind(&metadata.sort_title)
        .bind(metadata.year)
        .bind(metadata.status)
        .bind(&metadata.overview)
        .bind(&metadata.images)
        .bind(&metadata.genres)
        .bind(&metadata.ratings)
        .bind(metadata.runtime)
        .bind(metadata.in_cinemas)
        .bind(metadata.physical_release)
        .bind(metadata.digital_release)
        .bind(&metadata.certification)
        .bind(&metadata.website)
        .bind(&metadata.youtube_trailer_id)
        .bind(&metadata.studio)
        .bind(metadata.popularity)
        .bind(metadata.collection_tmdb_id)
        .bind(&metadata.collection_title)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::Database(db_err) => {
                db_err.code().map_or_else(
                    || Error::Database(db_err.to_string()),
                    |code| {
                        if code == "23505" {
                            // PostgreSQL UNIQUE constraint violation
                            Error::Validation("Movie metadata already exists".to_string())
                        } else {
                            Error::Database(db_err.to_string())
                        }
                    },
                )
            }
            _ => Error::Database(e.to_string()),
        })?;

        let id: i32 = result.try_get("Id")?;

        // Fetch and return the inserted metadata
        sqlx::query_as::<_, MovieMetadata>("SELECT * FROM MovieMetadata WHERE Id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::Database(e.to_string()))
    }
}
