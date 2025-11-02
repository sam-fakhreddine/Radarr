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
- Node.js 20.x and Yarn 1.22.x (for frontend)

## Quick Start

### Backend Only

1. Build and run the Rust backend:
   ```bash
   cargo run --release
   ```

2. The API will be available at http://localhost:7878/api/v3/

### Full Stack (Backend + Frontend)

1. Build the frontend:
   ```bash
   cd legacy
   yarn install
   yarn build
   cd ..
   ```

2. Run the backend (serves both API and frontend):
   ```bash
   cargo run --release
   ```

3. Access the application:
   - **Web UI**: http://localhost:7878/
   - **API**: http://localhost:7878/api/v3/movie

See [FRONTEND_INTEGRATION.md](FRONTEND_INTEGRATION.md) for detailed frontend setup and development workflow.

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

4. Run migrations (automatically run on startup):
   ```bash
   cargo run --release
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
