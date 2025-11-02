# Design Document

## Overview

This design document outlines the Rust implementation of Radarr's movie CRUD operations. The architecture follows a layered approach with clear separation between the API layer, service layer, and data access layer. The implementation uses modern Rust patterns and libraries while maintaining API compatibility with the existing Radarr V3 endpoints.

**Project Layout**: The Rust implementation is in the root directory (`radarr_core/`, `radarr_api/`), while the original C# code has been moved to `legacy/` for reference during conversion.

### Technology Stack

- **Web Framework**: Axum (async, type-safe routing)
- **Database**: sqlx (compile-time checked SQL, async)
- **Serialization**: serde + serde_json (JSON handling)
- **Validation**: validator (declarative validation)
- **Error Handling**: thiserror + anyhow
- **Async Runtime**: tokio

## Architecture

### Layer Structure

```
┌─────────────────────────────────────┐
│     API Layer (Axum Handlers)       │
│  - HTTP routing                     │
│  - Request/response mapping         │
│  - Validation                       │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│     Service Layer                   │
│  - Business logic                   │
│  - Domain operations                │
│  - Event publishing                 │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│     Repository Layer (sqlx)         │
│  - Database queries                 │
│  - Transaction management           │
│  - Data mapping                     │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│     Database (SQLite/PostgreSQL)    │
└─────────────────────────────────────┘
```

### Module Organization

```
radarr_core/
├── domain/
│   ├── movie.rs           # Movie entity
│   ├── movie_metadata.rs  # MovieMetadata entity
│   └── types.rs           # Enums, value objects
├── repository/
│   ├── movie_repository.rs
│   └── traits.rs          # Repository interfaces
├── service/
│   └── movie_service.rs   # Business logic
└── error.rs               # Domain errors

radarr_api/
├── handlers/
│   └── movie_handler.rs   # HTTP handlers
├── resources/
│   └── movie_resource.rs  # API DTOs
├── validation/
│   └── movie_validator.rs # Request validation
└── error.rs               # API errors
```

## Components and Interfaces

### 1. Domain Models

#### Movie Entity

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Movie {
    pub id: i32,
    pub movie_metadata_id: i32,
    pub monitored: bool,
    pub minimum_availability: MovieStatusType,
    pub quality_profile_id: i32,
    pub path: String,
    pub root_folder_path: Option<String>,
    pub added: DateTime<Utc>,
    pub tags: Vec<i32>,
    pub add_options: Option<serde_json::Value>,
    pub last_search_time: Option<DateTime<Utc>>,
    pub movie_file_id: i32,
    
    // Lazy-loaded relationship
    #[sqlx(skip)]
    pub movie_metadata: Option<MovieMetadata>,
}

impl Movie {
    pub fn has_file(&self) -> bool {
        self.movie_file_id > 0
    }
    
    pub fn is_available(&self, delay: i32) -> bool {
        // Implementation of availability logic
    }
    
    pub fn apply_changes(&mut self, other: &Movie) {
        self.path = other.path.clone();
        self.quality_profile_id = other.quality_profile_id;
        self.monitored = other.monitored;
        self.minimum_availability = other.minimum_availability;
        self.root_folder_path = other.root_folder_path.clone();
        self.tags = other.tags.clone();
    }
}
```

#### MovieMetadata Entity

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MovieMetadata {
    pub id: i32,
    pub tmdb_id: i32,
    pub imdb_id: Option<String>,
    pub title: String,
    pub original_title: Option<String>,
    pub clean_title: String,
    pub sort_title: Option<String>,
    pub year: i32,
    pub status: MovieStatusType,
    pub overview: Option<String>,
    pub images: serde_json::Value,  // JSON array
    pub genres: Vec<String>,
    pub ratings: Ratings,
    pub runtime: i32,
    pub in_cinemas: Option<DateTime<Utc>>,
    pub physical_release: Option<DateTime<Utc>>,
    pub digital_release: Option<DateTime<Utc>>,
    pub certification: Option<String>,
    pub website: Option<String>,
    pub youtube_trailer_id: Option<String>,
    pub studio: Option<String>,
    pub popularity: f32,
    pub collection_tmdb_id: i32,
    pub collection_title: Option<String>,
}
```

