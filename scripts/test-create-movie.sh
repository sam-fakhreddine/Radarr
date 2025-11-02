#!/bin/bash
# Test POST /api/v3/movie - Create a new movie

echo "Testing POST /api/v3/movie..."
echo ""

curl -s -X POST http://localhost:7878/api/v3/movie \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Inception",
    "year": 2010,
    "tmdbId": 27205,
    "imdbId": "tt1375666",
    "path": "/movies/Inception (2010)",
    "qualityProfileId": 1,
    "monitored": true,
    "minimumAvailability": "released",
    "status": "released",
    "overview": "A thief who steals corporate secrets through the use of dream-sharing technology is given the inverse task of planting an idea into the mind of a C.E.O.",
    "runtime": 148,
    "genres": ["Action", "Science Fiction", "Thriller"],
    "ratings": {
      "imdb": {
        "value": 8.8,
        "votes": 2000000
      },
      "tmdb": {
        "value": 8.3,
        "votes": 25000
      }
    },
    "certification": "PG-13",
    "studio": "Warner Bros.",
    "tags": [1, 2]
  }' | jq '.'

echo ""
echo "Done!"
