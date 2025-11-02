# Frontend Integration Guide

This document explains how the legacy Radarr frontend is integrated with the new Rust backend.

## Overview

The Rust backend serves both:
- **API endpoints** at `/api/v3/*`
- **Static frontend files** from `legacy/_output/UI/`
- **SPA fallback** to `index.html` for client-side routing

## Architecture

```
┌─────────────────────────────────────────┐
│         Axum Web Server (Rust)          │
│              Port 7878                  │
├─────────────────────────────────────────┤
│                                         │
│  ┌──────────────┐  ┌─────────────────┐ │
│  │  API Routes  │  │  Static Files   │ │
│  │  /api/v3/*   │  │  /*, /index.html│ │
│  └──────────────┘  └─────────────────┘ │
│         │                   │           │
│         ▼                   ▼           │
│  ┌──────────────┐  ┌─────────────────┐ │
│  │ Movie Service│  │ ServeDir/File   │ │
│  │  (Business)  │  │  (tower-http)   │ │
│  └──────────────┘  └─────────────────┘ │
│         │                   │           │
│         ▼                   ▼           │
│  ┌──────────────┐  ┌─────────────────┐ │
│  │  Repository  │  │ legacy/_output/ │ │
│  │   (SQLite)   │  │      UI/        │ │
│  └──────────────┘  └─────────────────┘ │
└─────────────────────────────────────────┘
```

## Building the Frontend

The legacy frontend is a React/TypeScript application built with Webpack.

### Prerequisites

- Node.js 20.x (managed by Volta)
- Yarn 1.22.x

### Build Steps

```bash
# Navigate to legacy directory
cd legacy

# Install dependencies
yarn install

# Build for production
yarn build

# Output will be in legacy/_output/UI/
```

### Development Mode

For frontend development with hot reload:

```bash
cd legacy
yarn watch
```

This will watch for changes and rebuild automatically.

## Running the Application

### Start the Rust Backend

```bash
# Build and run in release mode
cargo run --release

# Or for development with debug logging
RUST_LOG=debug cargo run
```

The server will start on `http://localhost:7878`

### Access the Application

- **Frontend UI**: http://localhost:7878/
- **API Endpoints**: http://localhost:7878/api/v3/movie
- **API Documentation**: See `scripts/` directory for test scripts

## How It Works

### Static File Serving

The Rust backend uses `tower-http`'s `ServeDir` to serve static files:

```rust
// In radarr_api/src/routes.rs
let serve_dir = ServeDir::new("legacy/_output/UI");
let index_service = ServeFile::new("legacy/_output/UI/index.html");

Router::new()
    .merge(movie_routes().with_state(state))
    .nest_service("/", serve_dir.clone())
    .fallback_service(index_service)
```

### Request Routing

1. **API Requests** (`/api/v3/*`): Handled by Axum handlers
2. **Static Files** (`/Content/*`, `/*.js`, `/*.css`): Served from `legacy/_output/UI/`
3. **SPA Routes** (`/movies`, `/settings`, etc.): Fallback to `index.html` for client-side routing

### CORS Configuration

CORS is configured in the tower-http middleware to allow frontend development:

```rust
.layer(
    tower_http::trace::TraceLayer::new_for_http()
        // ... tracing configuration
)
```

## Frontend Configuration

The legacy frontend expects the API to be available at `/api/v3/`. Since both are served from the same origin, no CORS configuration is needed in production.

### API Base URL

The frontend automatically uses the same origin for API calls:
- Frontend: `http://localhost:7878/`
- API: `http://localhost:7878/api/v3/`

## Development Workflow

### Full Stack Development

1. **Terminal 1**: Run the Rust backend
   ```bash
   cargo run --release
   ```

2. **Terminal 2**: Watch frontend changes (optional)
   ```bash
   cd legacy
   yarn watch
   ```

3. **Browser**: Open http://localhost:7878/

### Frontend-Only Development

If you only need to work on the frontend:

```bash
cd legacy
yarn watch
```

Then refresh the browser after changes are built.

### Backend-Only Development

The frontend is already built, so you can just work on the Rust code:

```bash
cargo watch -x run
```

## Troubleshooting

