//! Movie API resource

use chrono::{DateTime, Utc};
use radarr_core::domain::types::{MovieStatusType, Ratings};
use serde::{Deserialize, Serialize};

/// Media cover image information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaCover {
    /// Type of cover (poster, fanart, banner, etc.)
    pub cover_type: String,
    /// URL to the image
    pub url: String,
    /// Remote URL (original source)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_url: Option<String>,
}

/// Movie statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieStatistics {
    /// Size of movie file in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_on_disk: Option<i64>,
    /// Percentage of movie downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_of_movie: Option<f32>,
}

/// Collection information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieCollection {
    /// Collection name
    pub name: String,
    /// TMDB collection ID
    pub tmdb_id: i32,
    /// Collection images
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<MediaCover>>,
}

/// Movie resource for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieResource {
    /// Unique identifier (0 for new movies)
    #[serde(default)]
    pub id: i32,

    /// Movie title
    #[serde(default)]
    pub title: String,

    /// Original title (in original language)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,

    /// Alternative titles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_titles: Option<Vec<String>>,

    /// Release year
    #[serde(default)]
    pub year: i32,

    /// The Movie Database (TMDB) ID
    #[serde(default)]
    pub tmdb_id: i32,

    /// Internet Movie Database (IMDB) ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<String>,

    /// File system path where movie is/will be stored
    #[serde(default)]
    pub path: String,

    /// Root folder path (used during creation)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_folder_path: Option<String>,

    /// Quality profile ID for this movie
    #[serde(default)]
    pub quality_profile_id: i32,

    /// Whether the movie is being monitored for downloads
    #[serde(default)]
    pub monitored: bool,

    /// Minimum availability required before searching
    #[serde(default)]
    pub minimum_availability: MovieStatusType,

    /// Whether the movie is available for download
    #[serde(default)]
    pub is_available: bool,

    /// Whether the movie has an associated file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_file: Option<bool>,

    /// Foreign key to `MovieFile` (0 if no file)
    #[serde(default)]
    pub movie_file_id: i32,

    /// Current status of the movie
    #[serde(default)]
    pub status: MovieStatusType,

    /// Movie overview/synopsis
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,

    /// Images (posters, fanart, etc.)
    #[serde(default)]
    pub images: Vec<MediaCover>,

    /// List of genres
    #[serde(default)]
    pub genres: Vec<String>,

    /// Ratings from various sources
    #[serde(default)]
    pub ratings: Ratings,

    /// Runtime in minutes
    #[serde(default)]
    pub runtime: i32,

    /// Date movie was/will be in cinemas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_cinemas: Option<DateTime<Utc>>,

    /// Physical release date (DVD/Blu-ray)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_release: Option<DateTime<Utc>>,

    /// Digital release date (streaming/download)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digital_release: Option<DateTime<Utc>>,

    /// Content rating/certification (e.g., PG-13, R)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certification: Option<String>,

    /// Official website URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,

    /// `YouTube` trailer ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub you_tube_trailer_id: Option<String>,

    /// Studio/production company
    #[serde(skip_serializing_if = "Option::is_none")]
    pub studio: Option<String>,

    /// Tags associated with this movie
    #[serde(default)]
    pub tags: Vec<i32>,

    /// Timestamp when movie was added to library
    #[serde(default = "chrono::Utc::now")]
    pub added: DateTime<Utc>,

    /// Additional options used when adding the movie
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_options: Option<serde_json::Value>,

    /// Last time a search was performed for this movie
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_search_time: Option<DateTime<Utc>>,

    /// Movie statistics (file size, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<MovieStatistics>,

    /// Collection information if part of a collection
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<MovieCollection>,

    /// Cleaned title for searching/matching
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_title: Option<String>,

    /// Title used for sorting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_title: Option<String>,

    /// Popularity score
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f32>,
}

