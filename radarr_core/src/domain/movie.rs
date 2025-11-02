//! Movie entity

use crate::domain::movie_metadata::MovieMetadata;
use crate::domain::types::MovieStatusType;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Movie entity representing a movie in the library
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Movie {
    /// Unique identifier
    #[sqlx(rename = "Id")]
    pub id: i32,

    /// Foreign key to `MovieMetadata`
    #[serde(rename = "movieMetadataId")]
    #[sqlx(rename = "MovieMetadataId")]
    pub movie_metadata_id: i32,

    /// Whether the movie is being monitored for downloads
    #[sqlx(rename = "Monitored")]
    pub monitored: bool,

    /// Minimum availability required before searching
    #[serde(rename = "minimumAvailability")]
    #[sqlx(rename = "MinimumAvailability")]
    pub minimum_availability: MovieStatusType,

    /// Quality profile ID for this movie
    #[serde(rename = "qualityProfileId")]
    #[sqlx(rename = "QualityProfileId")]
    pub quality_profile_id: i32,

    /// File system path where movie is/will be stored
    #[sqlx(rename = "Path")]
    pub path: String,

    /// Root folder path (used during creation)
    #[serde(rename = "rootFolderPath")]
    #[sqlx(rename = "RootFolderPath")]
    pub root_folder_path: Option<String>,

    /// Timestamp when movie was added to library
    #[sqlx(rename = "Added")]
    pub added: DateTime<Utc>,

    /// Tags associated with this movie
    #[sqlx(rename = "Tags")]
    pub tags: serde_json::Value,

    /// Additional options used when adding the movie
    #[serde(rename = "addOptions")]
    #[sqlx(rename = "AddOptions")]
    pub add_options: Option<serde_json::Value>,

    /// Last time a search was performed for this movie
    #[serde(rename = "lastSearchTime")]
    #[sqlx(rename = "LastSearchTime")]
    pub last_search_time: Option<DateTime<Utc>>,

    /// Foreign key to `MovieFile` (0 if no file)
    #[serde(rename = "movieFileId")]
    #[sqlx(rename = "MovieFileId")]
    pub movie_file_id: i32,

    /// Lazy-loaded movie metadata (not stored in Movies table)
    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movie_metadata: Option<MovieMetadata>,
}

impl Movie {
    /// Checks if the movie has an associated file
    #[must_use]
    pub const fn has_file(&self) -> bool {
        self.movie_file_id > 0
    }

    /// Applies changes from another `Movie` instance
    ///
    /// This is used during updates to apply only the fields that should be modifiable
    pub fn apply_changes(&mut self, other: &Self) {
        self.path.clone_from(&other.path);
        self.quality_profile_id = other.quality_profile_id;
        self.monitored = other.monitored;
        self.minimum_availability = other.minimum_availability;
        self.root_folder_path.clone_from(&other.root_folder_path);
        self.tags = other.tags.clone();
    }

    /// Parses the tags JSON into a `Vec<i32>`
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON cannot be deserialized into a `Vec<i32>`
    pub fn get_tags(&self) -> Result<Vec<i32>, serde_json::Error> {
        serde_json::from_value(self.tags.clone())
    }