#### Domain Types

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "camelCase")]
pub enum MovieStatusType {
    Tba,
    Announced,
    InCinemas,
    Released,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ratings {
    pub imdb: Option<Rating>,
    pub tmdb: Option<Rating>,
    pub metacritic: Option<Rating>,
    pub rotten_tomatoes: Option<Rating>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    pub value: f32,
    pub votes: i32,
}
```

### 2. Repository Layer

#### MovieRepository Trait

```rust
#[async_trait]
pub trait MovieRepository: Send + Sync {
    async fn get(&self, id: i32) -> Result<Movie>;
    async fn find(&self, id: i32) -> Result<Option<Movie>>;
    async fn all(&self) -> Result<Vec<Movie>>;
    async fn insert(&self, movie: &Movie) -> Result<Movie>;
    async fn update(&self, movie: &Movie) -> Result<Movie>;
    async fn delete(&self, id: i32) -> Result<()>;
    
    // Lookup methods
    async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<Movie>>;
    async fn find_by_imdb_id(&self, imdb_id: &str) -> Result<Option<Movie>>;
    async fn find_by_path(&self, path: &str) -> Result<Option<Movie>>;
    async fn find_by_titles(&self, titles: &[String]) -> Result<Vec<Movie>>;
    
    // Bulk operations
    async fn insert_many(&self, movies: &[Movie]) -> Result<()>;
    async fn update_many(&self, movies: &[Movie]) -> Result<()>;
    async fn delete_many(&self, ids: &[i32]) -> Result<()>;
    
    // Query methods
    async fn all_movie_paths(&self) -> Result<HashMap<i32, String>>;
    async fn all_movie_tmdb_ids(&self) -> Result<Vec<i32>>;
    async fn movies_between_dates(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        include_unmonitored: bool,
    ) -> Result<Vec<Movie>>;
}
```

#### SqlxMovieRepository Implementation

```rust
pub struct SqlxMovieRepository {
    pool: PgPool,  // Or SqlitePool
}

impl SqlxMovieRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    async fn load_metadata(&self, movies: &mut [Movie]) -> Result<()> {
        let metadata_ids: Vec<i32> = movies
            .iter()
            .map(|m| m.movie_metadata_id)
            .collect();
            
        let metadata = sqlx::query_as::<_, MovieMetadata>(
            "SELECT * FROM MovieMetadata WHERE Id = ANY($1)"
        )
        .bind(&metadata_ids)
        .fetch_all(&self.pool)
        .await?;
        
        let metadata_map: HashMap<i32, MovieMetadata> = metadata
            .into_iter()
            .map(|m| (m.id, m))
            .collect();
            
        for movie in movies {
            movie.movie_metadata = metadata_map
                .get(&movie.movie_metadata_id)
                .cloned();
        }
        
        Ok(())
    }
}

#[async_trait]
impl MovieRepository for SqlxMovieRepository {
    async fn get(&self, id: i32) -> Result<Movie> {
        let mut movie = sqlx::query_as::<_, Movie>(
            r#"
            SELECT m.* 
            FROM Movies m
            WHERE m.Id = $1
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::NotFound(format!("Movie with id {} not found", id)),
            _ => Error::Database(e.into()),
        })?;
        
        self.load_metadata(&mut [movie.clone()]).await?;
        
        Ok(movie)
    }
    
    async fn insert(&self, movie: &Movie) -> Result<Movie> {
        let id = sqlx::query_scalar::<_, i32>(
            r#"
            INSERT INTO Movies (
                MovieMetadataId, Monitored, MinimumAvailability,
                QualityProfileId, Path, RootFolderPath, Added,
                Tags, AddOptions, LastSearchTime, MovieFileId
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING Id
            "#
        )
        .bind(movie.movie_metadata_id)
        .bind(movie.monitored)
        .bind(&movie.minimum_availability)
        .bind(movie.quality_profile_id)
        .bind(&movie.path)
        .bind(&movie.root_folder_path)
        .bind(movie.added)
        .bind(serde_json::to_value(&movie.tags)?)
        .bind(&movie.add_options)
        .bind(movie.last_search_time)
        .bind(movie.movie_file_id)
        .fetch_one(&self.pool)
        .await?;
        
        self.get(id).await
    }
    
    // Additional implementations...
}
```

### 3. Service Layer

#### MovieService

```rust
pub struct MovieService {
    repository: Arc<dyn MovieRepository>,
    config: Arc<Config>,
}

