# Testing Strategy

**All code must be thoroughly tested with unit tests, integration tests, and compatibility verification.**

## Core Principles

1. **Test-Driven Development** - Write tests alongside implementation
2. **Comprehensive Coverage** - Aim for >80% code coverage
3. **Test All Paths** - Happy path, error cases, edge cases
4. **API Compatibility** - Verify responses match original Radarr
5. **Fast Feedback** - Tests should run quickly

## Test Types

### Unit Tests

Test individual functions and methods in isolation.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movie_status_is_available() {
        let status = MovieStatusType::Released;
        assert!(status.is_available());
        
        let status = MovieStatusType::TBA;
        assert!(!status.is_available());
    }
    
    #[tokio::test]
    async fn test_validate_movie_path() {
        let movie = Movie {
            path: String::new(),
            root_folder_path: None,
            ..Default::default()
        };
        
        let result = validate_movie(&movie);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Validation(_)));
    }
}
```

### Integration Tests

Test complete workflows with real database.

```rust
// tests/integration/movie_crud.rs
use radarr_api::*;
use sqlx::SqlitePool;

#[tokio::test]
async fn test_create_and_get_movie() {
    // Setup test database
    let pool = setup_test_db().await;
    let app = create_test_app(pool.clone()).await;
    
    // Create movie
    let create_request = json!({
        "title": "Inception",
        "year": 2010,
        "tmdbId": 27205,
        "qualityProfileId": 1,
        "rootFolderPath": "/movies",
        "monitored": true
    });
    
    let response = app
        .post("/api/v3/movie")
        .json(&create_request)
        .send()
        .await;
    
    assert_eq!(response.status(), 201);
    let created: MovieResource = response.json().await;
    assert_eq!(created.title, "Inception");
    assert!(created.id > 0);
    
    // Get movie
    let response = app
        .get(&format!("/api/v3/movie/{}", created.id))
        .send()
        .await;
    
    assert_eq!(response.status(), 200);
    let fetched: MovieResource = response.json().await;
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.title, "Inception");
    
    // Cleanup
    cleanup_test_db(pool).await;
}
```

### Compatibility Tests

Verify API responses match original Radarr format.

```rust
#[tokio::test]
async fn test_movie_response_format_matches_radarr() {
    let movie = create_test_movie().await;
    let resource = MovieResource::from_movie(movie);
    let json = serde_json::to_value(&resource).unwrap();
    
    // Verify all required fields exist
    assert!(json.get("id").is_some());
    assert!(json.get("title").is_some());
    assert!(json.get("year").is_some());
    assert!(json.get("tmdbId").is_some());
    assert!(json.get("qualityProfileId").is_some());
    assert!(json.get("monitored").is_some());
    assert!(json.get("path").is_some());
    assert!(json.get("added").is_some());
    
    // Verify field types
    assert!(json["id"].is_number());
    assert!(json["title"].is_string());
    assert!(json["year"].is_number());
    assert!(json["monitored"].is_boolean());
    
    // Verify camelCase naming
    assert!(json.get("qualityProfileId").is_some());
    assert!(json.get("quality_profile_id").is_none());
}
```

### Error Case Tests

Test all error scenarios.

```rust
#[tokio::test]
async fn test_create_movie_duplicate_tmdb_id() {
    let app = setup_test_app().await;
    
    let movie = json!({
        "title": "Inception",
        "tmdbId": 27205,
        "qualityProfileId": 1,
        "rootFolderPath": "/movies"
    });
    
    // Create first movie
    let response = app.post("/api/v3/movie").json(&movie).send().await;
    assert_eq!(response.status(), 201);
    
    // Try to create duplicate
    let response = app.post("/api/v3/movie").json(&movie).send().await;
    assert_eq!(response.status(), 400);
    
    let error: ErrorResponse = response.json().await;
    assert!(error.message.contains("already exists"));
}

#[tokio::test]
async fn test_get_movie_not_found() {
    let app = setup_test_app().await;
    
    let response = app.get("/api/v3/movie/99999").send().await;
    assert_eq!(response.status(), 404);
    
    let error: ErrorResponse = response.json().await;
    assert!(error.message.contains("not found"));
}

