#!/bin/bash
# Continuous monitoring - run this in background during your month of testing

METRICS_DIR="./metrics"
INTERVAL=3600  # Collect every hour

mkdir -p "$METRICS_DIR"

echo "Starting continuous metrics collection (every $INTERVAL seconds)"
echo "Metrics will be saved to: $METRICS_DIR"
echo "Press Ctrl+C to stop"

while true; do
    ./metrics-collector.sh
    sleep $INTERVAL
done
