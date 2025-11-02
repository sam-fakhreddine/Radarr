---
inclusion: always
---

# SOLID Principles in Rust

## Single Responsibility Principle (SRP)

**Each module, struct, or function should have one reason to change.**

### Good Example

```rust
// Good: Separate concerns
pub struct MovieRepository {
    pool: PgPool,
}

impl MovieRepository {
    // Only responsible for data access
    pub async fn get(&self, id: i32) -> Result<Movie> { ... }
}

pub struct MovieService {
    repository: Arc<dyn MovieRepository>,
}

impl MovieService {
    // Only responsible for business logic
    pub async fn add_movie(&self, movie: Movie) -> Result<Movie> { ... }
}

pub async fn create_movie_handler(
    State(service): State<Arc<MovieService>>,
    Json(resource): Json<MovieResource>,
) -> Result<Json<MovieResource>> {
    // Only responsible for HTTP handling
    let movie = resource.to_movie();
    let created = service.add_movie(movie).await?;
    Ok(Json(MovieResource::from_movie(created)))
}
```

### Bad Example

```rust
// Bad: Repository doing business logic AND data access
impl MovieRepository {
    pub async fn add_movie(&self, movie: Movie) -> Result<Movie> {
        // Business logic mixed with data access
        if self.find_by_tmdb_id(movie.tmdb_id).await?.is_some() {
            return Err(Error::AlreadyExists);
        }
        
        movie.added = Utc::now(); // Business logic
        
        self.insert(movie).await // Data access
    }
}
```

### Application

- Repository: Only database operations
- Service: Only business logic
- Handler: Only HTTP request/response
- Resource: Only data transfer
- Validator: Only validation logic

## Open/Closed Principle (OCP)

**Software entities should be open for extension but closed for modification.**

### Good Example

```rust
// Good: Use traits for extension
#[async_trait]
pub trait MovieRepository: Send + Sync {
    async fn get(&self, id: i32) -> Result<Movie>;
    async fn insert(&self, movie: &Movie) -> Result<Movie>;
}

// Extend with new implementation without modifying trait
pub struct SqliteMovieRepository {
    pool: SqlitePool,
}

#[async_trait]
impl MovieRepository for SqliteMovieRepository {
    async fn get(&self, id: i32) -> Result<Movie> { ... }
    async fn insert(&self, movie: &Movie) -> Result<Movie> { ... }
}

// Add PostgreSQL without changing existing code
pub struct PostgresMovieRepository {
    pool: PgPool,
}

#[async_trait]
impl MovieRepository for PostgresMovieRepository {
    async fn get(&self, id: i32) -> Result<Movie> { ... }
    async fn insert(&self, movie: &Movie) -> Result<Movie> { ... }
}
```

### Bad Example

```rust
// Bad: Hardcoded database type
pub struct MovieRepository {
    db_type: DatabaseType,
    sqlite_pool: Option<SqlitePool>,
    pg_pool: Option<PgPool>,
}

impl MovieRepository {
    pub async fn get(&self, id: i32) -> Result<Movie> {
        // Requires modification for each new database
        match self.db_type {
            DatabaseType::Sqlite => { ... }
            DatabaseType::Postgres => { ... }
            // Need to modify this for MySQL, etc.
        }
    }
}
```

### Application

- Define traits for all abstraction boundaries
- Use trait objects or generics for flexibility
- Add new implementations without changing existing code
- Use strategy pattern via traits

## Liskov Substitution Principle (LSP)

**Subtypes must be substitutable for their base types.**

### Good Example

```rust
// Good: All implementations honor the contract
#[async_trait]
pub trait MovieRepository: Send + Sync {
    /// Returns Movie if found, Error::NotFound if not found
    async fn get(&self, id: i32) -> Result<Movie>;
}

// Both implementations follow the same contract
impl MovieRepository for SqliteMovieRepository {
    async fn get(&self, id: i32) -> Result<Movie> {
        sqlx::query_as("SELECT * FROM Movies WHERE Id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => Error::NotFound,
                _ => Error::Database(e),
            })
    }
}

impl MovieRepository for PostgresMovieRepository {
    async fn get(&self, id: i32) -> Result<Movie> {
        sqlx::query_as("SELECT * FROM Movies WHERE Id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => Error::NotFound,
                _ => Error::Database(e),
            })
    }
}
```

### Bad Example

```rust
// Bad: Inconsistent behavior
impl MovieRepository for CachedMovieRepository {
    async fn get(&self, id: i32) -> Result<Movie> {
        // Violates LSP: returns Ok(default) instead of Error::NotFound
        self.cache.get(&id)
            .ok_or_else(|| Ok(Movie::default())) // Wrong!
    }
}
```

### Application

- All trait implementations must honor the same contract
- Document preconditions and postconditions
- Maintain consistent error handling
- Don't strengthen preconditions or weaken postconditions

## Interface Segregation Principle (ISP)

**Clients should not depend on interfaces they don't use.**

### Good Example

