-- Create QualityProfiles table
CREATE TABLE IF NOT EXISTS QualityProfiles (
    Id INTEGER PRIMARY KEY AUTOINCREMENT,
    Name TEXT NOT NULL,
    Cutoff INTEGER NOT NULL,
    Items TEXT NOT NULL,
    MinFormatScore INTEGER NOT NULL DEFAULT 0,
    CutoffFormatScore INTEGER NOT NULL DEFAULT 0,
    FormatItems TEXT NOT NULL,
    Language INTEGER NOT NULL DEFAULT 1,
    UpgradeAllowed INTEGER NOT NULL DEFAULT 1
);

-- Insert default quality profile
INSERT OR IGNORE INTO QualityProfiles (Id, Name, Cutoff, Items, MinFormatScore, CutoffFormatScore, FormatItems, Language, UpgradeAllowed)
VALUES (1, 'HD-1080p', 7, '[]', 0, 0, '[]', 1, 1);