#[tokio::test]
async fn test_create_movie_invalid_quality_profile() {
    let app = setup_test_app().await;
    
    let movie = json!({
        "title": "Inception",
        "tmdbId": 27205,
        "qualityProfileId": 99999, // Non-existent
        "rootFolderPath": "/movies"
    });
    
    let response = app.post("/api/v3/movie").json(&movie).send().await;
    assert_eq!(response.status(), 400);
}
```

### Edge Case Tests

Test boundary conditions and unusual inputs.

```rust
#[tokio::test]
async fn test_movie_with_special_characters() {
    let movie = json!({
        "title": "Movie: The \"Special\" Edition (2024)",
        "tmdbId": 12345,
        "qualityProfileId": 1,
        "rootFolderPath": "/movies"
    });
    
    let response = app.post("/api/v3/movie").json(&movie).send().await;
    assert_eq!(response.status(), 201);
}

#[tokio::test]
async fn test_movie_with_very_long_title() {
    let long_title = "A".repeat(500);
    let movie = json!({
        "title": long_title,
        "tmdbId": 12345,
        "qualityProfileId": 1,
        "rootFolderPath": "/movies"
    });
    
    let response = app.post("/api/v3/movie").json(&movie).send().await;
    // Should either accept or return validation error
    assert!(response.status() == 201 || response.status() == 400);
}

#[test]
fn test_movie_status_all_variants() {
    // Test all enum variants
    let statuses = vec![
        MovieStatusType::TBA,
        MovieStatusType::Announced,
        MovieStatusType::InCinemas,
        MovieStatusType::Released,
    ];
    
    for status in statuses {
        let json = serde_json::to_string(&status).unwrap();
        let deserialized: MovieStatusType = serde_json::from_str(&json).unwrap();
        assert_eq!(status, deserialized);
    }
}
```

## Test Organization

### Directory Structure

```
radarr_core/
├── src/
│   ├── domain/
│   │   ├── movie.rs
│   │   └── tests/          # Unit tests for domain
│   ├── repository/
│   │   ├── movie_repository.rs
│   │   └── tests/          # Unit tests for repository
│   └── service/
│       ├── movie_service.rs
│       └── tests/          # Unit tests for service

radarr_api/
├── src/
│   ├── handlers/
│   │   └── tests/          # Unit tests for handlers
│   └── resources/
│       └── tests/          # Unit tests for resources
└── tests/
    └── integration/        # Integration tests
        ├── movie_crud.rs
        ├── quality_profile.rs
        └── common/
            └── mod.rs      # Test utilities
```

### Test Utilities

Create reusable test helpers:

```rust
// tests/common/mod.rs
use sqlx::SqlitePool;

pub async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    
    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .unwrap();
    
    // Seed test data
    seed_quality_profiles(&pool).await;
    seed_root_folders(&pool).await;
    
    pool
}

pub async fn cleanup_test_db(pool: SqlitePool) {
    pool.close().await;
}

pub fn create_test_movie() -> Movie {
    Movie {
        id: 0,
        title: "Test Movie".to_string(),
        year: 2024,
        tmdb_id: 12345,
        quality_profile_id: 1,
        path: "/movies/Test Movie (2024)".to_string(),
        monitored: true,
        added: Utc::now(),
        ..Default::default()
    }
}

pub async fn seed_quality_profiles(pool: &SqlitePool) {
    sqlx::query("INSERT INTO QualityProfiles (Id, Name) VALUES (1, 'HD-1080p')")
        .execute(pool)
        .await
        .unwrap();
}
```

## Mock Implementations

Use mocks for testing in isolation:

```rust
#[cfg(test)]
pub struct MockMovieRepository {
    movies: Arc<Mutex<HashMap<i32, Movie>>>,
    next_id: Arc<Mutex<i32>>,
}

