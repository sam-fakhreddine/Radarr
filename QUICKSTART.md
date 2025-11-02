# Radarr Rust - Quick Start Guide

## Prerequisites

- Rust 1.70+ (install from https://rustup.rs/)
- SQLite (usually pre-installed on macOS/Linux)

## Running the Application

### 1. Start the API Server

```bash
cargo run --package radarr_api
```

The server will:
- Create `radarr.db` SQLite database if it doesn't exist
- Run all migrations automatically
- Start listening on `http://0.0.0.0:7878`

### 2. Custom Configuration

You can customize the server using environment variables:

```bash
# Custom database location
DATABASE_URL=sqlite:custom.db cargo run --package radarr_api

# Custom host and port
HOST=127.0.0.1 PORT=8080 cargo run --package radarr_api

# All together
DATABASE_URL=sqlite:data/radarr.db HOST=127.0.0.1 PORT=8080 cargo run --package radarr_api
```

## API Endpoints

Once running, the following endpoints are available:

### Get All Movies
```bash
curl http://localhost:7878/api/v3/movie
```

### Get Movie by ID
```bash
curl http://localhost:7878/api/v3/movie/1
```

### Create Movie
```bash
curl -X POST http://localhost:7878/api/v3/movie \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Inception",
    "year": 2010,
    "tmdbId": 27205,
    "qualityProfileId": 1,
    "path": "/movies/Inception (2010)",
    "monitored": true
  }'
```

### Update Movie
```bash
curl -X PUT http://localhost:7878/api/v3/movie/1 \
  -H "Content-Type: application/json" \
  -d '{
    "id": 1,
    "title": "Inception",
    "year": 2010,
    "tmdbId": 27205,
    "qualityProfileId": 1,
    "path": "/movies/Inception (2010)",
    "monitored": false
  }'
```

### Delete Movie
```bash
curl -X DELETE http://localhost:7878/api/v3/movie/1
```

## Development

### Run Tests
```bash
cargo test
```

### Run Linter
```bash
cargo clippy -- -D warnings
```

### Format Code
```bash
cargo fmt
```

### Run All Quality Checks
```bash
cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test
```

## Database Setup

The database is automatically initialized on first run. However, you need to manually create a quality profile before adding movies:

```bash
# Use the provided seed script
sqlite3 radarr.db < seed.sql
```

Or manually:

```sql
-- Connect to the database
sqlite3 radarr.db

-- Create a quality profile
INSERT INTO QualityProfiles (Id, Name, Cutoff, Items, MinFormatScore, CutoffFormatScore, FormatItems, Language, UpgradeAllowed)
VALUES (1, 'HD-1080p', 7, '[]', 0, 0, '[]', 1, 1);
```

## Troubleshooting

### Database locked error
If you get a "database is locked" error, make sure no other process is accessing the database file.

### Port already in use
If port 7878 is already in use, specify a different port:
```bash
PORT=8080 cargo run --package radarr_api
```

### Migration errors
If migrations fail, you can reset the database:
```bash
rm radarr.db radarr.db-shm radarr.db-wal
cargo run --package radarr_api
```

### UNIQUE constraint failed: _sqlx_migrations.version
This error occurs if you have duplicate migration files. The PostgreSQL migrations have been disabled (renamed to `.disabled`) to prevent conflicts. Only SQLite migrations will run.

## Project Structure

```
radarr_core/          # Core business logic and domain models
  ├── domain/         # Domain entities (Movie, MovieMetadata, etc.)
  ├── repository/     # Data access layer
  ├── service/        # Business logic services
  └── validation/     # Validation helpers

radarr_api/           # HTTP API layer
  ├── handlers/       # HTTP request handlers
  ├── resources/      # DTOs for API requests/responses
  ├── routes.rs       # Route configuration
  └── main.rs         # Application entry point

migrations/           # Database migrations
```

## Next Steps

- Add quality profiles via SQL or API
- Add root folders for movie storage
- Start adding movies via the API
- Explore the codebase and contribute!
