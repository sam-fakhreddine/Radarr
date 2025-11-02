-- Create MovieMetadata table (PostgreSQL)
-- This table stores TMDB-sourced metadata for movies
CREATE TABLE IF NOT EXISTS MovieMetadata (
    Id SERIAL PRIMARY KEY,
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
    InCinemas TIMESTAMP,
    PhysicalRelease TIMESTAMP,
    DigitalRelease TIMESTAMP,
    Certification TEXT,
    Website TEXT,
    YouTubeTrailerId TEXT,
    Studio TEXT,
    Popularity REAL NOT NULL,
    CollectionTmdbId INTEGER NOT NULL DEFAULT 0,
    CollectionTitle TEXT
);

-- Create indexes for frequently queried columns
CREATE UNIQUE INDEX IF NOT EXISTS IX_MovieMetadata_TmdbId ON MovieMetadata(TmdbId);
CREATE INDEX IF NOT EXISTS IX_MovieMetadata_ImdbId ON MovieMetadata(ImdbId);
CREATE INDEX IF NOT EXISTS IX_MovieMetadata_CleanTitle ON MovieMetadata(CleanTitle);