```rust
// Good: Separate focused traits
#[async_trait]
pub trait MovieReader: Send + Sync {
    async fn get(&self, id: i32) -> Result<Movie>;
    async fn all(&self) -> Result<Vec<Movie>>;
}

#[async_trait]
pub trait MovieWriter: Send + Sync {
    async fn insert(&self, movie: &Movie) -> Result<Movie>;
    async fn update(&self, movie: &Movie) -> Result<Movie>;
    async fn delete(&self, id: i32) -> Result<()>;
}

#[async_trait]
pub trait MovieLookup: Send + Sync {
    async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<Movie>>;
    async fn find_by_imdb_id(&self, imdb_id: &str) -> Result<Option<Movie>>;
}

// Implement only what's needed
pub struct ReadOnlyMovieService {
    reader: Arc<dyn MovieReader>,
}

pub struct FullMovieService {
    reader: Arc<dyn MovieReader>,
    writer: Arc<dyn MovieWriter>,
    lookup: Arc<dyn MovieLookup>,
}
```

### Bad Example

```rust
// Bad: Fat interface forces unnecessary dependencies
#[async_trait]
pub trait MovieRepository: Send + Sync {
    async fn get(&self, id: i32) -> Result<Movie>;
    async fn insert(&self, movie: &Movie) -> Result<Movie>;
    async fn update(&self, movie: &Movie) -> Result<Movie>;
    async fn delete(&self, id: i32) -> Result<()>;
    async fn find_by_tmdb_id(&self, tmdb_id: i32) -> Result<Option<Movie>>;
    async fn find_by_imdb_id(&self, imdb_id: &str) -> Result<Option<Movie>>;
    async fn bulk_insert(&self, movies: &[Movie]) -> Result<()>;
    async fn bulk_update(&self, movies: &[Movie]) -> Result<()>;
    async fn vacuum(&self) -> Result<()>;
    // ... 20 more methods
}

// Read-only service forced to depend on write methods
pub struct ReadOnlyService {
    repo: Arc<dyn MovieRepository>, // Depends on methods it never uses
}
```

### Application

- Create small, focused traits
- Compose traits when needed
- Don't force clients to implement unused methods
- Use trait bounds to combine traits

## Dependency Inversion Principle (DIP)

**Depend on abstractions, not concretions.**

### Good Example

```rust
// Good: Service depends on trait, not concrete type
pub struct MovieService {
    repository: Arc<dyn MovieRepository>, // Abstraction
    config: Arc<dyn Config>,              // Abstraction
}

impl MovieService {
    pub fn new(
        repository: Arc<dyn MovieRepository>,
        config: Arc<dyn Config>,
    ) -> Self {
        Self { repository, config }
    }
}

// Concrete types injected at runtime
fn main() {
    let repo: Arc<dyn MovieRepository> = Arc::new(SqliteMovieRepository::new(pool));
    let config: Arc<dyn Config> = Arc::new(FileConfig::load());
    let service = MovieService::new(repo, config);
}
```

### Bad Example

```rust
// Bad: Service depends on concrete implementation
pub struct MovieService {
    repository: SqliteMovieRepository, // Concrete type
    config: FileConfig,                // Concrete type
}

impl MovieService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repository: SqliteMovieRepository::new(pool), // Hardcoded
            config: FileConfig::load(),                   // Hardcoded
        }
    }
}
```

### Application

- Define traits for all dependencies
- Inject dependencies via constructor
- Use `Arc<dyn Trait>` for shared ownership
- Configure concrete types at application boundary
- Enable testing with mock implementations

## SOLID in Practice

### Layered Architecture

```rust
// Domain Layer (abstractions)
pub trait MovieRepository: Send + Sync { ... }
pub trait EventPublisher: Send + Sync { ... }

// Service Layer (business logic, depends on abstractions)
pub struct MovieService {
    repository: Arc<dyn MovieRepository>,
    events: Arc<dyn EventPublisher>,
}

// Infrastructure Layer (concrete implementations)
pub struct SqlxMovieRepository { ... }
pub struct InMemoryEventPublisher { ... }

// Application Layer (composition root)
fn build_app() -> Router {
    let repo: Arc<dyn MovieRepository> = Arc::new(SqlxMovieRepository::new(pool));
    let events: Arc<dyn EventPublisher> = Arc::new(InMemoryEventPublisher::new());
    let service = Arc::new(MovieService::new(repo, events));
    
    Router::new()
        .route("/movies", get(handlers::get_movies))
        .with_state(service)
}
```

### Testing with SOLID

```rust
// Easy to test with mock implementations
#[cfg(test)]
mod tests {
    struct MockMovieRepository {
        movies: HashMap<i32, Movie>,
    }
    
    #[async_trait]
    impl MovieRepository for MockMovieRepository {
        async fn get(&self, id: i32) -> Result<Movie> {
            self.movies.get(&id)
                .cloned()
                .ok_or(Error::NotFound)
        }
    }
    
    #[tokio::test]
    async fn test_add_movie() {
        let repo = Arc::new(MockMovieRepository::new());
        let service = MovieService::new(repo, Arc::new(MockConfig::new()));
        
        let movie = Movie { ... };
        let result = service.add_movie(movie).await;
        
        assert!(result.is_ok());
    }
}
```

## Checklist

Before committing code, verify:

- [ ] Each struct/module has a single responsibility
- [ ] New functionality added via traits, not modifications
- [ ] All trait implementations are substitutable
- [ ] Traits are small and focused
- [ ] Dependencies are injected as trait objects
- [ ] Concrete types only at application boundaries
- [ ] Tests use mock implementations
- [ ] No circular dependencies
