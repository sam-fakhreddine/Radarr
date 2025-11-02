#!/bin/bash
# Test PUT /api/v3/movie/:id - Update a movie

MOVIE_ID=${1:-1}

echo "Testing PUT /api/v3/movie/$MOVIE_ID..."
echo ""

curl -s -X PUT http://localhost:7879/api/v3/movie/$MOVIE_ID \
  -H "Content-Type: application/json" \
  -d '{
    "id": '$MOVIE_ID',
    "title": "Inception",
    "year": 2010,
    "tmdbId": 27205,
    "path": "/movies/Inception (2010) - Updated",
    "qualityProfileId": 1,
    "monitored": false,
    "minimumAvailability": "released",
    "status": "released",
    "tags": [1, 2, 3]
  }' | jq '.'

echo ""
echo "Done!"
