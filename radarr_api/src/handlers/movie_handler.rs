//! Movie HTTP handlers

use crate::error::AppError;
use crate::resources::movie_resource::{
    DeleteMovieParams, MovieQueryParams, MovieResource, UpdateMovieParams,
};
use crate::routes::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

/// Handles GET /api/v3/movie
///
/// Returns all movies or filters by TMDB ID if provided
///
/// # Arguments
///
/// * `service` - The movie service
/// * `params` - Query parameters (optional `tmdb_id` filter)
///
/// # Returns
///
/// Returns JSON array of movie resources
///
/// # Errors
///
/// Returns 500 for database errors
pub async fn get_all_movies(
    State(state): State<AppState>,
    Query(params): Query<MovieQueryParams>,
) -> Result<Json<Vec<MovieResource>>, AppError> {
    let service = &state.movie_service;

    // Handle tmdb_id filter if present
    if let Some(tmdb_id) = params.tmdb_id {
        if let Some(movie) = service.find_by_tmdb_id(tmdb_id).await? {
            let resource = MovieResource::from_movie(movie, 0);
            return Ok(Json(vec![resource]));
        }
        return Ok(Json(vec![]));
    }

    // Call service.get_all_movies if no filter
    let movies = service.get_all_movies().await?;

    // Map movies to resources
    let resources = movies
        .into_iter()
        .map(|m| MovieResource::from_movie(m, 0))
        .collect();

    // Return JSON response
    Ok(Json(resources))
}

/// Handles GET /api/v3/movie/:id
///
/// Returns a single movie by ID
///
/// # Arguments
///
/// * `service` - The movie service
/// * `id` - The movie ID from path
///
/// # Returns
///
/// Returns JSON movie resource
///
/// # Errors
///
/// Returns 404 if movie not found
/// Returns 500 for database errors
pub async fn get_movie_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<MovieResource>, AppError> {
    let service = &state.movie_service;

    // Call service.get_movie
    let movie = service.get_movie(id).await?;

    // Map movie to resource
    let resource = MovieResource::from_movie(movie, 0);

    // Return JSON response
    Ok(Json(resource))
}

/// Handles POST /api/v3/movie
///
/// Creates a new movie
///
/// # Arguments
///
/// * `service` - The movie service
/// * `resource` - The movie resource from request body
///
/// # Returns
///
/// Returns 201 Created with the created movie resource
///
/// # Errors
///
/// Returns 400 for validation errors (duplicate, missing fields, invalid references)
/// Returns 500 for database errors
pub async fn create_movie(
    State(state): State<AppState>,
    Json(resource): Json<MovieResource>,
) -> Result<(StatusCode, Json<MovieResource>), AppError> {
    let service = &state.movie_service;
    // Validate required fields
    if resource.path.is_empty() && resource.root_folder_path.is_none() {
        return Err(AppError(radarr_core::error::Error::Validation(
            "Path or rootFolderPath is required".to_string(),
        )));
    }

    if resource.quality_profile_id <= 0 {
        return Err(AppError(radarr_core::error::Error::Validation(
            "Valid qualityProfileId is required".to_string(),
        )));
    }

    if resource.tmdb_id <= 0 {
        return Err(AppError(radarr_core::error::Error::Validation(
            "Valid tmdbId is required".to_string(),
        )));
    }

    // Convert resource to movie
    let movie = resource.to_movie();

    // Create metadata from resource if provided
    let metadata = if resource.title.is_empty() {
        None
    } else {
        Some(radarr_core::domain::movie_metadata::MovieMetadata {
            id: 0,
            tmdb_id: resource.tmdb_id,
            imdb_id: resource.imdb_id.clone(),
            title: resource.title.clone(),
            original_title: resource.original_title.clone(),
            clean_title: resource.clean_title.clone().unwrap_or_else(|| {
                resource
                    .title
                    .to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect()
            }),
            sort_title: resource.sort_title.clone(),
            year: resource.year,
            status: resource.status,
            overview: resource.overview.clone(),
            images: serde_json::to_value(&resource.images)
                .unwrap_or_else(|_| serde_json::json!([])),
            genres: serde_json::to_value(&resource.genres)
                .unwrap_or_else(|_| serde_json::json!([])),
            ratings: serde_json::to_value(&resource.ratings)
                .unwrap_or_else(|_| serde_json::json!({})),
            runtime: resource.runtime,
            in_cinemas: resource.in_cinemas,
            physical_release: resource.physical_release,
            digital_release: resource.digital_release,
            certification: resource.certification.clone(),
            website: resource.website.clone(),
            youtube_trailer_id: resource.you_tube_trailer_id.clone(),
            studio: resource.studio.clone(),
            popularity: resource.popularity.unwrap_or(0.0),
            collection_tmdb_id: resource.collection.as_ref().map_or(0, |c| c.tmdb_id),
            collection_title: resource.collection.as_ref().map(|c| c.name.clone()),
        })
    };

    // Call service.add_movie_with_metadata
    let created = service
        .add_movie_with_metadata(movie, resource.tmdb_id, metadata)
        .await?;

    // Map to resource
    let created_resource = MovieResource::from_movie(created, 0);

    // Return 201 Created with resource
    Ok((StatusCode::CREATED, Json(created_resource)))
}

/// Handles PUT /api/v3/movie/:id
///
/// Updates an existing movie
///
/// # Arguments
///
/// * `service` - The movie service
/// * `id` - The movie ID from path
/// * `params` - Query parameters (`move_files` option)
/// * `resource` - The movie resource from request body
///
/// # Returns
///
/// Returns 202 Accepted with the updated movie resource
///
/// # Errors
///
/// Returns 404 if movie not found
/// Returns 400 for validation errors
/// Returns 500 for database errors
pub async fn update_movie(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(params): Query<UpdateMovieParams>,
    Json(resource): Json<MovieResource>,
) -> Result<(StatusCode, Json<MovieResource>), AppError> {
    let service = &state.movie_service;
    // Validate required fields
    if resource.path.is_empty() {
        return Err(AppError(radarr_core::error::Error::Validation(
            "Path is required".to_string(),
        )));
    }

    if resource.quality_profile_id <= 0 {
        return Err(AppError(radarr_core::error::Error::Validation(
            "Valid qualityProfileId is required".to_string(),
        )));
    }

    // Convert resource to movie
    let movie = resource.to_movie();

    // Call service.update_movie
    let updated = service.update_movie(id, movie).await?;

    // TODO: Handle move_files parameter (stub for now)
    if params.move_files {
        // File moving logic would go here
    }

    // Map to resource
    let updated_resource = MovieResource::from_movie(updated, 0);

    // Return 202 Accepted with resource
    Ok((StatusCode::ACCEPTED, Json(updated_resource)))
}

/// Handles DELETE /api/v3/movie/:id
///
/// Deletes a movie
///
/// # Arguments
///
/// * `service` - The movie service
/// * `id` - The movie ID from path
/// * `params` - Query parameters (`delete_files`, `add_import_exclusion` options)
///
/// # Returns
///
/// Returns 200 OK on successful deletion
///
/// # Errors
///
/// Returns 404 if movie not found
/// Returns 500 for database errors
pub async fn delete_movie(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(params): Query<DeleteMovieParams>,
) -> Result<StatusCode, AppError> {
    let service = &state.movie_service;

    // Call service.delete_movie
    service
        .delete_movie(id, params.delete_files, params.add_import_exclusion)
        .await?;

    // Return 200 OK
    Ok(StatusCode::OK)
}
