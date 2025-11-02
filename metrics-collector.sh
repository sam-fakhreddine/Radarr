#!/bin/bash
# Collect Radarr performance metrics for comparison

METRICS_DIR="./metrics"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
METRICS_FILE="$METRICS_DIR/metrics_$TIMESTAMP.json"

mkdir -p "$METRICS_DIR"

echo "Collecting metrics..."

# Get process stats
PID=$(pgrep -f "Radarr.*7878" | head -1)

if [ -z "$PID" ]; then
    echo "Radarr not running"
    exit 1
fi

# Collect metrics
cat > "$METRICS_FILE" <<EOF
{
  "timestamp": "$(date -Iseconds)",
  "process": {
    "pid": $PID,
    "memory_mb": $(ps -p $PID -o rss= | awk '{print $1/1024}'),
    "cpu_percent": $(ps -p $PID -o %cpu= | awk '{print $1}'),
    "threads": $(ps -p $PID -o nlwp= | awk '{print $1}')
  },
  "api_response_times": {},
  "database_size_mb": $(du -m liveData/config/radarr.db | cut -f1)
}
EOF

# Test API response times
echo "Testing API endpoints..."
for endpoint in "system/status" "movie" "health" "queue"; do
    RESPONSE_TIME=$(curl -s -w "%{time_total}" -o /dev/null "http://localhost:7878/api/v3/$endpoint" 2>/dev/null)
    # Update JSON with response time
    jq ".api_response_times[\"$endpoint\"] = $RESPONSE_TIME" "$METRICS_FILE" > "$METRICS_FILE.tmp" && mv "$METRICS_FILE.tmp" "$METRICS_FILE"
done

echo "Metrics saved to: $METRICS_FILE"
cat "$METRICS_FILE" | jq .
