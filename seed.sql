-- Seed script for Radarr database
-- Run with: sqlite3 radarr.db < seed.sql

-- Create a default quality profile
INSERT OR IGNORE INTO QualityProfiles (Id, Name, Cutoff, Items, MinFormatScore, CutoffFormatScore, FormatItems, Language, UpgradeAllowed)
VALUES (1, 'HD-1080p', 7, '[]', 0, 0, '[]', 1, 1);

-- Verify the data
SELECT 'Quality Profiles:' as '';
SELECT * FROM QualityProfiles;
