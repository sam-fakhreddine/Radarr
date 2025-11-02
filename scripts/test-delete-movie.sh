#!/bin/bash
# Test DELETE /api/v3/movie/:id - Delete a movie

MOVIE_ID=${1:-1}
DELETE_FILES=${2:-false}
ADD_EXCLUSION=${3:-false}

echo "Testing DELETE /api/v3/movie/$MOVIE_ID?deleteFiles=$DELETE_FILES&addImportExclusion=$ADD_EXCLUSION..."
echo ""

curl -s -X DELETE "http://localhost:7879/api/v3/movie/$MOVIE_ID?deleteFiles=$DELETE_FILES&addImportExclusion=$ADD_EXCLUSION" \
  -w "\nHTTP Status: %{http_code}\n"

echo ""
echo "Done!"
