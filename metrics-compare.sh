#!/bin/bash
# Compare metrics between baseline and optimized versions

METRICS_DIR="./metrics"

if [ ! -d "$METRICS_DIR" ]; then
    echo "No metrics directory found. Run metrics-collector.sh first."
    exit 1
fi

BASELINE=$(ls -t $METRICS_DIR/baseline_*.json 2>/dev/null | head -1)
LATEST=$(ls -t $METRICS_DIR/metrics_*.json 2>/dev/null | head -1)

if [ -z "$BASELINE" ]; then
    echo "No baseline metrics found. Save current metrics as baseline:"
    echo "  cp $LATEST $METRICS_DIR/baseline_$(date +%Y%m%d).json"
    exit 1
fi

echo "=== Performance Comparison ==="
echo ""
echo "Baseline: $(basename $BASELINE)"
echo "Current:  $(basename $LATEST)"
echo ""

# Memory comparison
BASELINE_MEM=$(jq -r '.process.memory_mb' "$BASELINE")
CURRENT_MEM=$(jq -r '.process.memory_mb' "$LATEST")
MEM_DIFF=$(echo "$CURRENT_MEM - $BASELINE_MEM" | bc)
MEM_PERCENT=$(echo "scale=2; ($MEM_DIFF / $BASELINE_MEM) * 100" | bc)

echo "Memory Usage:"
echo "  Baseline: ${BASELINE_MEM} MB"
echo "  Current:  ${CURRENT_MEM} MB"
echo "  Change:   ${MEM_DIFF} MB (${MEM_PERCENT}%)"
echo ""

# API response times
echo "API Response Times (seconds):"
for endpoint in system/status movie health queue; do
    BASELINE_TIME=$(jq -r ".api_response_times[\"$endpoint\"]" "$BASELINE")
    CURRENT_TIME=$(jq -r ".api_response_times[\"$endpoint\"]" "$LATEST")
    
    if [ "$BASELINE_TIME" != "null" ] && [ "$CURRENT_TIME" != "null" ]; then
        DIFF=$(echo "$CURRENT_TIME - $BASELINE_TIME" | bc)
        PERCENT=$(echo "scale=2; ($DIFF / $BASELINE_TIME) * 100" | bc)
        echo "  $endpoint: ${BASELINE_TIME}s -> ${CURRENT_TIME}s (${PERCENT}% change)"
    fi
done
echo ""

# Database size
BASELINE_DB=$(jq -r '.database_size_mb' "$BASELINE")
CURRENT_DB=$(jq -r '.database_size_mb' "$LATEST")
echo "Database Size:"
echo "  Baseline: ${BASELINE_DB} MB"
echo "  Current:  ${CURRENT_DB} MB"
