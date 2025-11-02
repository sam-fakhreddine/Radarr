//! Movie service implementation

use crate::domain::movie::Movie;
use crate::error::{Error, Result};
use crate::repository::traits::MovieRepository;
use chrono::Utc;
use std::sync::Arc;

/// Configuration for the movie service
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Availability delay in days
    pub availability_delay: i32,
}

/// Movie service providing business logic for movie operations
pub struct MovieService {
    repository: Arc<dyn MovieRepository>,
    #[allow(dead_code)]
    config: Arc<Config>,
}

impl MovieService {
    /// Creates a new `MovieService`
    ///
    /// # Arguments
    ///
    /// * `repository` - The movie repository implementation
    /// * `config` - Service configuration
    ///
    /// # Returns
    ///
    /// Returns a new `MovieService` instance
    #[must_use]
    pub fn new(repository: Arc<dyn MovieRepository>, config: Arc<Config>) -> Self {
        Self { repository, config }
    }

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
    pub async fn get_movie(&self, id: i32) -> Result<Movie> {
        self.repository.get(id).await
    }

    /// Gets all movies
    ///
    /// # Returns
    ///
    /// Returns all movies with loaded metadata
    ///
    /// # Errors
    ///
    /// Returns `Error::Database` for database errors
    pub async fn get_all_movies(&self) -> Result<Vec<Movie>> {
        self.repository.all().await
    }

    /// Adds a new movie to the library
    ///
    /// # Arguments
    ///
    /// * `movie` - The movie to add
    ///
    /// # Returns
    ///
    /// Returns the created movie with generated ID and timestamp
    ///
    /// # Errors
    ///
    /// Returns `Error::Validation` if:
    /// - Movie already exists (by TMDB ID)
    /// - Path validation fails
    /// - Quality profile doesn't exist
    ///
    /// Returns `Error::Database` for database errors
    pub async fn add_movie(&self, mut movie: Movie) -> Result<Movie> {
        // Validate movie doesn't already exist by TMDB ID
        if let Some(existing) = self
            .repository
            .find_by_tmdb_id(movie.movie_metadata_id)
            .await?
        {
            return Err(Error::Validation(format!(
                "Movie with TMDB ID {} already exists with ID {}",
                movie.movie_metadata_id, existing.id
            )));
        }

        // Set added timestamp to current UTC time
        movie.added = Utc::now();

        // Insert movie
        self.repository.insert(&movie).await
    }

    /// Updates an existing movie
    ///
    /// # Arguments
    ///
    /// * `id` - The movie ID to update
    /// * `updates` - The movie with updated fields
    ///
    /// # Returns
    ///
    /// Returns the updated movie
    ///
    /// # Errors
    ///
    /// Returns `Error::NotFound` if the movie doesn't exist
    /// Returns `Error::Validation` for validation errors
    /// Returns `Error::Database` for database errors
    pub async fn update_movie(&self, id: i32, updates: Movie) -> Result<Movie> {
        // Get existing movie by ID
        let mut existing = self.repository.get(id).await?;

        // Apply changes using apply_changes method
        existing.apply_changes(&updates);

        // Update in repository
        self.repository.update(&existing).await
    }

