# Requirements Document

## Introduction

This specification defines the requirements for porting Radarr's movie CRUD (Create, Read, Update, Delete) operations from C#/.NET to Rust. The Rust implementation SHALL maintain full API compatibility with the existing Radarr V3 API while providing equivalent functionality for managing movie entities in the database. This port focuses on the core domain model, database persistence layer, and REST API endpoints for movie management.

**Reference Implementation**: The original C# implementation is available in `legacy/src/` for reference during development.

## Glossary

- **Radarr-RS**: The Rust port of the Radarr application
- **Movie Entity**: A database record representing a movie with associated metadata, quality profile, and file information
- **MovieMetadata**: A separate entity containing TMDB-sourced information about a movie (title, year, ratings, etc.)
- **TMDB**: The Movie Database, an external metadata provider identified by TmdbId
- **IMDB**: Internet Movie Database, an external identifier for movies (ImdbId)
- **Quality Profile**: A configuration defining acceptable quality levels for movie files
- **REST API**: RESTful HTTP API endpoints for movie operations
- **Database Layer**: The persistence layer using sqlx for database operations
- **API Resource**: JSON representation of a movie entity exposed via REST endpoints

## Requirements

### Requirement 1: Movie Entity Retrieval

**User Story:** As an API client, I want to retrieve movie information by ID or query parameters, so that I can display movie details in the UI.

#### Acceptance Criteria

1. WHEN a GET request is made to `/api/v3/movie/{id}`, THE Radarr-RS SHALL return a JSON representation of the movie with the specified ID
2. WHEN a GET request is made to `/api/v3/movie` with a tmdbId query parameter, THE Radarr-RS SHALL return a JSON array containing the movie matching that TMDB ID
3. WHEN a GET request is made to `/api/v3/movie` without query parameters, THE Radarr-RS SHALL return a JSON array containing all movies in the database
4. WHEN a movie is retrieved, THE Radarr-RS SHALL include associated MovieMetadata fields (title, year, overview, ratings, images)
5. WHEN a movie is retrieved, THE Radarr-RS SHALL include computed fields (isAvailable, hasFile, statistics)
6. IF the requested movie ID does not exist, THEN THE Radarr-RS SHALL return HTTP status 404

### Requirement 2: Movie Entity Creation

**User Story:** As an API client, I want to add new movies to the library, so that Radarr can monitor and download them.

#### Acceptance Criteria

1. WHEN a POST request is made to `/api/v3/movie` with valid movie data, THE Radarr-RS SHALL create a new movie record in the database
2. WHEN a movie is created, THE Radarr-RS SHALL validate that the path is not empty or the rootFolderPath is provided
3. WHEN a movie is created with a rootFolderPath, THE Radarr-RS SHALL construct the full path using the movie title and year
4. WHEN a movie is created, THE Radarr-RS SHALL validate that the qualityProfileId references an existing quality profile
5. WHEN a movie is created, THE Radarr-RS SHALL set the added timestamp to the current UTC time
6. WHEN a movie is successfully created, THE Radarr-RS SHALL return HTTP status 201 with the created movie resource
7. IF the movie already exists by TMDB ID, THEN THE Radarr-RS SHALL return HTTP status 400 with a validation error
8. IF required fields are missing or invalid, THEN THE Radarr-RS SHALL return HTTP status 400 with validation errors

### Requirement 3: Movie Entity Update

**User Story:** As an API client, I want to update movie properties, so that I can change monitoring status, quality profiles, or paths.

#### Acceptance Criteria

1. WHEN a PUT request is made to `/api/v3/movie/{id}` with valid movie data, THE Radarr-RS SHALL update the movie record in the database
2. WHEN a movie is updated, THE Radarr-RS SHALL allow modification of path, qualityProfileId, monitored, minimumAvailability, rootFolderPath, and tags fields
3. WHEN a movie is updated with moveFiles query parameter set to true, THE Radarr-RS SHALL queue a move command for the movie files
4. WHEN a movie is updated, THE Radarr-RS SHALL validate that the path is not empty
5. WHEN a movie is updated, THE Radarr-RS SHALL validate that the qualityProfileId references an existing quality profile
6. WHEN a movie is successfully updated, THE Radarr-RS SHALL return HTTP status 202 with the updated movie resource
7. IF the movie ID does not exist, THEN THE Radarr-RS SHALL return HTTP status 404
8. IF validation fails, THEN THE Radarr-RS SHALL return HTTP status 400 with validation errors

### Requirement 4: Movie Entity Deletion

**User Story:** As an API client, I want to delete movies from the library, so that I can remove unwanted content.

#### Acceptance Criteria

1. WHEN a DELETE request is made to `/api/v3/movie/{id}`, THE Radarr-RS SHALL remove the movie record from the database
2. WHEN a movie is deleted with deleteFiles query parameter set to true, THE Radarr-RS SHALL delete associated movie files from disk
3. WHEN a movie is deleted with addImportExclusion query parameter set to true, THE Radarr-RS SHALL add the movie to the import exclusion list
4. WHEN a movie is successfully deleted, THE Radarr-RS SHALL return HTTP status 200
5. IF the movie ID does not exist, THEN THE Radarr-RS SHALL return HTTP status 404

