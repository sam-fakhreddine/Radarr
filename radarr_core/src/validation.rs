//! Validation helpers for movie operations

use crate::domain::movie::Movie;
use crate::error::{Error, Result};

/// Validates a movie's path
///
/// # Arguments
///
/// * `movie` - The movie to validate
///
/// # Returns
///
/// Returns `Ok(())` if the path is valid
///
/// # Errors
///
/// Returns `Error::Validation` if:
/// - Both path and `root_folder_path` are empty
/// - Path contains invalid characters
pub fn validate_movie_path(movie: &Movie) -> Result<()> {
    // Check if both path and root_folder_path are empty
    if movie.path.is_empty() && movie.root_folder_path.is_none() {
        return Err(Error::Validation(
            "Either path or rootFolderPath must be provided".to_string(),
        ));
    }

    // If path is provided, validate it's not just whitespace
    if !movie.path.is_empty() && movie.path.trim().is_empty() {
        return Err(Error::Validation(
            "Path cannot be empty or contain only whitespace".to_string(),
        ));
    }

    // Basic path validation - check for null bytes which are invalid in paths
    if movie.path.contains('\0') {
        return Err(Error::Validation(
            "Path contains invalid null character".to_string(),
        ));
    }

    Ok(())
}

/// Validates that a quality profile ID is valid (greater than 0)
///
/// # Arguments
///
/// * `quality_profile_id` - The quality profile ID to validate
///
/// # Returns
///
/// Returns `Ok(())` if the quality profile ID is valid
///
/// # Errors
///
/// Returns `Error::Validation` if the quality profile ID is invalid (≤ 0)
pub fn validate_quality_profile_id(quality_profile_id: i32) -> Result<()> {
    if quality_profile_id <= 0 {
        return Err(Error::Validation(format!(
            "Invalid quality profile ID: {quality_profile_id}. Quality profile ID must be greater than 0"
        )));
    }

    Ok(())
}

/// Validates a movie before creation or update
///
/// # Arguments
///
/// * `movie` - The movie to validate
///
/// # Returns
///
/// Returns `Ok(())` if the movie is valid
///
/// # Errors
///
/// Returns `Error::Validation` if any validation fails
pub fn validate_movie(movie: &Movie) -> Result<()> {
    // Validate path
    validate_movie_path(movie)?;

    // Validate quality profile ID
    validate_quality_profile_id(movie.quality_profile_id)?;

    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::domain::types::MovieStatusType;
    use chrono::Utc;

    fn create_valid_movie() -> Movie {
        Movie {
            id: 0,
            movie_metadata_id: 1,
            monitored: true,
            minimum_availability: MovieStatusType::Released,
            quality_profile_id: 1,
            path: "/movies/Test Movie (2024)".to_string(),
            root_folder_path: Some("/movies".to_string()),
            added: Utc::now(),
            tags: serde_json::json!([]),
            add_options: None,
            last_search_time: None,
            movie_file_id: 0,
            movie_metadata: None,
        }
    }

    #[test]
    fn test_validate_movie_path_valid() {
        let movie = create_valid_movie();
        let result = validate_movie_path(&movie);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_movie_path_empty_both() {
        let mut movie = create_valid_movie();
        movie.path = String::new();
        movie.root_folder_path = None;

        let result = validate_movie_path(&movie);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }

    #[test]
    fn test_validate_movie_path_whitespace_only() {
        let mut movie = create_valid_movie();
        movie.path = "   ".to_string();

        let result = validate_movie_path(&movie);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }

    #[test]
    fn test_validate_movie_path_null_character() {
        let mut movie = create_valid_movie();
        movie.path = "/movies/test\0movie".to_string();

        let result = validate_movie_path(&movie);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }

    #[test]
    fn test_validate_movie_path_with_root_folder() {
        let mut movie = create_valid_movie();
        movie.path = String::new();
        movie.root_folder_path = Some("/movies".to_string());

        let result = validate_movie_path(&movie);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_quality_profile_id_valid() {
        let result = validate_quality_profile_id(1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_quality_profile_id_zero() {
        let result = validate_quality_profile_id(0);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }

    #[test]
    fn test_validate_quality_profile_id_negative() {
        let result = validate_quality_profile_id(-1);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }

    #[test]
    fn test_validate_movie_valid() {
        let movie = create_valid_movie();
        let result = validate_movie(&movie);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_movie_invalid_path() {
        let mut movie = create_valid_movie();
        movie.path = String::new();
        movie.root_folder_path = None;

        let result = validate_movie(&movie);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }

    #[test]
    fn test_validate_movie_invalid_quality_profile() {
        let mut movie = create_valid_movie();
        movie.quality_profile_id = 0;

        let result = validate_movie(&movie);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }
}