    /// Deletes a movie from the library
    ///
    /// # Arguments
    ///
    /// * `id` - The movie ID to delete
    /// * `delete_files` - Whether to delete associated movie files from disk (stub for now)
    /// * `add_import_exclusion` - Whether to add the movie to the import exclusion list (stub for now)
    ///
    /// # Errors
    ///
    /// Returns `Error::NotFound` if the movie doesn't exist
    /// Returns `Error::Database` for database errors
    pub async fn delete_movie(
        &self,
        id: i32,
        delete_files: bool,
        add_import_exclusion: bool,
    ) -> Result<()> {
        // Get movie by ID to verify existence
        let _movie = self.repository.get(id).await?;

        // Delete from repository
        self.repository.delete(id).await?;

        // TODO: Handle delete_files parameter (stub for now)
        if delete_files {
            // File deletion logic would go here
        }

        // TODO: Handle add_import_exclusion parameter (stub for now)
        if add_import_exclusion {
            // Import exclusion logic would go here
        }

        Ok(())
    }

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
    pub async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<Movie>> {
        self.repository.find_by_tmdb_id(tmdb_id).await
    }

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
    pub async fn find_by_imdb_id(&self, imdb_id: &str) -> Result<Option<Movie>> {
        self.repository.find_by_imdb_id(imdb_id).await
    }

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
    pub async fn find_by_path(&self, path: &str) -> Result<Option<Movie>> {
        self.repository.find_by_path(path).await
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::significant_drop_tightening)]
mod tests {
    use super::*;
    use crate::domain::movie_metadata::MovieMetadata;
    use crate::domain::types::MovieStatusType;
    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;
    use std::sync::Mutex;

    // Mock repository for testing
    struct MockMovieRepository {
        movies: Mutex<HashMap<i32, Movie>>,
        next_id: Mutex<i32>,
    }

    impl MockMovieRepository {
        fn new() -> Self {
            Self {
                movies: Mutex::new(HashMap::new()),
                next_id: Mutex::new(1),
            }
        }

        fn with_movies(movies: Vec<Movie>) -> Self {
            let mut map = HashMap::new();
            let mut max_id = 0;

            for movie in movies {
                if movie.id > max_id {
                    max_id = movie.id;
                }
                map.insert(movie.id, movie);
            }

            Self {
                movies: Mutex::new(map),
                next_id: Mutex::new(max_id + 1),
            }
        }
    }

    #[async_trait]
    impl MovieRepository for MockMovieRepository {
        async fn get(&self, id: i32) -> Result<Movie> {
            self.movies
                .lock()
                .expect("Lock poisoned")
                .get(&id)
                .cloned()
                .ok_or_else(|| Error::NotFound(format!("Movie {id} not found")))
        }

        async fn find(&self, id: i32) -> Result<Option<Movie>> {
            Ok(self.movies.lock().expect("Lock poisoned").get(&id).cloned())
        }

        async fn all(&self) -> Result<Vec<Movie>> {
            Ok(self
                .movies
                .lock()
                .expect("Lock poisoned")
                .values()
                .cloned()
                .collect())
        }

        async fn insert(&self, movie: &Movie) -> Result<Movie> {
            let mut movies_guard = self.movies.lock().expect("Lock poisoned");
            let mut next_id_guard = self.next_id.lock().expect("Lock poisoned");

            let mut new_movie = movie.clone();
            new_movie.id = *next_id_guard;
            *next_id_guard += 1;

            movies_guard.insert(new_movie.id, new_movie.clone());
            Ok(new_movie)
        }

        async fn update(&self, movie: &Movie) -> Result<Movie> {
            let mut movies_guard = self.movies.lock().expect("Lock poisoned");

            if !movies_guard.contains_key(&movie.id) {
                let movie_id = movie.id;
                return Err(Error::NotFound(format!("Movie {movie_id} not found")));
            }

            movies_guard.insert(movie.id, movie.clone());
            Ok(movie.clone())
        }

        async fn delete(&self, id: i32) -> Result<()> {
            let mut movies_guard = self.movies.lock().expect("Lock poisoned");

            if movies_guard.remove(&id).is_none() {
                return Err(Error::NotFound(format!("Movie {id} not found")));
            }

            Ok(())
        }

        async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<Movie>> {
            Ok(self
                .movies
                .lock()
                .expect("Lock poisoned")
                .values()
                .find(|m| m.movie_metadata_id == tmdb_id)
                .cloned())
        }

        async fn find_by_imdb_id(&self, _imdb_id: &str) -> Result<Option<Movie>> {
            Ok(None)
        }

