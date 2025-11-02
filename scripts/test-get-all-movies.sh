#!/bin/bash
# Test GET /api/v3/movie - Get all movies

echo "Testing GET /api/v3/movie..."
echo ""

curl -s http://localhost:7879/api/v3/movie | jq '.'

echo ""
echo "Done!"