#[cfg(test)]
impl MockMovieRepository {
    pub fn new() -> Self {
        Self {
            movies: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
    
    pub fn with_movies(movies: Vec<Movie>) -> Self {
        let mut map = HashMap::new();
        let mut max_id = 0;
        
        for movie in movies {
            if movie.id > max_id {
                max_id = movie.id;
            }
            map.insert(movie.id, movie);
        }
        
        Self {
            movies: Arc::new(Mutex::new(map)),
            next_id: Arc::new(Mutex::new(max_id + 1)),
        }
    }
}

#[cfg(test)]
#[async_trait]
impl MovieRepository for MockMovieRepository {
    async fn get(&self, id: i32) -> Result<Movie> {
        self.movies
            .lock()
            .await
            .get(&id)
            .cloned()
            .ok_or(Error::NotFound(format!("Movie {} not found", id)))
    }
    
    async fn insert(&self, movie: &Movie) -> Result<Movie> {
        let mut movies = self.movies.lock().await;
        let mut next_id = self.next_id.lock().await;
        
        let mut new_movie = movie.clone();
        new_movie.id = *next_id;
        *next_id += 1;
        
        movies.insert(new_movie.id, new_movie.clone());
        Ok(new_movie)
    }
    
    async fn all(&self) -> Result<Vec<Movie>> {
        Ok(self.movies.lock().await.values().cloned().collect())
    }
}
```

## Running Tests

### All Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_create_movie

# Run tests in specific module
cargo test movie_crud
```

### Unit Tests Only

```bash
# Run only unit tests (in src/)
cargo test --lib
```

### Integration Tests Only

```bash
# Run only integration tests (in tests/)
cargo test --test '*'

# Run specific integration test file
cargo test --test movie_crud
```

### With Coverage

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage

# Open coverage report
open coverage/index.html
```

## Test Quality Standards

### Every Test Must

- [ ] Have a descriptive name explaining what it tests
- [ ] Test one specific behavior
- [ ] Be independent (no shared state)
- [ ] Be deterministic (same result every time)
- [ ] Clean up after itself
- [ ] Run quickly (<1 second for unit tests)
- [ ] Have clear assertions with helpful messages

### Bad Test Example

```rust
#[tokio::test]
async fn test_movie() {
    let movie = Movie::default();
    assert!(movie.id == 0);
    // What are we testing? Why does this matter?
}
```

### Good Test Example

```rust
#[tokio::test]
async fn test_new_movie_has_zero_id_before_persistence() {
    // Arrange
    let movie = Movie::default();
    
    // Act
    let id = movie.id;
    
    // Assert
    assert_eq!(
        id, 0,
        "New movie should have ID of 0 before being saved to database"
    );
}
```

## Continuous Testing

### Pre-commit Hook

```bash
#!/bin/bash
echo "Running tests..."
cargo test --quiet
if [ $? -ne 0 ]; then
    echo "❌ Tests failed. Fix them before committing."
    exit 1
fi
```

### CI/CD Pipeline

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test --all-features
      
      - name: Generate coverage
        run: |
          cargo install cargo-tarpaulin
          cargo tarpaulin --out Xml
      
      - name: Upload coverage
        uses: codecov/codecov-action@v3
```

## Test Checklist

Before merging any code:

- [ ] All new code has unit tests
- [ ] All public APIs have integration tests
- [ ] All error cases are tested
- [ ] Edge cases are covered
- [ ] Tests are passing locally
- [ ] Tests pass in CI
- [ ] Coverage is >80%
- [ ] No flaky tests
- [ ] Test names are descriptive
- [ ] Mock implementations are used appropriately

## Remember

- **Tests are documentation** - They show how code should be used
- **Test behavior, not implementation** - Tests should survive refactoring
- **Fast tests = happy developers** - Keep tests quick
- **Failing tests are good** - They catch bugs before production
- **Coverage is a guide, not a goal** - 100% coverage doesn't mean bug-free
- **Write tests as you code** - Don't leave them for later

## Common Testing Patterns

### Arrange-Act-Assert

```rust
#[test]
fn test_example() {
    // Arrange - Set up test data
    let input = "test";
    
    // Act - Execute the code under test
    let result = function_under_test(input);
    
    // Assert - Verify the result
    assert_eq!(result, expected);
}
```

### Table-Driven Tests

```rust
#[test]
fn test_multiple_cases() {
    let test_cases = vec![
        ("input1", "expected1"),
        ("input2", "expected2"),
        ("input3", "expected3"),
    ];
    
    for (input, expected) in test_cases {
        let result = function_under_test(input);
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}
```

### Async Test Helpers

```rust
#[tokio::test]
async fn test_async_operation() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```