### Requirement 5: Database Schema Compatibility

**User Story:** As a system administrator, I want the Rust implementation to use the same database schema, so that I can migrate between implementations without data loss.

#### Acceptance Criteria

1. THE Radarr-RS SHALL use a Movies table with columns matching the C# implementation (Id, MovieMetadataId, Monitored, MinimumAvailability, QualityProfileId, Path, RootFolderPath, Added, Tags, AddOptions, LastSearchTime, MovieFileId)
2. THE Radarr-RS SHALL use a MovieMetadata table with columns matching the C# implementation (Id, TmdbId, ImdbId, Title, OriginalTitle, CleanTitle, SortTitle, Year, Status, Overview, Images, Genres, Ratings, Runtime, InCinemas, PhysicalRelease, DigitalRelease, Certification, Website, YouTubeTrailerId, Studio, Popularity, CollectionTmdbId, CollectionTitle)
3. THE Radarr-RS SHALL support both SQLite and PostgreSQL database backends
4. THE Radarr-RS SHALL use foreign key relationships between Movies and MovieMetadata tables
5. THE Radarr-RS SHALL serialize complex fields (Tags, Images, Genres, Ratings) as JSON in the database

### Requirement 6: Movie Lookup Operations

**User Story:** As an API client, I want to find movies by various identifiers, so that I can avoid duplicates and locate existing entries.

#### Acceptance Criteria

1. WHEN FindByTmdbId is called with a TMDB ID, THE Radarr-RS SHALL return the movie with matching MovieMetadata TmdbId
2. WHEN FindByImdbId is called with an IMDB ID, THE Radarr-RS SHALL return the movie with matching MovieMetadata ImdbId
3. WHEN FindByPath is called with a file path, THE Radarr-RS SHALL return the movie with matching Path field
4. WHEN FindByTitle is called with a title and year, THE Radarr-RS SHALL search by clean title in MovieMetadata, AlternativeTitles, and Translations
5. WHEN multiple movies match a title search, THE Radarr-RS SHALL filter by year if provided
6. IF no movie matches the search criteria, THEN THE Radarr-RS SHALL return None

### Requirement 7: Movie Availability Calculation

**User Story:** As an API client, I want to know if a movie is available for download, so that I can determine if searching should occur.

#### Acceptance Criteria

1. WHEN IsAvailable is called on a movie, THE Radarr-RS SHALL compare the current UTC time against the minimum availability date
2. WHEN MinimumAvailability is TBA or Announced, THE Radarr-RS SHALL return true for availability
3. WHEN MinimumAvailability is InCinemas, THE Radarr-RS SHALL use the InCinemas date from MovieMetadata
4. WHEN MinimumAvailability is Released, THE Radarr-RS SHALL use the earlier of PhysicalRelease or DigitalRelease dates
5. WHEN availability delay is configured, THE Radarr-RS SHALL add the delay in days to the minimum availability date
6. IF no release dates are available, THE Radarr-RS SHALL use InCinemas plus 90 days as the default availability date

### Requirement 8: JSON Serialization Compatibility

**User Story:** As an API client, I want JSON responses to match the existing format, so that existing frontends work without modification.

#### Acceptance Criteria

1. THE Radarr-RS SHALL serialize movie resources with camelCase field names
2. THE Radarr-RS SHALL include all fields present in the C# MovieResource (id, title, year, tmdbId, imdbId, path, qualityProfileId, monitored, minimumAvailability, status, overview, images, ratings, genres, tags, added, hasFile, movieFileId, statistics, isAvailable, etc.)
3. THE Radarr-RS SHALL omit null optional fields from JSON output
4. THE Radarr-RS SHALL serialize DateTime fields in ISO 8601 format with UTC timezone
5. THE Radarr-RS SHALL serialize enum fields as strings matching C# enum names (e.g., "released", "inCinemas")
6. THE Radarr-RS SHALL serialize nested objects (ratings, collection, statistics) with matching structure

### Requirement 9: Database Transaction Handling

**User Story:** As a system operator, I want database operations to be atomic, so that data integrity is maintained during failures.

#### Acceptance Criteria

1. WHEN a movie create operation fails, THE Radarr-RS SHALL rollback all database changes
2. WHEN a movie update operation fails, THE Radarr-RS SHALL rollback all database changes
3. WHEN a movie delete operation fails, THE Radarr-RS SHALL rollback all database changes
4. THE Radarr-RS SHALL use database transactions for all write operations
5. THE Radarr-RS SHALL release database connections after operations complete

### Requirement 10: Error Handling and Validation

**User Story:** As an API client, I want clear error messages, so that I can understand and fix validation failures.

#### Acceptance Criteria

1. WHEN validation fails, THE Radarr-RS SHALL return a JSON error response with field-specific messages
2. WHEN a database error occurs, THE Radarr-RS SHALL return HTTP status 500 with a generic error message
3. WHEN a resource is not found, THE Radarr-RS SHALL return HTTP status 404 with a descriptive message
4. THE Radarr-RS SHALL validate path format before database operations
5. THE Radarr-RS SHALL validate that quality profile exists before creating or updating movies
6. THE Radarr-RS SHALL log all errors with sufficient context for debugging