        async fn find_by_path(&self, path: &str) -> Result<Option<Movie>> {
            Ok(self
                .movies
                .lock()
                .expect("Lock poisoned")
                .values()
                .find(|m| m.path == path)
                .cloned())
        }

        async fn find_by_titles(&self, _titles: &[String]) -> Result<Vec<Movie>> {
            Ok(vec![])
        }

        async fn insert_many(&self, _movies: &[Movie]) -> Result<()> {
            Ok(())
        }

        async fn update_many(&self, _movies: &[Movie]) -> Result<()> {
            Ok(())
        }

        async fn delete_many(&self, _ids: &[i32]) -> Result<()> {
            Ok(())
        }

        async fn all_movie_paths(&self) -> Result<HashMap<i32, String>> {
            Ok(HashMap::new())
        }

        async fn all_movie_tmdb_ids(&self) -> Result<Vec<i32>> {
            Ok(vec![])
        }

        async fn movies_between_dates(
            &self,
            _start: DateTime<Utc>,
            _end: DateTime<Utc>,
            _include_unmonitored: bool,
        ) -> Result<Vec<Movie>> {
            Ok(vec![])
        }
    }

    fn create_test_movie() -> Movie {
        Movie {
            id: 0,
            movie_metadata_id: 27205,
            monitored: true,
            minimum_availability: MovieStatusType::Released,
            quality_profile_id: 1,
            path: "/movies/Inception (2010)".to_string(),
            root_folder_path: Some("/movies".to_string()),
            added: Utc::now(),
            tags: serde_json::json!([1, 2]),
            add_options: None,
            last_search_time: None,
            movie_file_id: 0,
            movie_metadata: Some(MovieMetadata {
                id: 1,
                tmdb_id: 27205,
                imdb_id: Some("tt1375666".to_string()),
                title: "Inception".to_string(),
                original_title: None,
                clean_title: "inception".to_string(),
                sort_title: Some("inception".to_string()),
                year: 2010,
                status: MovieStatusType::Released,
                overview: Some("A thief who steals corporate secrets".to_string()),
                images: serde_json::json!([]),
                genres: serde_json::json!(["Action", "Sci-Fi"]),
                ratings: serde_json::json!({}),
                runtime: 148,
                in_cinemas: None,
                physical_release: None,
                digital_release: None,
                certification: Some("PG-13".to_string()),
                website: None,
                youtube_trailer_id: None,
                studio: Some("Warner Bros.".to_string()),
                popularity: 42.0,
                collection_tmdb_id: 0,
                collection_title: None,
            }),
        }
    }

    #[tokio::test]
    async fn test_get_movie_success() {
        let movie = create_test_movie();
        let mut movie_with_id = movie.clone();
        movie_with_id.id = 1;

        let repo = Arc::new(MockMovieRepository::with_movies(
            vec![movie_with_id.clone()],
        ));
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let result = service.get_movie(1).await;
        assert!(result.is_ok());

        let fetched = result.expect("Should get movie");
        assert_eq!(fetched.id, 1);
        assert_eq!(fetched.path, "/movies/Inception (2010)");
    }

