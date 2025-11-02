#!/bin/bash
# Quick start script - sets up everything and runs tests

set -e

echo "========================================="
echo "Radarr API Quick Start"
echo "========================================="
echo ""

# Step 1: Reset database
echo "Step 1: Resetting database..."
./scripts/reset-database.sh
echo ""

# Step 2: Build
echo "Step 2: Building project..."
cargo build --release
echo ""

# Step 3: Start server in background
echo "Step 3: Starting server..."
cargo run --release > server.log 2>&1 &
SERVER_PID=$!
echo "Server started with PID: $SERVER_PID"
echo ""

# Wait for server to be ready
echo "Waiting for server to be ready..."
for i in {1..30}; do
    if curl -s http://localhost:7878/api/v3/movie > /dev/null 2>&1; then
        echo "✅ Server is ready!"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "❌ Server failed to start within 30 seconds"
        echo "Check server.log for details"
        kill $SERVER_PID 2>/dev/null || true
        exit 1
    fi
    sleep 1
done
echo ""

# Step 4: Seed data
echo "Step 4: Seeding test data..."
./scripts/seed-test-data.sh
echo ""

# Step 5: Run tests
echo "Step 5: Running all tests..."
echo ""
./scripts/run-all-tests.sh

echo ""
echo "========================================="
echo "Quick Start Complete!"
echo "========================================="
echo ""
echo "Server is running with PID: $SERVER_PID"
echo "Server logs: server.log"
echo ""
echo "To stop the server:"
echo "  kill $SERVER_PID"
echo ""
echo "To run tests again:"
echo "  ./scripts/run-all-tests.sh"
