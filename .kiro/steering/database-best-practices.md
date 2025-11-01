# Database Best Practices

**All database operations must be safe, efficient, and maintainable using sqlx.**

## Core Principles

1. **Compile-Time Verification** - Use sqlx macros for query checking
2. **Parameterized Queries** - Never concatenate user input into SQL
3. **Transaction Management** - Use transactions for multi-step operations
4. **Connection Pooling** - Reuse connections efficiently
5. **Migration Management** - Version control all schema changes

## sqlx Configuration

### Cargo.toml

```toml
[dependencies]
sqlx = { version = "0.7", features = [
    "runtime-tokio-rustls",
    "sqlite",
    "postgres",
    "migrate",
    "macros",
    "chrono"
] }

[dev-dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite"] }
```

### Environment Setup

```bash
# .env
DATABASE_URL=sqlite:radarr.db
# or
DATABASE_URL=postgresql://user:pass@localhost/radarr

# For compile-time checking
SQLX_OFFLINE=false
```

### Build Configuration

```toml
# .cargo/config.toml
[env]
SQLX_OFFLINE = "true"  # Use offline mode in CI
```

## Query Patterns

### Good: Compile-Time Checked Queries

```rust
// ✅ Good: Type-safe, compile-time verified
pub async fn get_movie(&self, id: i32) -> Result<Movie> {
    sqlx::query_as!(
        Movie,
        r#"
        SELECT 
            m.Id as id,
            m.Path as path,
            m.Monitored as monitored,
            m.QualityProfileId as quality_profile_id,
            m.Added as added,
            mm.TmdbId as tmdb_id,
            mm.Title as title,
            mm.Year as year
        FROM Movies m
        JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id
        WHERE m.Id = ?
        "#,
        id
    )
    .fetch_one(&self.pool)
    .await
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => Error::NotFound(format!("Movie {} not found", id)),
        _ => Error::Database(e.to_string()),
    })
}
```

### Bad: String Concatenation

```rust
// ❌ Bad: SQL injection risk, no type checking
pub async fn get_movie(&self, id: i32) -> Result<Movie> {
    let query = format!("SELECT * FROM Movies WHERE Id = {}", id);
    sqlx::query_as(&query)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from)
}
```

### Good: Parameterized Queries

```rust
// ✅ Good: Safe from SQL injection
pub async fn find_by_title(&self, title: &str) -> Result<Vec<Movie>> {
    sqlx::query_as!(
        Movie,
        r#"
        SELECT m.*, mm.*
        FROM Movies m
        JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id
        WHERE mm.Title LIKE ?
        "#,
        format!("%{}%", title)
    )
    .fetch_all(&self.pool)
    .await
    .map_err(|e| Error::Database(e.to_string()))
}
```

## Transactions

### Use Transactions for Multi-Step Operations

```rust
// ✅ Good: Atomic operation
pub async fn add_movie_with_metadata(
    &self,
    metadata: MovieMetadata,
    movie: Movie,
) -> Result<Movie> {
    let mut tx = self.pool.begin().await?;
    
    // Insert metadata first
    let metadata_id = sqlx::query!(
        "INSERT INTO MovieMetadata (TmdbId, Title, Year) VALUES (?, ?, ?)",
        metadata.tmdb_id,
        metadata.title,
        metadata.year
    )
    .execute(&mut *tx)
    .await?
    .last_insert_rowid();
    
    // Insert movie with metadata reference
    let movie_id = sqlx::query!(
        "INSERT INTO Movies (MovieMetadataId, Path, Monitored, QualityProfileId, Added) 
         VALUES (?, ?, ?, ?, ?)",
        metadata_id,
        movie.path,
        movie.monitored,
        movie.quality_profile_id,
        movie.added
    )
    .execute(&mut *tx)
    .await?
    .last_insert_rowid();
    
    // Commit transaction
    tx.commit().await?;
    
    // Return created movie
    self.get(movie_id as i32).await
}
```

