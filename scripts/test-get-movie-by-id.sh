#!/bin/bash
# Test GET /api/v3/movie/:id - Get movie by ID

MOVIE_ID=${1:-1}

echo "Testing GET /api/v3/movie/$MOVIE_ID..."
echo ""

curl -s http://localhost:7879/api/v3/movie/$MOVIE_ID | jq '.'

echo ""
echo "Done!"