impl MovieService {
    pub fn new(repository: Arc<dyn MovieRepository>, config: Arc<Config>) -> Self {
        Self { repository, config }
    }
    
    pub async fn get_movie(&self, id: i32) -> Result<Movie> {
        self.repository.get(id).await
    }
    
    pub async fn get_all_movies(&self) -> Result<Vec<Movie>> {
        self.repository.all().await
    }
    
    pub async fn add_movie(&self, mut movie: Movie) -> Result<Movie> {
        // Validate movie doesn't already exist
        if let Some(existing) = self.find_by_tmdb_id(movie.movie_metadata_id).await? {
            return Err(Error::Validation(
                "Movie already exists".to_string()
            ));
        }
        
        // Set added timestamp
        movie.added = Utc::now();
        
        // Insert movie
        let created = self.repository.insert(&movie).await?;
        
        Ok(created)
    }
    
    pub async fn update_movie(&self, id: i32, updates: Movie) -> Result<Movie> {
        let mut existing = self.repository.get(id).await?;
        existing.apply_changes(&updates);
        
        self.repository.update(&existing).await
    }
    
    pub async fn delete_movie(
        &self,
        id: i32,
        delete_files: bool,
        add_import_exclusion: bool,
    ) -> Result<()> {
        let movie = self.repository.get(id).await?;
        
        // Delete from database
        self.repository.delete(id).await?;
        
        // Handle file deletion if requested
        if delete_files && !movie.path.is_empty() {
            // File deletion logic would go here
        }
        
        // Handle import exclusion if requested
        if add_import_exclusion {
            // Import exclusion logic would go here
        }
        
        Ok(())
    }
    
    pub async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<Movie>> {
        self.repository.find_by_tmdb_id(tmdb_id).await
    }
    
    pub async fn find_by_imdb_id(&self, imdb_id: &str) -> Result<Option<Movie>> {
        self.repository.find_by_imdb_id(imdb_id).await
    }
}
```

### 4. API Layer

#### MovieResource (DTO)

```rust
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieResource {
    pub id: i32,
    pub title: String,
    pub original_title: Option<String>,
    pub year: i32,
    pub tmdb_id: i32,
    pub imdb_id: Option<String>,
    pub path: String,
    pub quality_profile_id: i32,
    pub monitored: bool,
    pub minimum_availability: MovieStatusType,
    pub is_available: bool,
    pub has_file: Option<bool>,
    pub movie_file_id: i32,
    pub status: MovieStatusType,
    pub overview: Option<String>,
    pub images: Vec<MediaCover>,
    pub genres: Vec<String>,
    pub ratings: Ratings,
    pub runtime: i32,
    pub in_cinemas: Option<DateTime<Utc>>,
    pub physical_release: Option<DateTime<Utc>>,
    pub digital_release: Option<DateTime<Utc>>,
    pub certification: Option<String>,
    pub tags: Vec<i32>,
    pub added: DateTime<Utc>,
    pub root_folder_path: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<MovieStatistics>,
}

impl MovieResource {
    pub fn from_movie(movie: Movie, avail_delay: i32) -> Self {
        let metadata = movie.movie_metadata.as_ref();
        
        Self {
            id: movie.id,
            title: metadata.map(|m| m.title.clone()).unwrap_or_default(),
            original_title: metadata.and_then(|m| m.original_title.clone()),
            year: metadata.map(|m| m.year).unwrap_or(0),
            tmdb_id: metadata.map(|m| m.tmdb_id).unwrap_or(0),
            imdb_id: metadata.and_then(|m| m.imdb_id.clone()),
            path: movie.path,
            quality_profile_id: movie.quality_profile_id,
            monitored: movie.monitored,
            minimum_availability: movie.minimum_availability,
            is_available: movie.is_available(avail_delay),
            has_file: Some(movie.has_file()),
            movie_file_id: movie.movie_file_id,
            status: metadata.map(|m| m.status).unwrap_or(MovieStatusType::Tba),
            overview: metadata.and_then(|m| m.overview.clone()),
            images: vec![],  // Parse from metadata.images JSON
            genres: metadata.map(|m| m.genres.clone()).unwrap_or_default(),
            ratings: metadata.map(|m| m.ratings.clone()).unwrap_or_default(),
            runtime: metadata.map(|m| m.runtime).unwrap_or(0),
            in_cinemas: metadata.and_then(|m| m.in_cinemas),
            physical_release: metadata.and_then(|m| m.physical_release),
            digital_release: metadata.and_then(|m| m.digital_release),
            certification: metadata.and_then(|m| m.certification.clone()),
            tags: movie.tags,
            added: movie.added,
            root_folder_path: movie.root_folder_path,
            statistics: None,
        }
    }
    