### Transaction Error Handling

```rust
pub async fn update_multiple_movies(&self, updates: Vec<MovieUpdate>) -> Result<()> {
    let mut tx = self.pool.begin().await?;
    
    for update in updates {
        let result = sqlx::query!(
            "UPDATE Movies SET Monitored = ? WHERE Id = ?",
            update.monitored,
            update.id
        )
        .execute(&mut *tx)
        .await;
        
        if let Err(e) = result {
            // Transaction automatically rolls back on drop
            return Err(Error::Database(e.to_string()));
        }
    }
    
    tx.commit().await?;
    Ok(())
}
```

## Migrations

### Directory Structure

```
migrations/
├── 20240101000001_create_movies_table.sql
├── 20240101000002_create_quality_profiles_table.sql
├── 20240101000003_add_movie_indexes.sql
└── 20240101000004_add_movie_metadata_table.sql
```

### Migration File Format

```sql
-- migrations/20240101000001_create_movies_table.sql
-- Create Movies table
CREATE TABLE IF NOT EXISTS Movies (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    MovieMetadataId INTEGER NOT NULL,
    Path TEXT NOT NULL,
    Monitored INTEGER NOT NULL DEFAULT 1,
    QualityProfileId INTEGER NOT NULL,
    Added TEXT NOT NULL,
    FOREIGN KEY (MovieMetadataId) REFERENCES MovieMetadata(Id),
    FOREIGN KEY (QualityProfileId) REFERENCES QualityProfiles(Id)
);

-- Create indexes
CREATE INDEX IF NOT EXISTS IX_Movies_MovieMetadataId ON Movies(MovieMetadataId);
CREATE INDEX IF NOT EXISTS IX_Movies_QualityProfileId ON Movies(QualityProfileId);
CREATE UNIQUE INDEX IF NOT EXISTS IX_Movies_Path ON Movies(Path);
```

### Running Migrations

```rust
// In main.rs or setup
pub async fn run_migrations(pool: &Pool<Sqlite>) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| Error::Migration(e.to_string()))?;
    Ok(())
}
```

### Migration Best Practices

- **One change per migration** - Keep migrations focused
- **Reversible when possible** - Consider rollback scenarios
- **Test migrations** - Test both up and down migrations
- **Never modify existing migrations** - Create new ones instead
- **Use timestamps** - Name migrations with timestamp prefix
- **Document breaking changes** - Add comments for major changes

## Connection Pooling

### Pool Configuration

```rust
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::Duration;

pub async fn create_pool(database_url: &str) -> Result<SqlitePool> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
        .map_err(|e| Error::Database(e.to_string()))
}
```

### Pool Usage

```rust
// ✅ Good: Pass pool reference
pub struct MovieRepository {
    pool: SqlitePool,
}

impl MovieRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
    
    pub async fn get(&self, id: i32) -> Result<Movie> {
        // Pool automatically manages connections
        sqlx::query_as!(Movie, "SELECT * FROM Movies WHERE Id = ?", id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::from(e))
    }
}
```

## Error Handling

### Map Database Errors Appropriately

```rust
pub async fn get_movie(&self, id: i32) -> Result<Movie> {
    sqlx::query_as!(Movie, "SELECT * FROM Movies WHERE Id = ?", id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                Error::NotFound(format!("Movie with ID {} not found", id))
            }
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    Error::Conflict("Movie already exists".to_string())
                } else if db_err.is_foreign_key_violation() {
                    Error::Validation("Invalid foreign key reference".to_string())
                } else {
                    Error::Database(db_err.to_string())
                }
            }
            _ => Error::Database(e.to_string()),
        })
}
```

### Constraint Violations