    /// Determines if the movie is available for download based on minimum availability and release dates
    ///
    /// # Arguments
    ///
    /// * `availability_delay` - Number of days to add to the availability date
    ///
    /// # Returns
    ///
    /// Returns `true` if the movie is available, `false` otherwise
    #[must_use]
    pub fn is_available(&self, availability_delay: i32) -> bool {
        let now = Utc::now();

        // TBA and Announced are always available (no date restrictions)
        match self.minimum_availability {
            MovieStatusType::Tba | MovieStatusType::Announced => return true,
            _ => {}
        }

        // Get the metadata to check release dates
        let Some(metadata) = &self.movie_metadata else {
            return false; // Can't determine availability without metadata
        };

        // Determine the availability date based on minimum availability setting
        let availability_date = match self.minimum_availability {
            MovieStatusType::InCinemas => {
                // Use InCinemas date
                metadata.in_cinemas
            }
            MovieStatusType::Released => {
                // Use the earlier of PhysicalRelease or DigitalRelease
                match (metadata.physical_release, metadata.digital_release) {
                    (Some(physical), Some(digital)) => Some(physical.min(digital)),
                    (Some(physical), None) => Some(physical),
                    (None, Some(digital)) => Some(digital),
                    (None, None) => {
                        // If no release dates, fall back to InCinemas + 90 days
                        metadata.in_cinemas.map(|ic| ic + Duration::days(90))
                    }
                }
            }
            _ => return true, // Already handled TBA/Announced above
        };

        // If we have an availability date, check if it's in the past (with delay applied)
        availability_date.map_or_else(
            || {
                // No availability date means we can't determine availability
                // Default to InCinemas + 90 days if InCinemas exists
                metadata.in_cinemas.is_some_and(|in_cinemas| {
                    let default_date =
                        in_cinemas + Duration::days(90 + i64::from(availability_delay));
                    now >= default_date
                })
            },
            |date| {
                let date_with_delay = date + Duration::days(i64::from(availability_delay));
                now >= date_with_delay
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_movie() -> Movie {
        Movie {
            id: 1,
            movie_metadata_id: 1,
            monitored: true,
            minimum_availability: MovieStatusType::Released,
            quality_profile_id: 1,
            path: "/movies/Inception (2010)".to_string(),
            root_folder_path: Some("/movies".to_string()),
            added: Utc::now(),
            tags: serde_json::json!([1, 2, 3]),
            add_options: None,
            last_search_time: None,
            movie_file_id: 0,
            movie_metadata: None,
        }
    }

    #[test]
    fn test_movie_creation() {
        let movie = create_test_movie();
        assert_eq!(movie.id, 1);
        assert_eq!(movie.movie_metadata_id, 1);
        assert!(movie.monitored);
        assert_eq!(movie.quality_profile_id, 1);
    }

    #[test]
    fn test_has_file_with_no_file() {
        let movie = create_test_movie();
        assert!(!movie.has_file());
    }

    #[test]
    fn test_has_file_with_file() {
        let mut movie = create_test_movie();
        movie.movie_file_id = 5;
        assert!(movie.has_file());
    }

    #[test]
    fn test_apply_changes() {
        let mut movie = create_test_movie();
        let mut changes = create_test_movie();

        changes.path = "/new/path".to_string();
        changes.quality_profile_id = 2;
        changes.monitored = false;
        changes.minimum_availability = MovieStatusType::InCinemas;
        changes.tags = serde_json::json!([4, 5]);

        movie.apply_changes(&changes);

        assert_eq!(movie.path, "/new/path");
        assert_eq!(movie.quality_profile_id, 2);
        assert!(!movie.monitored);
        assert_eq!(movie.minimum_availability, MovieStatusType::InCinemas);
        assert_eq!(movie.tags, serde_json::json!([4, 5]));

        // These should not change
        assert_eq!(movie.id, 1);
        assert_eq!(movie.movie_metadata_id, 1);
    }

    #[test]
    fn test_get_tags() -> Result<(), Box<dyn std::error::Error>> {
        let movie = create_test_movie();
        let tags = movie.get_tags()?;

        assert_eq!(tags.len(), 3);
        assert_eq!(tags, vec![1, 2, 3]);

        Ok(())
    }

    #[test]
    fn test_movie_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let movie = create_test_movie();
        let json = serde_json::to_value(&movie)?;

        assert_eq!(json["id"], 1);
        assert_eq!(json["movieMetadataId"], 1);
        assert_eq!(json["monitored"], true);
        assert_eq!(json["qualityProfileId"], 1);
        assert_eq!(json["path"], "/movies/Inception (2010)");

        Ok(())
    }

    #[test]
    fn test_is_available_tba() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::Tba;
        movie.movie_metadata = Some(MovieMetadata {
            id: 1,
            tmdb_id: 1,
            imdb_id: None,
            title: "Test".to_string(),
            original_title: None,
            clean_title: "test".to_string(),
            sort_title: None,
            year: 2024,
            status: MovieStatusType::Tba,
            overview: None,
            images: serde_json::json!([]),
            genres: serde_json::json!([]),
            ratings: serde_json::json!({}),
            runtime: 0,
            in_cinemas: None,
            physical_release: None,
            digital_release: None,
            certification: None,
            website: None,
            youtube_trailer_id: None,
            studio: None,
            popularity: 0.0,
            collection_tmdb_id: 0,
            collection_title: None,
        });

        assert!(movie.is_available(0));
    }

    #[test]
    fn test_is_available_announced() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::Announced;
        movie.movie_metadata = Some(MovieMetadata {
            id: 1,
            tmdb_id: 1,
            imdb_id: None,
            title: "Test".to_string(),
            original_title: None,
            clean_title: "test".to_string(),
            sort_title: None,
            year: 2024,
            status: MovieStatusType::Announced,
            overview: None,
            images: serde_json::json!([]),
            genres: serde_json::json!([]),
            ratings: serde_json::json!({}),
            runtime: 0,
            in_cinemas: None,
            physical_release: None,
            digital_release: None,
            certification: None,
            website: None,
            youtube_trailer_id: None,
            studio: None,
            popularity: 0.0,
            collection_tmdb_id: 0,
            collection_title: None,
        });

        assert!(movie.is_available(0));
    }