    pub fn to_movie(&self) -> Movie {
        Movie {
            id: self.id,
            movie_metadata_id: 0,  // Set by service
            monitored: self.monitored,
            minimum_availability: self.minimum_availability,
            quality_profile_id: self.quality_profile_id,
            path: self.path.clone(),
            root_folder_path: self.root_folder_path.clone(),
            added: self.added,
            tags: self.tags.clone(),
            add_options: None,
            last_search_time: None,
            movie_file_id: self.movie_file_id,
            movie_metadata: None,
        }
    }
}
```

#### HTTP Handlers

```rust
pub async fn get_all_movies(
    State(service): State<Arc<MovieService>>,
    Query(params): Query<MovieQueryParams>,
) -> Result<Json<Vec<MovieResource>>, ApiError> {
    if let Some(tmdb_id) = params.tmdb_id {
        if let Some(movie) = service.find_by_tmdb_id(tmdb_id).await? {
            return Ok(Json(vec![MovieResource::from_movie(movie, 0)]));
        }
        return Ok(Json(vec![]));
    }
    
    let movies = service.get_all_movies().await?;
    let resources = movies
        .into_iter()
        .map(|m| MovieResource::from_movie(m, 0))
        .collect();
    
    Ok(Json(resources))
}

pub async fn get_movie_by_id(
    State(service): State<Arc<MovieService>>,
    Path(id): Path<i32>,
) -> Result<Json<MovieResource>, ApiError> {
    let movie = service.get_movie(id).await?;
    Ok(Json(MovieResource::from_movie(movie, 0)))
}

pub async fn create_movie(
    State(service): State<Arc<MovieService>>,
    Json(resource): Json<MovieResource>,
) -> Result<(StatusCode, Json<MovieResource>), ApiError> {
    let movie = resource.to_movie();
    let created = service.add_movie(movie).await?;
    
    Ok((
        StatusCode::CREATED,
        Json(MovieResource::from_movie(created, 0)),
    ))
}

pub async fn update_movie(
    State(service): State<Arc<MovieService>>,
    Path(id): Path<i32>,
    Query(params): Query<UpdateMovieParams>,
    Json(resource): Json<MovieResource>,
) -> Result<(StatusCode, Json<MovieResource>), ApiError> {
    let movie = resource.to_movie();
    let updated = service.update_movie(id, movie).await?;
    
    Ok((
        StatusCode::ACCEPTED,
        Json(MovieResource::from_movie(updated, 0)),
    ))
}

pub async fn delete_movie(
    State(service): State<Arc<MovieService>>,
    Path(id): Path<i32>,
    Query(params): Query<DeleteMovieParams>,
) -> Result<StatusCode, ApiError> {
    service.delete_movie(
        id,
        params.delete_files.unwrap_or(false),
        params.add_import_exclusion.unwrap_or(false),
    ).await?;
    
    Ok(StatusCode::OK)
}
```

#### Router Configuration

```rust
pub fn movie_routes() -> Router<AppState> {
    Router::new()
        .route("/api/v3/movie", get(get_all_movies).post(create_movie))
        .route(
            "/api/v3/movie/:id",
            get(get_movie_by_id)
                .put(update_movie)
                .delete(delete_movie),
        )
}
```

## Data Models

### Database Schema

```sql
-- Movies table
CREATE TABLE Movies (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    MovieMetadataId INTEGER NOT NULL,
    Monitored INTEGER NOT NULL,
    MinimumAvailability TEXT NOT NULL,
    QualityProfileId INTEGER NOT NULL,
    Path TEXT NOT NULL,
    RootFolderPath TEXT,
    Added TEXT NOT NULL,
    Tags TEXT,
    AddOptions TEXT,
    LastSearchTime TEXT,
    MovieFileId INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (MovieMetadataId) REFERENCES MovieMetadata(Id)
);

