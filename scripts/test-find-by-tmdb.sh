#!/bin/bash
# Test GET /api/v3/movie?tmdbId=X - Find movie by TMDB ID

TMDB_ID=${1:-27205}

echo "Testing GET /api/v3/movie?tmdbId=$TMDB_ID..."
echo ""

curl -s "http://localhost:7878/api/v3/movie?tmdbId=$TMDB_ID" | jq '.'

echo ""
echo "Done!"