    #[test]
    fn test_is_available_in_cinemas_past() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::InCinemas;
        movie.movie_metadata = Some(MovieMetadata {
            id: 1,
            tmdb_id: 1,
            imdb_id: None,
            title: "Test".to_string(),
            original_title: None,
            clean_title: "test".to_string(),
            sort_title: None,
            year: 2024,
            status: MovieStatusType::InCinemas,
            overview: None,
            images: serde_json::json!([]),
            genres: serde_json::json!([]),
            ratings: serde_json::json!({}),
            runtime: 0,
            in_cinemas: Some(Utc::now() - Duration::days(30)),
            physical_release: None,
            digital_release: None,
            certification: None,
            website: None,
            youtube_trailer_id: None,
            studio: None,
            popularity: 0.0,
            collection_tmdb_id: 0,
            collection_title: None,
        });

        assert!(movie.is_available(0));
    }

    #[test]
    fn test_is_available_in_cinemas_future() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::InCinemas;
        movie.movie_metadata = Some(MovieMetadata {
            id: 1,
            tmdb_id: 1,
            imdb_id: None,
            title: "Test".to_string(),
            original_title: None,
            clean_title: "test".to_string(),
            sort_title: None,
            year: 2024,
            status: MovieStatusType::InCinemas,
            overview: None,
            images: serde_json::json!([]),
            genres: serde_json::json!([]),
            ratings: serde_json::json!({}),
            runtime: 0,
            in_cinemas: Some(Utc::now() + Duration::days(30)),
            physical_release: None,
            digital_release: None,
            certification: None,
            website: None,
            youtube_trailer_id: None,
            studio: None,
            popularity: 0.0,
            collection_tmdb_id: 0,
            collection_title: None,
        });

        assert!(!movie.is_available(0));
    }

    #[test]
    fn test_is_available_released_with_physical() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::Released;
        movie.movie_metadata = Some(MovieMetadata {
            id: 1,
            tmdb_id: 1,
            imdb_id: None,
            title: "Test".to_string(),
            original_title: None,
            clean_title: "test".to_string(),
            sort_title: None,
            year: 2024,
            status: MovieStatusType::Released,
            overview: None,
            images: serde_json::json!([]),
            genres: serde_json::json!([]),
            ratings: serde_json::json!({}),
            runtime: 0,
            in_cinemas: Some(Utc::now() - Duration::days(120)),
            physical_release: Some(Utc::now() - Duration::days(30)),
            digital_release: None,
            certification: None,
            website: None,
            youtube_trailer_id: None,
            studio: None,
            popularity: 0.0,
            collection_tmdb_id: 0,
            collection_title: None,
        });

        assert!(movie.is_available(0));
    }

    #[test]
    fn test_is_available_released_with_digital() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::Released;
        movie.movie_metadata = Some(MovieMetadata {
            id: 1,
            tmdb_id: 1,
            imdb_id: None,
            title: "Test".to_string(),
            original_title: None,
            clean_title: "test".to_string(),
            sort_title: None,
            year: 2024,
            status: MovieStatusType::Released,
            overview: None,
            images: serde_json::json!([]),
            genres: serde_json::json!([]),
            ratings: serde_json::json!({}),
            runtime: 0,
            in_cinemas: Some(Utc::now() - Duration::days(120)),
            physical_release: None,
            digital_release: Some(Utc::now() - Duration::days(30)),
            certification: None,
            website: None,
            youtube_trailer_id: None,
            studio: None,
            popularity: 0.0,
            collection_tmdb_id: 0,
            collection_title: None,
        });

        assert!(movie.is_available(0));
    }

    #[test]
    fn test_is_available_released_earliest_date() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::Released;
        movie.movie_metadata = Some(MovieMetadata {
            id: 1,
            tmdb_id: 1,
            imdb_id: None,
            title: "Test".to_string(),
            original_title: None,
            clean_title: "test".to_string(),
            sort_title: None,
            year: 2024,
            status: MovieStatusType::Released,
            overview: None,
            images: serde_json::json!([]),
            genres: serde_json::json!([]),
            ratings: serde_json::json!({}),
            runtime: 0,
            in_cinemas: Some(Utc::now() - Duration::days(120)),
            physical_release: Some(Utc::now() - Duration::days(30)),
            digital_release: Some(Utc::now() - Duration::days(60)),
            certification: None,
            website: None,
            youtube_trailer_id: None,
            studio: None,
            popularity: 0.0,
            collection_tmdb_id: 0,
            collection_title: None,
        });

        // Should use digital release (60 days ago) as it's earlier than physical (30 days ago)
        assert!(movie.is_available(0));
    }

    #[test]
    fn test_is_available_with_delay() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::InCinemas;
        movie.movie_metadata = Some(MovieMetadata {
            id: 1,
            tmdb_id: 1,
            imdb_id: None,
            title: "Test".to_string(),
            original_title: None,
            clean_title: "test".to_string(),
            sort_title: None,
            year: 2024,
            status: MovieStatusType::InCinemas,
            overview: None,
            images: serde_json::json!([]),
            genres: serde_json::json!([]),
            ratings: serde_json::json!({}),
            runtime: 0,
            in_cinemas: Some(Utc::now() - Duration::days(5)),
            physical_release: None,
            digital_release: None,
            certification: None,
            website: None,
            youtube_trailer_id: None,
            studio: None,
            popularity: 0.0,
            collection_tmdb_id: 0,
            collection_title: None,
        });

        // With 10 day delay, movie from 5 days ago should not be available
        assert!(!movie.is_available(10));

        // With 3 day delay, movie from 5 days ago should be available
        assert!(movie.is_available(3));
    }

    #[test]
    fn test_is_available_missing_dates_fallback() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::Released;
        movie.movie_metadata = Some(MovieMetadata {
            id: 1,
            tmdb_id: 1,
            imdb_id: None,
            title: "Test".to_string(),
            original_title: None,
            clean_title: "test".to_string(),
            sort_title: None,
            year: 2024,
            status: MovieStatusType::Released,
            overview: None,
            images: serde_json::json!([]),
            genres: serde_json::json!([]),
            ratings: serde_json::json!({}),
            runtime: 0,
            in_cinemas: Some(Utc::now() - Duration::days(100)),
            physical_release: None,
            digital_release: None,
            certification: None,
            website: None,
            youtube_trailer_id: None,
            studio: None,
            popularity: 0.0,
            collection_tmdb_id: 0,
            collection_title: None,
        });

        // Should use InCinemas + 90 days as fallback
        assert!(movie.is_available(0));
    }

    #[test]
    fn test_is_available_no_metadata() {
        let mut movie = create_test_movie();
        movie.minimum_availability = MovieStatusType::Released;
        movie.movie_metadata = None;

        // Without metadata, can't determine availability
        assert!(!movie.is_available(0));
    }
}