```rust
pub async fn insert_movie(&self, movie: &Movie) -> Result<Movie> {
    let result = sqlx::query!(
        "INSERT INTO Movies (Path, Monitored, QualityProfileId, Added) 
         VALUES (?, ?, ?, ?)",
        movie.path,
        movie.monitored,
        movie.quality_profile_id,
        movie.added
    )
    .execute(&self.pool)
    .await;
    
    match result {
        Ok(result) => {
            let id = result.last_insert_rowid() as i32;
            self.get(id).await
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(Error::Conflict(format!("Movie with path '{}' already exists", movie.path)))
            } else if db_err.is_foreign_key_violation() {
                Err(Error::Validation("Invalid quality profile ID".to_string()))
            } else {
                Err(Error::Database(db_err.to_string()))
            }
        }
        Err(e) => Err(Error::Database(e.to_string())),
    }
}
```

## Type Mapping

### Custom Type Implementations

```rust
use sqlx::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "TEXT")]
#[serde(rename_all = "camelCase")]
pub enum MovieStatusType {
    #[serde(rename = "tba")]
    TBA,
    Announced,
    InCinemas,
    Released,
}

// sqlx will automatically handle conversion
```

### JSON Columns

```rust
use sqlx::types::Json;

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieRatings {
    pub imdb: Option<Rating>,
    pub tmdb: Option<Rating>,
    pub rotten_tomatoes: Option<Rating>,
}

pub async fn get_movie_with_ratings(&self, id: i32) -> Result<Movie> {
    sqlx::query_as!(
        Movie,
        r#"
        SELECT 
            Id as id,
            Ratings as "ratings: Json<MovieRatings>"
        FROM Movies
        WHERE Id = ?
        "#,
        id
    )
    .fetch_one(&self.pool)
    .await
    .map_err(|e| Error::from(e))
}
```

## Query Optimization

### Use Indexes

```sql
-- Add indexes for frequently queried columns
CREATE INDEX IX_Movies_TmdbId ON Movies(TmdbId);
CREATE INDEX IX_Movies_Monitored ON Movies(Monitored);
CREATE INDEX IX_Movies_QualityProfileId ON Movies(QualityProfileId);
```

### Avoid N+1 Queries

```rust
// ❌ Bad: N+1 query problem
pub async fn get_movies_with_metadata(&self) -> Result<Vec<MovieWithMetadata>> {
    let movies = self.get_all_movies().await?;
    
    let mut result = Vec::new();
    for movie in movies {
        // This executes a query for EACH movie
        let metadata = self.get_metadata(movie.metadata_id).await?;
        result.push(MovieWithMetadata { movie, metadata });
    }
    
    Ok(result)
}

// ✅ Good: Single query with JOIN
pub async fn get_movies_with_metadata(&self) -> Result<Vec<MovieWithMetadata>> {
    sqlx::query_as!(
        MovieWithMetadata,
        r#"
        SELECT 
            m.*,
            mm.*
        FROM Movies m
        JOIN MovieMetadata mm ON m.MovieMetadataId = mm.Id
        "#
    )
    .fetch_all(&self.pool)
    .await
    .map_err(|e| Error::from(e))
}
```

### Pagination

```rust
pub async fn get_movies_paginated(
    &self,
    page: i32,
    page_size: i32,
) -> Result<Vec<Movie>> {
    let offset = (page - 1) * page_size;
    
    sqlx::query_as!(
        Movie,
        r#"
        SELECT * FROM Movies
        ORDER BY Added DESC
        LIMIT ? OFFSET ?
        "#,
        page_size,
        offset
    )
    .fetch_all(&self.pool)
    .await
    .map_err(|e| Error::from(e))
}
```

## Testing with Databases

