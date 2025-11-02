---
inclusion: always
---

# DRY Principle (Don't Repeat Yourself)

**Every piece of knowledge must have a single, unambiguous, authoritative representation within a system.**

## Core Concepts

### What is DRY?
- Avoid duplicating logic, data structures, or algorithms
- Extract common patterns into reusable abstractions
- Single source of truth for each concept
- Changes should only need to happen in one place

### What DRY is NOT
- Not about avoiding any code similarity
- Not about premature abstraction
- Not about making everything generic
- Not about eliminating all repetition

## Code Duplication

### Bad: Duplicated Logic
```rust
// Bad: Same validation logic repeated
pub async fn create_movie(movie: Movie) -> Result<Movie> {
    if movie.path.is_empty() && movie.root_folder_path.is_none() {
        return Err(Error::Validation("Path or root folder required".into()));
    }
    if movie.quality_profile_id <= 0 {
        return Err(Error::Validation("Invalid quality profile".into()));
    }
    // ... insert logic
}

pub async fn update_movie(movie: Movie) -> Result<Movie> {
    if movie.path.is_empty() && movie.root_folder_path.is_none() {
        return Err(Error::Validation("Path or root folder required".into()));
    }
    if movie.quality_profile_id <= 0 {
        return Err(Error::Validation("Invalid quality profile".into()));
    }
    // ... update logic
}
```

### Good: Extract Common Logic
```rust
// Good: Single validation function
fn validate_movie(movie: &Movie) -> Result<()> {
    if movie.path.is_empty() && movie.root_folder_path.is_none() {
        return Err(Error::Validation("Path or root folder required".into()));
    }
    if movie.quality_profile_id <= 0 {
        return Err(Error::Validation("Invalid quality profile".into()));
    }
    Ok(())
}

pub async fn create_movie(movie: Movie) -> Result<Movie> {
    validate_movie(&movie)?;
    // ... insert logic
}

pub async fn update_movie(movie: Movie) -> Result<Movie> {
    validate_movie(&movie)?;
    // ... update logic
}
```

## Data Structure Duplication

### Bad: Repeated Structures
```rust
// Bad: Same fields in multiple structs
pub struct CreateMovieRequest {
    pub title: String,
    pub year: i32,
    pub tmdb_id: i32,
    pub path: String,
    pub quality_profile_id: i32,
    pub monitored: bool,
}

pub struct UpdateMovieRequest {
    pub title: String,
    pub year: i32,
    pub tmdb_id: i32,
    pub path: String,
    pub quality_profile_id: i32,
    pub monitored: bool,
}

pub struct MovieResponse {
    pub title: String,
    pub year: i32,
    pub tmdb_id: i32,
    pub path: String,
    pub quality_profile_id: i32,
    pub monitored: bool,
}
```

### Good: Shared Base Structure
```rust
// Good: Single source of truth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieData {
    pub title: String,
    pub year: i32,
    pub tmdb_id: i32,
    pub path: String,
    pub quality_profile_id: i32,
    pub monitored: bool,
}

pub struct CreateMovieRequest {
    #[serde(flatten)]
    pub movie: MovieData,
}

pub struct UpdateMovieRequest {
    pub id: i32,
    #[serde(flatten)]
    pub movie: MovieData,
}

pub struct MovieResponse {
    pub id: i32,
    #[serde(flatten)]
    pub movie: MovieData,
    pub added: DateTime<Utc>,
}
```

## Query Duplication

### Bad: Repeated SQL
```rust
// Bad: Same query pattern repeated
impl MovieRepository {
    pub async fn get(&self, id: i32) -> Result<Movie> {
        sqlx::query_as::<_, Movie>(
            "SELECT m.*, mm.* FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id 
             WHERE m.Id = ?"
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
    
    pub async fn find_by_path(&self, path: &str) -> Result<Option<Movie>> {
        sqlx::query_as::<_, Movie>(
            "SELECT m.*, mm.* FROM Movies m 
             JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id 
             WHERE m.Path = ?"
        )
        .bind(path)
        .fetch_optional(&self.pool)
        .await
    }
}
```