impl MovieResource {
    /// Converts a `Movie` entity to a `MovieResource` for API responses
    ///
    /// # Arguments
    ///
    /// * `movie` - The movie entity to convert
    /// * `availability_delay` - Number of days to add to the availability date
    ///
    /// # Returns
    ///
    /// Returns a `MovieResource` with all fields populated from the movie and its metadata
    #[must_use]
    #[allow(dead_code)] // Will be used by handlers
    pub fn from_movie(movie: radarr_core::domain::movie::Movie, availability_delay: i32) -> Self {
        let metadata = movie.movie_metadata.as_ref();

        // Parse images from JSON
        let images = metadata
            .and_then(|m| serde_json::from_value::<Vec<MediaCover>>(m.images.clone()).ok())
            .unwrap_or_default();

        // Parse genres from JSON
        let genres = metadata
            .and_then(|m| serde_json::from_value::<Vec<String>>(m.genres.clone()).ok())
            .unwrap_or_default();

        // Parse ratings from JSON
        let ratings = metadata
            .and_then(|m| serde_json::from_value::<Ratings>(m.ratings.clone()).ok())
            .unwrap_or_default();

        // Parse tags from JSON
        let tags = movie.get_tags().unwrap_or_default();

        // Build collection if present
        let collection = metadata.and_then(|m| {
            if m.collection_tmdb_id > 0 {
                Some(MovieCollection {
                    name: m.collection_title.clone().unwrap_or_default(),
                    tmdb_id: m.collection_tmdb_id,
                    images: None,
                })
            } else {
                None
            }
        });

        // Calculate is_available before consuming movie
        let is_available = movie.is_available(availability_delay);
        let has_file = movie.has_file();

        Self {
            id: movie.id,
            title: metadata.map_or_else(String::new, |m| m.title.clone()),
            original_title: metadata.and_then(|m| m.original_title.clone()),
            alternative_titles: None, // Would come from AlternativeTitles table
            year: metadata.map_or(0, |m| m.year),
            tmdb_id: metadata.map_or(0, |m| m.tmdb_id),
            imdb_id: metadata.and_then(|m| m.imdb_id.clone()),
            path: movie.path,
            root_folder_path: movie.root_folder_path,
            quality_profile_id: movie.quality_profile_id,
            monitored: movie.monitored,
            minimum_availability: movie.minimum_availability,
            is_available,
            has_file: Some(has_file),
            movie_file_id: movie.movie_file_id,
            status: metadata.map_or(MovieStatusType::Tba, |m| m.status),
            overview: metadata.and_then(|m| m.overview.clone()),
            images,
            genres,
            ratings,
            runtime: metadata.map_or(0, |m| m.runtime),
            in_cinemas: metadata.and_then(|m| m.in_cinemas),
            physical_release: metadata.and_then(|m| m.physical_release),
            digital_release: metadata.and_then(|m| m.digital_release),
            certification: metadata.and_then(|m| m.certification.clone()),
            website: metadata.and_then(|m| m.website.clone()),
            you_tube_trailer_id: metadata.and_then(|m| m.youtube_trailer_id.clone()),
            studio: metadata.and_then(|m| m.studio.clone()),
            tags,
            added: movie.added,
            add_options: movie.add_options,
            last_search_time: movie.last_search_time,
            statistics: None, // Would be calculated from MovieFile
            collection,
            clean_title: metadata.map(|m| m.clean_title.clone()),
            sort_title: metadata.and_then(|m| m.sort_title.clone()),
            popularity: metadata.map(|m| m.popularity),
        }
    }

    /// Converts a `MovieResource` to a `Movie` entity
    ///
    /// Note: This conversion is primarily used for create/update operations.
    /// The `movie_metadata_id` field will need to be set separately based on
    /// the TMDB ID lookup.
    ///
    /// # Returns
    ///
    /// Returns a `Movie` entity with fields populated from the resource
    #[must_use]
    #[allow(dead_code)] // Will be used by handlers
    pub fn to_movie(&self) -> radarr_core::domain::movie::Movie {
        // Convert tags Vec<i32> to JSON
        let tags = serde_json::to_value(&self.tags).unwrap_or_else(|_| serde_json::json!([]));

        radarr_core::domain::movie::Movie {
            id: self.id,
            movie_metadata_id: 0, // Will be set by service layer based on TMDB ID
            monitored: self.monitored,
            minimum_availability: self.minimum_availability,
            quality_profile_id: self.quality_profile_id,
            path: self.path.clone(),
            root_folder_path: self.root_folder_path.clone(),
            added: self.added,
            tags,
            add_options: self.add_options.clone(),
            last_search_time: self.last_search_time,
            movie_file_id: self.movie_file_id,
            movie_metadata: None, // Will be loaded by repository if needed
        }
    }
}

/// Query parameters for GET /api/v3/movie
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Will be used by handlers
pub struct MovieQueryParams {
    /// Filter by TMDB ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmdb_id: Option<i32>,

    /// Whether to exclude local covers (for image optimization)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_local_covers: Option<bool>,

    /// Language ID for localized content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_id: Option<i32>,
}

/// Query parameters for PUT /api/v3/movie/:id
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Will be used by handlers
pub struct UpdateMovieParams {
    /// Whether to move files when path changes
    #[serde(default)]
    pub move_files: bool,
}

/// Query parameters for DELETE /api/v3/movie/:id
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Will be used by handlers
pub struct DeleteMovieParams {
    /// Whether to delete associated movie files from disk
    #[serde(default)]
    pub delete_files: bool,

    /// Whether to add the movie to the import exclusion list
    #[serde(default)]
    pub add_import_exclusion: bool,
}
