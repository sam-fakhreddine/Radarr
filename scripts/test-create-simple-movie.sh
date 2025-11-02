#!/bin/bash
# Test POST /api/v3/movie - Create a simple movie with minimal data

echo "Testing POST /api/v3/movie (simple)..."
echo ""

curl -s -X POST http://localhost:7878/api/v3/movie \
  -H "Content-Type: application/json" \
  -d '{
    "title": "Test Movie",
    "year": 2024,
    "tmdbId": 999999,
    "path": "/movies/Test Movie (2024)",
    "qualityProfileId": 1,
    "monitored": true,
    "minimumAvailability": "released",
    "status": "released",
    "tags": []
  }' | jq '.'

echo ""
echo "Done!"
