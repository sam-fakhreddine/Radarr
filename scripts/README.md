# Test Scripts

Shell scripts for testing the Radarr API endpoints.

## Prerequisites

- Server running on `http://localhost:7878`
- `curl` installed
- `jq` installed (for JSON formatting)
- `sqlite3` installed (for seeding data)

## Setup

Make all scripts executable:

```bash
chmod +x scripts/*.sh
```

## Database Management

### Reset Database
Deletes the database and starts fresh:
```bash
./scripts/reset-database.sh
```

### Seed Test Data
Adds quality profiles and movie metadata for testing:
```bash
./scripts/seed-test-data.sh
```

This creates:
- Quality Profile: HD-1080p (ID: 1)
- Movie Metadata:
  - Inception (TMDB: 27205)
  - Fight Club (TMDB: 550)
  - Forrest Gump (TMDB: 13)

## Individual Tests

### Get All Movies
```bash
./scripts/test-get-all-movies.sh
```

### Get Movie by ID
```bash
./scripts/test-get-movie-by-id.sh [MOVIE_ID]
# Example: ./scripts/test-get-movie-by-id.sh 1
```

### Find Movie by TMDB ID
```bash
./scripts/test-find-by-tmdb.sh [TMDB_ID]
# Example: ./scripts/test-find-by-tmdb.sh 27205
```

### Create Movie
```bash
./scripts/test-create-movie.sh
```

Creates "Inception" with full metadata.

### Update Movie
```bash
./scripts/test-update-movie.sh [MOVIE_ID]
# Example: ./scripts/test-update-movie.sh 1
```

### Delete Movie
```bash
./scripts/test-delete-movie.sh [MOVIE_ID] [DELETE_FILES] [ADD_EXCLUSION]
# Examples:
./scripts/test-delete-movie.sh 1
./scripts/test-delete-movie.sh 1 true false
```

## Run All Tests

Runs a complete test suite:
```bash
./scripts/run-all-tests.sh
```

This will:
1. Check if server is running
2. Get all movies (empty)
3. Create a movie
4. Verify creation
5. Get movie by ID
6. Find by TMDB ID
7. Update movie
8. Verify update
9. Delete movie
10. Verify deletion

## Quick Start Workflow

```bash
# 1. Reset and prepare database
./scripts/reset-database.sh

# 2. Start server (in another terminal)
cargo run --release

# 3. Seed test data
./scripts/seed-test-data.sh

# 4. Run all tests
./scripts/run-all-tests.sh
```

## Custom Test Examples

### Create Movie with Existing Metadata
After seeding, you can create movies using the seeded metadata:

```bash
curl -X POST http://localhost:7878/api/v3/movie \
  -H "Content-Type: application/json" \
  -d '{
    "tmdbId": 550,
    "path": "/movies/Fight Club (1999)",
    "qualityProfileId": 1,
    "monitored": true,
    "minimumAvailability": "released"
  }' | jq '.'
```

The service will automatically look up the metadata for TMDB ID 550 (Fight Club).

### Create Movie with New Metadata
You can also create a movie with metadata that doesn't exist yet:

```bash
curl -X POST http://localhost:7878/api/v3/movie \
  -H "Content-Type: application/json" \
  -d '{
    "title": "The Matrix",
    "year": 1999,
    "tmdbId": 603,
    "path": "/movies/The Matrix (1999)",
    "qualityProfileId": 1,
    "monitored": true,
    "minimumAvailability": "released",
    "status": "released",
    "overview": "A computer hacker learns about the true nature of reality.",
    "runtime": 136,
    "genres": ["Action", "Science Fiction"],
    "tags": []
  }' | jq '.'
```

The service will create both the metadata and the movie.

## Troubleshooting

### Server Not Running
```bash
# Check if server is running
curl http://localhost:7878/api/v3/movie

# If not, start it
cargo run --release
```

### Database Locked
```bash
# Stop the server
pkill -f radarr_api

# Remove lock files
rm -f radarr.db-shm radarr.db-wal

# Restart server
cargo run --release
```

### jq Not Installed
```bash
# macOS
brew install jq

# Ubuntu/Debian
sudo apt-get install jq
```

### Permission Denied
```bash
# Make scripts executable
chmod +x scripts/*.sh
```
