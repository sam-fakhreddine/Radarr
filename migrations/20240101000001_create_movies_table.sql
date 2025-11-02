-- Create Movies table
-- This table stores movie records with references to metadata and quality profiles
CREATE TABLE IF NOT EXISTS Movies (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    MovieMetadataId INTEGER NOT NULL,
    Monitored INTEGER NOT NULL DEFAULT 1,
    MinimumAvailability TEXT NOT NULL,
    QualityProfileId INTEGER NOT NULL,
    Path TEXT NOT NULL,
    RootFolderPath TEXT,
    Added TEXT NOT NULL,
    Tags TEXT,
    AddOptions TEXT,
    LastSearchTime TEXT,
    MovieFileId INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (MovieMetadataId) REFERENCES MovieMetadata(Id)
);

-- Create indexes for frequently queried columns
CREATE INDEX IF NOT EXISTS IX_Movies_MovieMetadataId ON Movies(MovieMetadataId);
CREATE INDEX IF NOT EXISTS IX_Movies_Path ON Movies(Path);
