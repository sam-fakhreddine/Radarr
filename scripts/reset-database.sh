#!/bin/bash
# Reset the database (delete and recreate)

echo "Resetting database..."
echo ""

# Stop server if running
echo "Stopping server if running..."
pkill -f radarr_api || true

# Remove database
echo "Removing old database..."
rm -f radarr.db radarr.db-shm radarr.db-wal

echo ""
echo "Database reset complete!"
echo ""
echo "Next steps:"
echo "  1. Start the server: cargo run --release"
echo "  2. Run seed script: ./scripts/seed-test-data.sh"