### In-Memory Database for Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    
    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:")
            .await
            .expect("Failed to create test database");
        
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
        
        pool
    }
    
    #[tokio::test]
    async fn test_insert_movie() {
        let pool = setup_test_db().await;
        let repo = MovieRepository::new(pool);
        
        let movie = Movie {
            path: "/movies/test".to_string(),
            monitored: true,
            quality_profile_id: 1,
            added: Utc::now(),
            ..Default::default()
        };
        
        let result = repo.insert(&movie).await;
        assert!(result.is_ok());
    }
}
```

### Test Data Cleanup

```rust
#[tokio::test]
async fn test_with_cleanup() {
    let pool = setup_test_db().await;
    let repo = MovieRepository::new(pool.clone());
    
    // Test operations
    let movie = create_test_movie();
    repo.insert(&movie).await.unwrap();
    
    // Cleanup
    sqlx::query!("DELETE FROM Movies")
        .execute(&pool)
        .await
        .unwrap();
}
```

## Offline Mode (sqlx-cli)

### Prepare Queries for Offline Compilation

```bash
# Install sqlx-cli
cargo install sqlx-cli

# Create .env with DATABASE_URL
echo "DATABASE_URL=sqlite:radarr.db" > .env

# Prepare queries (generates sqlx-data.json)
cargo sqlx prepare

# Commit sqlx-data.json to version control
git add sqlx-data.json
```

### CI/CD with Offline Mode

```yaml
# .github/workflows/ci.yml
- name: Check sqlx queries
  run: |
    cargo sqlx prepare --check
```

## Database-Specific Considerations

### SQLite

```rust
// Enable foreign keys (required for SQLite)
sqlx::query("PRAGMA foreign_keys = ON")
    .execute(&pool)
    .await?;

// Enable WAL mode for better concurrency
sqlx::query("PRAGMA journal_mode = WAL")
    .execute(&pool)
    .await?;
```

### PostgreSQL

```rust
// Use PostgreSQL-specific features
sqlx::query!(
    r#"
    INSERT INTO Movies (Path, Monitored, QualityProfileId, Added)
    VALUES ($1, $2, $3, $4)
    RETURNING Id
    "#,
    movie.path,
    movie.monitored,
    movie.quality_profile_id,
    movie.added
)
.fetch_one(&pool)
.await?;
```

## Checklist

Before committing database code:

- [ ] All queries use parameterized inputs
- [ ] Queries are compile-time checked with sqlx macros
- [ ] Transactions used for multi-step operations
- [ ] Proper error handling and mapping
- [ ] Indexes added for frequently queried columns
- [ ] Migrations are versioned and tested
- [ ] Connection pool properly configured
- [ ] Tests use in-memory database
- [ ] Foreign key constraints enabled (SQLite)
- [ ] sqlx-data.json updated for offline mode

## Remember

- **Never concatenate user input into SQL** - Always use parameters
- **Use transactions for consistency** - Multi-step operations should be atomic
- **Leverage compile-time checking** - sqlx macros catch errors early
- **Index strategically** - Add indexes for query performance
- **Test with real database** - In-memory SQLite for fast tests
- **Version control migrations** - Never modify existing migrations
- **Handle errors appropriately** - Map database errors to domain errors
- **Use connection pooling** - Don't create connections per request

## Common Pitfalls

### ❌ Don't

```rust
// SQL injection risk
let query = format!("SELECT * FROM Movies WHERE Title = '{}'", title);

// No error handling
let movie = sqlx::query_as("SELECT * FROM Movies WHERE Id = ?")
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap(); // Don't unwrap!

// Missing transaction
async fn transfer_movie(from_id: i32, to_id: i32) {
    delete_movie(from_id).await?; // What if this fails?
    create_movie(to_id).await?;   // Inconsistent state!
}
```

### ✅ Do

```rust
// Parameterized query
sqlx::query_as!(Movie, "SELECT * FROM Movies WHERE Title = ?", title)

// Proper error handling
let movie = sqlx::query_as!(Movie, "SELECT * FROM Movies WHERE Id = ?", id)
    .fetch_one(&pool)
    .await
    .map_err(|e| Error::from(e))?;

// Use transaction
async fn transfer_movie(from_id: i32, to_id: i32) -> Result<()> {
    let mut tx = pool.begin().await?;
    delete_movie(&mut tx, from_id).await?;
    create_movie(&mut tx, to_id).await?;
    tx.commit().await?;
    Ok(())
}
```