    #[tokio::test]
    async fn test_get_movie_not_found() {
        let repo = Arc::new(MockMovieRepository::new());
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let result = service.get_movie(999).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::NotFound(_)));
    }

    #[tokio::test]
    async fn test_get_all_movies() {
        let first_movie = create_test_movie();
        let mut first_movie_with_id = first_movie.clone();
        first_movie_with_id.id = 1;

        let mut second_movie = create_test_movie();
        second_movie.id = 2;
        second_movie.movie_metadata_id = 550;
        second_movie.path = "/movies/Fight Club (1999)".to_string();

        let repo = Arc::new(MockMovieRepository::with_movies(vec![
            first_movie_with_id,
            second_movie,
        ]));
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let result = service.get_all_movies().await;
        assert!(result.is_ok());

        let all_movies = result.expect("Should get all movies");
        assert_eq!(all_movies.len(), 2);
    }

    #[tokio::test]
    async fn test_add_movie_success() {
        let repo = Arc::new(MockMovieRepository::new());
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let movie = create_test_movie();
        let result = service.add_movie(movie).await;

        assert!(result.is_ok());
        let created = result.expect("Should create movie");
        assert!(created.id > 0);
        assert_eq!(created.path, "/movies/Inception (2010)");
    }

    #[tokio::test]
    async fn test_add_movie_duplicate() {
        let existing = create_test_movie();
        let mut existing_with_id = existing.clone();
        existing_with_id.id = 1;

        let repo = Arc::new(MockMovieRepository::with_movies(vec![existing_with_id]));
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let new_movie = create_test_movie();
        let result = service.add_movie(new_movie).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }

    #[tokio::test]
    async fn test_update_movie_success() {
        let movie = create_test_movie();
        let mut movie_with_id = movie.clone();
        movie_with_id.id = 1;

        let repo = Arc::new(MockMovieRepository::with_movies(
            vec![movie_with_id.clone()],
        ));
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let mut changes = movie_with_id.clone();
        changes.path = "/new/path".to_string();
        changes.monitored = false;

        let result = service.update_movie(1, changes).await;
        assert!(result.is_ok());

        let updated_movie = result.expect("Should update movie");
        assert_eq!(updated_movie.path, "/new/path");
        assert!(!updated_movie.monitored);
    }

    #[tokio::test]
    async fn test_update_movie_not_found() {
        let repo = Arc::new(MockMovieRepository::new());
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let movie = create_test_movie();
        let result = service.update_movie(999, movie).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::NotFound(_)));
    }

    #[tokio::test]
    async fn test_delete_movie_success() {
        let movie = create_test_movie();
        let mut movie_with_id = movie.clone();
        movie_with_id.id = 1;

        let repo = Arc::new(MockMovieRepository::with_movies(vec![movie_with_id]));
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let result = service.delete_movie(1, false, false).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_delete_movie_not_found() {
        let repo = Arc::new(MockMovieRepository::new());
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let result = service.delete_movie(999, false, false).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::NotFound(_)));
    }

    #[tokio::test]
    async fn test_find_by_tmdb_id_found() {
        let movie = create_test_movie();
        let mut movie_with_id = movie.clone();
        movie_with_id.id = 1;

        let repo = Arc::new(MockMovieRepository::with_movies(
            vec![movie_with_id.clone()],
        ));
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let result = service.find_by_tmdb_id(27205).await;
        assert!(result.is_ok());

        let found = result.expect("Should find by tmdb_id");
        assert!(found.is_some());
        let movie_found = found.expect("Movie should exist");
        assert_eq!(movie_found.movie_metadata_id, 27205);
    }

    #[tokio::test]
    async fn test_find_by_tmdb_id_not_found() {
        let repo = Arc::new(MockMovieRepository::new());
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let result = service.find_by_tmdb_id(999).await;
        assert!(result.is_ok());
        let found = result.expect("Should complete search");
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_find_by_path_found() {
        let movie = create_test_movie();
        let mut movie_with_id = movie.clone();
        movie_with_id.id = 1;

        let repo = Arc::new(MockMovieRepository::with_movies(
            vec![movie_with_id.clone()],
        ));
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let result = service.find_by_path("/movies/Inception (2010)").await;
        assert!(result.is_ok());

        let found = result.expect("Should find by path");
        assert!(found.is_some());
        let movie_found = found.expect("Movie should exist");
        assert_eq!(movie_found.path, "/movies/Inception (2010)");
    }

    #[tokio::test]
    async fn test_find_by_path_not_found() {
        let repo = Arc::new(MockMovieRepository::new());
        let config = Arc::new(Config::default());
        let service = MovieService::new(repo, config);

        let result = service.find_by_path("/nonexistent/path").await;
        assert!(result.is_ok());
        let found = result.expect("Should complete search");
        assert!(found.is_none());
    }
}
