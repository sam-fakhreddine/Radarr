#!/bin/bash
# Seed the database with test data

echo "Seeding test data..."
echo ""

# Create quality profile first
echo "Creating quality profile..."
sqlite3 radarr.db "INSERT OR IGNORE INTO QualityProfiles (Id, Name, Cutoff, Items, MinFormatScore, CutoffFormatScore, FormatItems, Language, UpgradeAllowed) VALUES (1, 'HD-1080p', 7, '[]', 0, 0, '[]', 1, 1);"

# Create some movie metadata
echo "Creating movie metadata..."

sqlite3 radarr.db "INSERT OR IGNORE INTO MovieMetadata (Id, TmdbId, ImdbId, Title, OriginalTitle, CleanTitle, SortTitle, Year, Status, Overview, Images, Genres, Ratings, Runtime, InCinemas, PhysicalRelease, DigitalRelease, Certification, Website, YouTubeTrailerId, Studio, Popularity, CollectionTmdbId, CollectionTitle) VALUES 
(1, 27205, 'tt1375666', 'Inception', 'Inception', 'inception', 'inception', 2010, 'released', 'A thief who steals corporate secrets through dream-sharing technology.', '[]', '[\"Action\", \"Science Fiction\", \"Thriller\"]', '{\"imdb\": {\"value\": 8.8, \"votes\": 2000000}}', 148, '2010-07-16', '2010-12-07', '2010-12-07', 'PG-13', 'https://www.warnerbros.com/movies/inception', 'YoHD9XEInc0', 'Warner Bros.', 95.5, 0, NULL),
(2, 550, 'tt0137523', 'Fight Club', 'Fight Club', 'fightclub', 'fightclub', 1999, 'released', 'An insomniac office worker and a devil-may-care soapmaker form an underground fight club.', '[]', '[\"Drama\"]', '{\"imdb\": {\"value\": 8.8, \"votes\": 1900000}}', 139, '1999-10-15', '2000-06-06', '2000-06-06', 'R', NULL, NULL, '20th Century Fox', 92.3, 0, NULL),
(3, 13, 'tt0109830', 'Forrest Gump', 'Forrest Gump', 'forrestgump', 'forrestgump', 1994, 'released', 'The presidencies of Kennedy and Johnson unfold through the perspective of an Alabama man.', '[]', '[\"Comedy\", \"Drama\", \"Romance\"]', '{\"imdb\": {\"value\": 8.8, \"votes\": 1800000}}', 142, '1994-07-06', '1995-04-27', '1995-04-27', 'PG-13', NULL, NULL, 'Paramount', 88.9, 0, NULL);"

echo ""
echo "Test data seeded successfully!"
echo ""
echo "You can now:"
echo "  - Create movies using the metadata above"
echo "  - Test GET /api/v3/movie"
echo "  - Test POST /api/v3/movie with tmdbId: 27205, 550, or 13"
