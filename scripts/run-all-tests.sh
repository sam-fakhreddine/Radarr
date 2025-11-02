#!/bin/bash
# Run all API tests in sequence

set -e

echo "========================================="
echo "Running All API Tests"
echo "========================================="
echo ""

# Check if server is running
if ! curl -s http://localhost:7879/api/v3/movie > /dev/null 2>&1; then
    echo "❌ Server is not running on port 7879"
    echo "Please start the server first: cargo run --release"
    exit 1
fi

echo "✅ Server is running"
echo ""

# Test 1: Get all movies (should be empty initially)
echo "========================================="
echo "Test 1: GET /api/v3/movie (all movies)"
echo "========================================="
./scripts/test-get-all-movies.sh
echo ""

# Test 2: Create a movie
echo "========================================="
echo "Test 2: POST /api/v3/movie (create)"
echo "========================================="
./scripts/test-create-movie.sh
echo ""

# Test 3: Get all movies (should have 1 movie)
echo "========================================="
echo "Test 3: GET /api/v3/movie (verify creation)"
echo "========================================="
./scripts/test-get-all-movies.sh
echo ""

# Test 4: Get movie by ID
echo "========================================="
echo "Test 4: GET /api/v3/movie/1 (by ID)"
echo "========================================="
./scripts/test-get-movie-by-id.sh 1
echo ""

# Test 5: Find by TMDB ID
echo "========================================="
echo "Test 5: GET /api/v3/movie?tmdbId=27205"
echo "========================================="
./scripts/test-find-by-tmdb.sh 27205
echo ""

# Test 6: Update movie
echo "========================================="
echo "Test 6: PUT /api/v3/movie/1 (update)"
echo "========================================="
./scripts/test-update-movie.sh 1
echo ""

# Test 7: Get updated movie
echo "========================================="
echo "Test 7: GET /api/v3/movie/1 (verify update)"
echo "========================================="
./scripts/test-get-movie-by-id.sh 1
echo ""

# Test 8: Delete movie
echo "========================================="
echo "Test 8: DELETE /api/v3/movie/1"
echo "========================================="
./scripts/test-delete-movie.sh 1
echo ""

# Test 9: Verify deletion
echo "========================================="
echo "Test 9: GET /api/v3/movie/1 (should 404)"
echo "========================================="
./scripts/test-get-movie-by-id.sh 1
echo ""

echo "========================================="
echo "All Tests Complete!"
echo "========================================="
