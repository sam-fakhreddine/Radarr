//! Domain value objects and enums

use serde::{Deserialize, Serialize};
use sqlx::Type;

/// Movie status type representing the release state of a movie
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "camelCase")]
pub enum MovieStatusType {
    /// To Be Announced - Release date not yet announced
    #[serde(rename = "tba")]
    Tba,
    /// Announced - Release date announced but not yet in cinemas
    Announced,
    /// In Cinemas - Currently showing in theaters
    InCinemas,
    /// Released - Available for home viewing
    Released,
}

impl Default for MovieStatusType {
    fn default() -> Self {
        Self::Tba
    }
}

/// Individual rating from a specific source
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    /// Rating value (e.g., 8.5 out of 10)
    pub value: f32,
    /// Number of votes/reviews
    pub votes: i32,
}

impl Rating {
    /// Creates a new `Rating`
    #[must_use]
    pub const fn new(value: f32, votes: i32) -> Self {
        Self { value, votes }
    }
}

/// Collection of ratings from various sources
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    /// IMDB rating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imdb: Option<Rating>,
    /// The Movie Database (TMDB) rating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmdb: Option<Rating>,
    /// Metacritic rating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metacritic: Option<Rating>,
    /// Rotten Tomatoes rating
    #[serde(rename = "rottenTomatoes", skip_serializing_if = "Option::is_none")]
    pub rotten_tomatoes: Option<Rating>,
}

impl Ratings {
    /// Creates a new empty `Ratings` collection
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movie_status_serialization() {
        // Test TBA serialization
        let status = MovieStatusType::Tba;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""tba""#);

        // Test Announced serialization
        let status = MovieStatusType::Announced;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""announced""#);

        // Test InCinemas serialization
        let status = MovieStatusType::InCinemas;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""inCinemas""#);

        // Test Released serialization
        let status = MovieStatusType::Released;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, r#""released""#);
    }

    #[test]
    fn test_movie_status_deserialization() {
        // Test TBA deserialization
        let status: MovieStatusType = serde_json::from_str(r#""tba""#).unwrap();
        assert_eq!(status, MovieStatusType::Tba);

        // Test Announced deserialization
        let status: MovieStatusType = serde_json::from_str(r#""announced""#).unwrap();
        assert_eq!(status, MovieStatusType::Announced);

        // Test InCinemas deserialization
        let status: MovieStatusType = serde_json::from_str(r#""inCinemas""#).unwrap();
        assert_eq!(status, MovieStatusType::InCinemas);

        // Test Released deserialization
        let status: MovieStatusType = serde_json::from_str(r#""released""#).unwrap();
        assert_eq!(status, MovieStatusType::Released);
    }

    #[test]
    fn test_movie_status_default() {
        let status = MovieStatusType::default();
        assert_eq!(status, MovieStatusType::Tba);
    }

    #[test]
    fn test_rating_creation() {
        let rating = Rating::new(8.5, 1000);
        assert_eq!(rating.value, 8.5);
        assert_eq!(rating.votes, 1000);
    }

    #[test]
    fn test_rating_serialization() {
        let rating = Rating {
            value: 8.5,
            votes: 1000,
        };
        let json = serde_json::to_string(&rating).unwrap();
        let expected = r#"{"value":8.5,"votes":1000}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_rating_deserialization() {
        let json = r#"{"value":8.5,"votes":1000}"#;
        let rating: Rating = serde_json::from_str(json).unwrap();
        assert_eq!(rating.value, 8.5);
        assert_eq!(rating.votes, 1000);
    }

    #[test]
    fn test_ratings_serialization() {
        let ratings = Ratings {
            imdb: Some(Rating::new(8.5, 1000)),
            tmdb: Some(Rating::new(8.0, 500)),
            metacritic: None,
            rotten_tomatoes: Some(Rating::new(90.0, 200)),
        };

        let json = serde_json::to_value(&ratings).unwrap();
        assert!(json.get("imdb").is_some());
        assert!(json.get("tmdb").is_some());
        assert!(json.get("metacritic").is_none());
        assert!(json.get("rottenTomatoes").is_some());
    }

    #[test]
    fn test_ratings_deserialization() {
        let json = r#"{
            "imdb": {"value": 8.5, "votes": 1000},
            "tmdb": {"value": 8.0, "votes": 500},
            "rottenTomatoes": {"value": 90.0, "votes": 200}
        }"#;

        let ratings: Ratings = serde_json::from_str(json).unwrap();
        assert!(ratings.imdb.is_some());
        assert_eq!(ratings.imdb.unwrap().value, 8.5);
        assert!(ratings.tmdb.is_some());
        assert!(ratings.metacritic.is_none());
        assert!(ratings.rotten_tomatoes.is_some());
    }

    #[test]
    fn test_ratings_default() {
        let ratings = Ratings::default();
        assert!(ratings.imdb.is_none());
        assert!(ratings.tmdb.is_none());
        assert!(ratings.metacritic.is_none());
        assert!(ratings.rotten_tomatoes.is_none());
    }
}
