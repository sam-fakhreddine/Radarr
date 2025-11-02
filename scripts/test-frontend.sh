#!/bin/bash

# Test script for frontend integration

BASE_URL="http://localhost:7878"

echo "Testing Frontend Integration..."
echo "================================"
echo ""

# Test 1: Initialize endpoint
echo "1. Testing /initialize.json endpoint..."
INIT_RESPONSE=$(curl -s "$BASE_URL/initialize.json")
echo "$INIT_RESPONSE" | jq '.'
if echo "$INIT_RESPONSE" | jq -e '.apiRoot' > /dev/null; then
    echo "✅ Initialize endpoint working"
else
    echo "❌ Initialize endpoint failed"
    exit 1
fi
echo ""

# Test 2: Index.html with URL_BASE replacement
echo "2. Testing index.html URL_BASE replacement..."
INDEX_CONTENT=$(curl -s "$BASE_URL/")
if echo "$INDEX_CONTENT" | grep -q "urlBase: ''"; then
    echo "✅ URL_BASE replaced correctly"
else
    echo "❌ URL_BASE not replaced"
    echo "Content: $INDEX_CONTENT" | head -20
    exit 1
fi
echo ""

# Test 3: Static files (CSS)
echo "3. Testing static file serving (CSS)..."
CSS_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/Content/styles.css")
if [ "$CSS_STATUS" = "200" ]; then
    echo "✅ Static CSS files served correctly"
else
    echo "❌ Static CSS files not found (Status: $CSS_STATUS)"
    exit 1
fi
echo ""

# Test 4: Static files (JS)
echo "4. Testing static file serving (JS)..."
JS_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/index.js")
if [ "$JS_STATUS" = "200" ]; then
    echo "✅ Static JS files served correctly"
else
    echo "❌ Static JS files not found (Status: $JS_STATUS)"
    exit 1
fi
echo ""

# Test 5: API endpoint
echo "5. Testing API endpoint..."
API_RESPONSE=$(curl -s "$BASE_URL/api/v3/movie")
if echo "$API_RESPONSE" | jq -e '.' > /dev/null 2>&1; then
    echo "✅ API endpoint working"
    echo "Movies count: $(echo "$API_RESPONSE" | jq 'length')"
else
    echo "❌ API endpoint failed"
    exit 1
fi
echo ""

# Test 6: SPA fallback
echo "6. Testing SPA fallback routing..."
SPA_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE_URL/movies")
if [ "$SPA_STATUS" = "200" ]; then
    echo "✅ SPA fallback working"
else
    echo "❌ SPA fallback not working (Status: $SPA_STATUS)"
    exit 1
fi
echo ""

echo "================================"
echo "✅ All frontend integration tests passed!"
echo ""
echo "Open http://localhost:7878 in your browser to use the application"