### Frontend Not Loading

1. **Check if frontend is built**:
   ```bash
   ls -la legacy/_output/UI/
   ```
   Should contain `index.html`, `*.js`, and `Content/` directory

2. **Rebuild frontend**:
   ```bash
   cd legacy
   yarn clean
   yarn build
   ```

3. **Check server logs**:
   ```bash
   RUST_LOG=debug cargo run
   ```

### API Not Working

1. **Test API directly**:
   ```bash
   curl http://localhost:7878/api/v3/movie
   ```

2. **Check database**:
   ```bash
   sqlite3 radarr.db "SELECT * FROM Movies;"
   ```

3. **Run migrations**:
   ```bash
   cargo run --release
   # Migrations run automatically on startup
   ```

### 404 Errors

- **Static files**: Ensure `legacy/_output/UI/` exists and contains files
- **API routes**: Check that routes are defined in `radarr_api/src/routes.rs`
- **SPA routes**: Should fallback to `index.html` automatically

## File Structure

```
Radarr/
├── radarr_api/          # Rust API server
│   └── src/
│       ├── main.rs      # Server entry point
│       └── routes.rs    # Route configuration
├── radarr_core/         # Business logic
├── legacy/              # Legacy frontend
│   ├── frontend/        # Source code
│   │   └── src/
│   ├── _output/         # Build output
│   │   └── UI/          # Served by Rust backend
│   └── package.json
└── migrations/          # Database migrations
```

## Production Deployment

### Build Steps

```bash
# 1. Build frontend
cd legacy
yarn install --frozen-lockfile
yarn build

# 2. Build backend
cd ..
cargo build --release

# 3. Run
./target/release/radarr_api
```

### Environment Variables

```bash
# Database
DATABASE_URL=sqlite:radarr.db

# Server
HOST=0.0.0.0
PORT=7878

# Features
AVAILABILITY_DELAY=0

# Logging
RUST_LOG=info
```

### Docker (Future)

A Dockerfile can be created to build both frontend and backend:

```dockerfile
# Stage 1: Build frontend
FROM node:20 AS frontend
WORKDIR /app/legacy
COPY legacy/package.json legacy/yarn.lock ./
RUN yarn install --frozen-lockfile
COPY legacy/ ./
RUN yarn build

# Stage 2: Build backend
FROM rust:1.75 AS backend
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY radarr_api/ radarr_api/
COPY radarr_core/ radarr_core/
COPY migrations/ migrations/
COPY --from=frontend /app/legacy/_output/ legacy/_output/
RUN cargo build --release

# Stage 3: Runtime
FROM debian:bookworm-slim
COPY --from=backend /app/target/release/radarr_api /usr/local/bin/
COPY --from=frontend /app/legacy/_output/ /app/legacy/_output/
COPY migrations/ /app/migrations/
WORKDIR /app
CMD ["radarr_api"]
```

## Stub Endpoints

The following endpoints return minimal valid responses to allow the frontend to load:

- `/api/v3/system/status` - System information
- `/api/v3/config/ui` - UI configuration
- `/api/v3/qualityprofile` - Quality profiles
- `/api/v3/language` - Available languages
- `/api/v3/localization` - Localization strings
- `/api/v3/localization/language` - Current language
- `/api/v3/tag` - Tags
- `/api/v3/collection` - Collections
- `/api/v3/customFilter` - Custom filters
- `/api/v3/indexerFlag` - Indexer flags
- `/api/v3/importlist` - Import lists

These stubs allow the frontend to load and display the movie list. Full implementations will be added as needed.

## Next Steps

- [ ] Add authentication/authorization
- [ ] Implement remaining API endpoints (quality profiles, tags, etc.)
- [ ] Add WebSocket support for real-time updates
- [ ] Optimize frontend bundle size
- [ ] Add API versioning
- [ ] Implement caching strategies
- [ ] Add health check endpoints
- [ ] Set up monitoring and metrics

## Resources

- [Axum Documentation](https://docs.rs/axum/)
- [Tower HTTP Documentation](https://docs.rs/tower-http/)
- [Legacy Frontend README](legacy/README.md)
- [API Documentation](scripts/README.md)
