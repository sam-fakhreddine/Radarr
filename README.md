# Radarr Rust Implementation

A Rust port of Radarr's movie CRUD operations, maintaining full API compatibility with the existing Radarr V3 API.

## Project Structure

### Rust Implementation (Active Development)
- `radarr_core/` - Core domain models, business logic, and data access layer
- `radarr_api/` - HTTP API server using Axum
- `migrations/` - Database migration files

### Legacy Code (Reference Only)
- `legacy/` - Original C#/.NET implementation and frontend code
  - `legacy/src/` - C# source code
  - `legacy/frontend/` - TypeScript/React frontend
  - `legacy/distribution/` - Distribution packages

## Prerequisites

- Rust 1.75 or later
- SQLite or PostgreSQL

## Setup

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Update the `DATABASE_URL` in `.env` if needed

3. Build the project:
   ```bash
   cargo build
   ```

4. Run migrations (once implemented):
   ```bash
   cargo sqlx migrate run
   ```

5. Run the API server:
   ```bash
   cargo run --bin radarr_api
   ```

## Development

### Running Tests

```bash
cargo test
```

### Linting

```bash
cargo clippy -- -D warnings
```

### Formatting

```bash
cargo fmt
```

### Database Migrations

```bash
# Create a new migration
cargo sqlx migrate add <migration_name>

# Run migrations
cargo sqlx migrate run

# Revert last migration
cargo sqlx migrate revert
```

## Architecture

The project follows a layered architecture:

- **Domain Layer** (`radarr_core/src/domain/`) - Core entities and types
- **Repository Layer** (`radarr_core/src/repository/`) - Data access
- **Service Layer** (`radarr_core/src/service/`) - Business logic
- **API Layer** (`radarr_api/src/`) - HTTP handlers and routing

## License

GPL-3.0
