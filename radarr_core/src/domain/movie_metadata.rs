//! MovieMetadata entity

use crate::domain::types::{MovieStatusType, Ratings};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// `MovieMetadata` entity containing TMDB-sourced information about a movie
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MovieMetadata {
    /// Unique identifier
    pub id: i32,

    /// The Movie Database (TMDB) ID
    #[serde(rename = "tmdbId")]
    #[sqlx(rename = "TmdbId")]
    pub tmdb_id: i32,

    /// Internet Movie Database (IMDB) ID
    #[serde(rename = "imdbId")]
    #[sqlx(rename = "ImdbId")]
    pub imdb_id: Option<String>,

    /// Movie title
    #[sqlx(rename = "Title")]
    pub title: String,

    /// Original title (in original language)
    #[serde(rename = "originalTitle")]
    #[sqlx(rename = "OriginalTitle")]
    pub original_title: Option<String>,

    /// Cleaned title for searching/matching
    #[serde(rename = "cleanTitle")]
    #[sqlx(rename = "CleanTitle")]
    pub clean_title: String,

    /// Title used for sorting
    #[serde(rename = "sortTitle")]
    #[sqlx(rename = "SortTitle")]
    pub sort_title: Option<String>,

    /// Release year
    #[sqlx(rename = "Year")]
    pub year: i32,

    /// Current status of the movie
    #[sqlx(rename = "Status")]
    pub status: MovieStatusType,

    /// Movie overview/synopsis
    #[sqlx(rename = "Overview")]
    pub overview: Option<String>,

    /// Images as JSON (posters, fanart, etc.)
    #[sqlx(rename = "Images")]
    pub images: serde_json::Value,

    /// List of genres
    #[sqlx(rename = "Genres")]
    pub genres: serde_json::Value,

    /// Ratings from various sources
    #[sqlx(rename = "Ratings")]
    pub ratings: serde_json::Value,

    /// Runtime in minutes
    #[sqlx(rename = "Runtime")]
    pub runtime: i32,

    /// Date movie was/will be in cinemas
    #[serde(rename = "inCinemas")]
    #[sqlx(rename = "InCinemas")]
    pub in_cinemas: Option<DateTime<Utc>>,

    /// Physical release date (DVD/Blu-ray)
    #[serde(rename = "physicalRelease")]
    #[sqlx(rename = "PhysicalRelease")]
    pub physical_release: Option<DateTime<Utc>>,

    /// Digital release date (streaming/download)
    #[serde(rename = "digitalRelease")]
    #[sqlx(rename = "DigitalRelease")]
    pub digital_release: Option<DateTime<Utc>>,

    /// Content rating/certification (e.g., PG-13, R)
    #[sqlx(rename = "Certification")]
    pub certification: Option<String>,

    /// Official website URL
    #[sqlx(rename = "Website")]
    pub website: Option<String>,

    /// `YouTube` trailer ID
    #[serde(rename = "youTubeTrailerId")]
    #[sqlx(rename = "YouTubeTrailerId")]
    pub youtube_trailer_id: Option<String>,

    /// Studio/production company
    #[sqlx(rename = "Studio")]
    pub studio: Option<String>,

    /// Popularity score
    #[sqlx(rename = "Popularity")]
    pub popularity: f32,

    /// TMDB collection ID if part of a collection
    #[serde(rename = "collectionTmdbId")]
    #[sqlx(rename = "CollectionTmdbId")]
    pub collection_tmdb_id: i32,

    /// Collection title if part of a collection
    #[serde(rename = "collectionTitle")]
    #[sqlx(rename = "CollectionTitle")]
    pub collection_title: Option<String>,
}

impl MovieMetadata {
    /// Parses the ratings JSON into a `Ratings` struct
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON cannot be deserialized into a `Ratings` struct
    pub fn get_ratings(&self) -> Result<Ratings, serde_json::Error> {
        serde_json::from_value(self.ratings.clone())
    }

    /// Parses the genres JSON into a `Vec<String>`
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON cannot be deserialized into a `Vec<String>`
    pub fn get_genres(&self) -> Result<Vec<String>, serde_json::Error> {
        serde_json::from_value(self.genres.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_metadata() -> MovieMetadata {
        MovieMetadata {
            id: 1,
            tmdb_id: 27205,
            imdb_id: Some("tt1375666".to_string()),
            title: "Inception".to_string(),
            original_title: Some("Inception".to_string()),
            clean_title: "inception".to_string(),
            sort_title: Some("inception".to_string()),
            year: 2010,
            status: MovieStatusType::Released,
            overview: Some("A thief who steals corporate secrets...".to_string()),
            images: serde_json::json!([]),
            genres: serde_json::json!(["Action", "Science Fiction", "Thriller"]),
            ratings: serde_json::json!({
                "imdb": {"value": 8.8, "votes": 2000000},
                "tmdb": {"value": 8.3, "votes": 25000}
            }),
            runtime: 148,
            in_cinemas: Some(Utc::now()),
            physical_release: Some(Utc::now()),
            digital_release: Some(Utc::now()),
            certification: Some("PG-13".to_string()),
            website: Some("https://www.warnerbros.com/movies/inception".to_string()),
            youtube_trailer_id: Some("YoHD9XEInc0".to_string()),
            studio: Some("Warner Bros.".to_string()),
            popularity: 95.5,
            collection_tmdb_id: 0,
            collection_title: None,
        }
    }

    #[test]
    fn test_movie_metadata_creation() {
        let metadata = create_test_metadata();
        assert_eq!(metadata.id, 1);
        assert_eq!(metadata.tmdb_id, 27205);
        assert_eq!(metadata.title, "Inception");
        assert_eq!(metadata.year, 2010);
    }

    #[test]
    fn test_movie_metadata_serialization() {
        let metadata = create_test_metadata();
        let json = serde_json::to_value(&metadata).unwrap();

        assert_eq!(json["id"], 1);
        assert_eq!(json["tmdbId"], 27205);
        assert_eq!(json["imdbId"], "tt1375666");
        assert_eq!(json["title"], "Inception");
        assert_eq!(json["year"], 2010);
    }

    #[test]
    fn test_get_ratings() {
        let metadata = create_test_metadata();
        let ratings = metadata.get_ratings().unwrap();

        assert!(ratings.imdb.is_some());
        assert_eq!(ratings.imdb.unwrap().value, 8.8);
        assert!(ratings.tmdb.is_some());
    }

    #[test]
    fn test_get_genres() {
        let metadata = create_test_metadata();
        let genres = metadata.get_genres().unwrap();

        assert_eq!(genres.len(), 3);
        assert!(genres.contains(&"Action".to_string()));
        assert!(genres.contains(&"Science Fiction".to_string()));
        assert!(genres.contains(&"Thriller".to_string()));
    }
}