### Good: Query Builder
```rust
// Good: Reusable query builder
impl MovieRepository {
    fn base_query() -> &'static str {
        "SELECT m.*, mm.* FROM Movies m 
         JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id"
    }
    
    pub async fn get(&self, id: i32) -> Result<Movie> {
        let query = format!("{} WHERE m.Id = ?", Self::base_query());
        sqlx::query_as::<_, Movie>(&query)
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }
    
    pub async fn find_by_path(&self, path: &str) -> Result<Option<Movie>> {
        let query = format!("{} WHERE m.Path = ?", Self::base_query());
        sqlx::query_as::<_, Movie>(&query)
            .bind(path)
            .fetch_optional(&self.pool)
            .await
    }
}
```

## Error Handling Duplication

### Bad: Repeated Error Mapping
```rust
// Bad: Same error mapping everywhere
pub async fn get_movie(&self, id: i32) -> Result<Movie> {
    self.repository.get(id).await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::NotFound(format!("Movie {} not found", id)),
            _ => Error::Database(e.into()),
        })
}

pub async fn get_quality_profile(&self, id: i32) -> Result<QualityProfile> {
    self.repository.get(id).await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::NotFound(format!("Profile {} not found", id)),
            _ => Error::Database(e.into()),
        })
}
```

### Good: Generic Error Mapper
```rust
// Good: Reusable error mapper
trait MapNotFound<T> {
    fn map_not_found(self, entity: &str, id: i32) -> Result<T>;
}

impl<T> MapNotFound<T> for Result<T, sqlx::Error> {
    fn map_not_found(self, entity: &str, id: i32) -> Result<T> {
        self.map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                Error::NotFound(format!("{} {} not found", entity, id))
            }
            _ => Error::Database(e.into()),
        })
    }
}

pub async fn get_movie(&self, id: i32) -> Result<Movie> {
    self.repository.get(id).await
        .map_not_found("Movie", id)
}

pub async fn get_quality_profile(&self, id: i32) -> Result<QualityProfile> {
    self.repository.get(id).await
        .map_not_found("QualityProfile", id)
}
```

## Conversion Duplication

### Bad: Repeated Conversions
```rust
// Bad: Same conversion logic in multiple places
impl MovieResource {
    pub fn from_movie(movie: Movie) -> Self {
        Self {
            id: movie.id,
            title: movie.movie_metadata.as_ref().map(|m| m.title.clone()).unwrap_or_default(),
            year: movie.movie_metadata.as_ref().map(|m| m.year).unwrap_or(0),
            // ... 20 more fields
        }
    }
}

impl MovieListItem {
    pub fn from_movie(movie: Movie) -> Self {
        Self {
            id: movie.id,
            title: movie.movie_metadata.as_ref().map(|m| m.title.clone()).unwrap_or_default(),
            year: movie.movie_metadata.as_ref().map(|m| m.year).unwrap_or(0),
            // ... same fields again
        }
    }
}
```

### Good: Shared Conversion Trait
```rust
// Good: Implement From trait once
impl From<Movie> for MovieData {
    fn from(movie: Movie) -> Self {
        Self {
            id: movie.id,
            title: movie.movie_metadata.as_ref().map(|m| m.title.clone()).unwrap_or_default(),
            year: movie.movie_metadata.as_ref().map(|m| m.year).unwrap_or(0),
            // ... all fields
        }
    }
}

impl MovieResource {
    pub fn from_movie(movie: Movie) -> Self {
        let data = MovieData::from(movie.clone());
        Self {
            data,
            statistics: None, // Resource-specific field
        }
    }
}

impl MovieListItem {
    pub fn from_movie(movie: Movie) -> Self {
        let data = MovieData::from(movie);
        Self { data }
    }
}
```

## Configuration Duplication