-- MovieMetadata table
CREATE TABLE MovieMetadata (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    TmdbId INTEGER NOT NULL UNIQUE,
    ImdbId TEXT,
    Title TEXT NOT NULL,
    OriginalTitle TEXT,
    CleanTitle TEXT NOT NULL,
    SortTitle TEXT,
    Year INTEGER NOT NULL,
    Status TEXT NOT NULL,
    Overview TEXT,
    Images TEXT NOT NULL,
    Genres TEXT NOT NULL,
    Ratings TEXT NOT NULL,
    Runtime INTEGER NOT NULL,
    InCinemas TEXT,
    PhysicalRelease TEXT,
    DigitalRelease TEXT,
    Certification TEXT,
    Website TEXT,
    YouTubeTrailerId TEXT,
    Studio TEXT,
    Popularity REAL NOT NULL,
    CollectionTmdbId INTEGER NOT NULL DEFAULT 0,
    CollectionTitle TEXT
);

-- Indexes
CREATE INDEX IX_Movies_MovieMetadataId ON Movies(MovieMetadataId);
CREATE INDEX IX_Movies_Path ON Movies(Path);
CREATE INDEX IX_MovieMetadata_TmdbId ON MovieMetadata(TmdbId);
CREATE INDEX IX_MovieMetadata_ImdbId ON MovieMetadata(ImdbId);
CREATE INDEX IX_MovieMetadata_CleanTitle ON MovieMetadata(CleanTitle);
```

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationError>>,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            Error::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            Error::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string(),
            ),
            Error::Serialization(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Serialization error occurred".to_string(),
            ),
            Error::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        
        let body = Json(ApiError {
            message,
            errors: None,
        });
        
        (status, body).into_response()
    }
}
```

## Testing Strategy

### Unit Tests

- Test domain model methods (is_available, apply_changes)
- Test resource mapping (from_movie, to_movie)
- Test validation logic
- Test error handling

### Integration Tests

- Test repository operations against real database
- Test complete CRUD flows through service layer
- Test transaction rollback scenarios
- Test concurrent access patterns

### API Tests

- Test all HTTP endpoints with valid requests
- Test error responses (404, 400, 500)
- Test query parameter handling
- Test JSON serialization/deserialization
- Test API compatibility with existing Radarr frontend

### Test Database Setup

```rust
async fn setup_test_db() -> PgPool {
    let pool = PgPoolOptions::new()
        .connect("postgres://test:test@localhost/radarr_test")
        .await
        .unwrap();
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();
    
    pool
}

#[tokio::test]
async fn test_create_movie() {
    let pool = setup_test_db().await;
    let repo = Arc::new(SqlxMovieRepository::new(pool));
    let service = MovieService::new(repo, Arc::new(Config::default()));
    
    let movie = Movie {
        id: 0,
        movie_metadata_id: 1,
        monitored: true,
        // ... other fields
    };
    
    let created = service.add_movie(movie).await.unwrap();
    assert!(created.id > 0);
}
```

## Performance Considerations

### Database Optimization

- Use connection pooling (sqlx pool)
- Implement prepared statements for common queries
- Add appropriate indexes on lookup columns
- Use batch operations for bulk inserts/updates
- Lazy-load relationships only when needed

### Caching Strategy

- Cache quality profiles (rarely change)
- Cache movie metadata for frequently accessed movies
- Use Arc for shared immutable data
- Consider Redis for distributed caching in future

### Async Operations

- Use tokio for async I/O
- Avoid blocking operations in async contexts
- Use spawn_blocking for CPU-intensive operations
- Implement request timeouts

## Migration Path

### Phase 1: Core Implementation

- Implement domain models
- Implement repository layer with SQLite support
- Implement service layer
- Basic API endpoints

### Phase 2: Feature Parity

- Add PostgreSQL support
- Implement all lookup methods
- Add validation
- Error handling improvements

### Phase 3: Optimization

- Add caching
- Performance tuning
- Load testing
- Memory profiling

### Phase 4: Integration

- Frontend compatibility testing
- Database migration tools
- Deployment documentation
- Monitoring and logging