### Bad: Hardcoded Values
```rust
// Bad: Magic numbers everywhere
pub async fn get_movies(&self) -> Result<Vec<Movie>> {
    let movies = self.repository.all().await?;
    if movies.len() > 1000 {
        return Err(Error::TooManyResults);
    }
    Ok(movies)
}

pub async fn get_quality_profiles(&self) -> Result<Vec<QualityProfile>> {
    let profiles = self.repository.all().await?;
    if profiles.len() > 1000 {
        return Err(Error::TooManyResults);
    }
    Ok(profiles)
}
```

### Good: Centralized Configuration
```rust
// Good: Single source of configuration
pub struct Config {
    pub max_results: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_results: 1000,
        }
    }
}

pub struct MovieService {
    repository: Arc<dyn MovieRepository>,
    config: Arc<Config>,
}

impl MovieService {
    pub async fn get_movies(&self) -> Result<Vec<Movie>> {
        let movies = self.repository.all().await?;
        if movies.len() > self.config.max_results {
            return Err(Error::TooManyResults);
        }
        Ok(movies)
    }
}
```

## When NOT to Apply DRY

### Acceptable Repetition

#### 1. Different Domains
```rust
// OK: Similar but semantically different
pub struct Movie {
    pub id: i32,
    pub title: String,
}

pub struct QualityProfile {
    pub id: i32,
    pub name: String, // Not "title" - different concept
}
```

#### 2. Coincidental Similarity
```rust
// OK: Happens to look similar now, but will diverge
pub fn validate_movie_path(path: &str) -> Result<()> {
    if path.is_empty() {
        return Err(Error::EmptyPath);
    }
    Ok(())
}

pub fn validate_config_path(path: &str) -> Result<()> {
    if path.is_empty() {
        return Err(Error::EmptyPath);
    }
    Ok(())
}
// These will likely have different validation rules later
```

#### 3. Test Data
```rust
// OK: Test data can be duplicated for clarity
#[test]
fn test_create_movie() {
    let movie = Movie {
        id: 1,
        title: "Inception".to_string(),
        year: 2010,
    };
    // ...
}

#[test]
fn test_update_movie() {
    let movie = Movie {
        id: 1,
        title: "Inception".to_string(),
        year: 2010,
    };
    // ...
}
```

## DRY Checklist

Before writing code:
- [ ] Is this logic already implemented elsewhere?
- [ ] Can this be extracted into a reusable function?
- [ ] Is this data structure duplicated?
- [ ] Are these queries following the same pattern?
- [ ] Is this configuration value hardcoded multiple times?

Before extracting:
- [ ] Will these implementations diverge in the future?
- [ ] Are they in the same domain?
- [ ] Will the abstraction make the code clearer?
- [ ] Is this premature optimization?

## Tools and Techniques

### Macros for Repetitive Code
```rust
// Use macros for boilerplate
macro_rules! impl_repository_get {
    ($type:ty, $table:expr) => {
        async fn get(&self, id: i32) -> Result<$type> {
            sqlx::query_as::<_, $type>(
                &format!("SELECT * FROM {} WHERE Id = ?", $table)
            )
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::from(e))
        }
    };
}
```

### Trait Implementations
```rust
// Use traits for common behavior
pub trait Entity {
    fn id(&self) -> i32;
    fn validate(&self) -> Result<()>;
}

impl Entity for Movie {
    fn id(&self) -> i32 { self.id }
    fn validate(&self) -> Result<()> { /* ... */ }
}
```

### Generic Functions
```rust
// Use generics for type-agnostic logic
pub async fn get_by_id<T>(
    pool: &PgPool,
    table: &str,
    id: i32,
) -> Result<T>
where
    T: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    sqlx::query_as(&format!("SELECT * FROM {} WHERE Id = ?", table))
        .bind(id)
        .fetch_one(pool)
        .await
        .map_err(Error::from)
}
```

## Remember

- **DRY is about knowledge, not code**
- **Duplication is cheaper than wrong abstraction**
- **Extract when you see the pattern three times (Rule of Three)**
- **Keep abstractions simple and focused**
- **Document why abstractions exist**
