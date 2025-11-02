# Performance Metrics Collection Guide

## Overview

Built-in telemetry to track performance improvements during your month of testing.

## What's Tracked

### API Performance
- Response times for all endpoints (min/avg/max)
- Request counts per endpoint
- Automatically logged every 5 minutes

### Database Performance
- Query execution times by operation type
- Connection pool metrics (opened/closed/active)
- Query counts and patterns

### System Resources
- Memory usage (MB)
- Thread count
- Process statistics

## Metrics Location

All metrics are automatically written to:
```
liveData/config/logs/performance-metrics.json
```

## Viewing Metrics

### Real-time
```bash
watch -n 5 cat liveData/config/logs/performance-metrics.json
```

### Pretty print
```bash
cat liveData/config/logs/performance-metrics.json | jq .
```

## Example Metrics Output

```json
{
  "timestamp": "2025-11-02T18:45:00Z",
  "process": {
    "memory_mb": 245.3,
    "threads": 42
  },
  "database": {
    "connections_opened": 150,
    "connections_closed": 148,
    "active_connections": 2,
    "queries": {
      "SELECT": {
        "count": 1250,
        "avg_ms": 2.3,
        "min_ms": 0.5,
        "max_ms": 45.2
      },
      "INSERT": {
        "count": 45,
        "avg_ms": 1.8,
        "min_ms": 0.8,
        "max_ms": 12.1
      }
    }
  },
  "api": {
    "endpoints": {
      "api/v3/movie": {
        "count": 523,
        "avg_ms": 125.4,
        "min_ms": 45.2,
        "max_ms": 850.3
      },
      "api/v3/queue": {
        "count": 234,
        "avg_ms": 23.1,
        "min_ms": 12.5,
        "max_ms": 156.7
      }
    }
  }
}
```

## Collecting Baseline

Before making changes:
```bash
# Let it run for a day
cp liveData/config/logs/performance-metrics.json metrics/baseline.json
```

## Comparing Performance

After your changes:
```bash
# Compare current vs baseline
jq -s '
{
  "memory_improvement": (.[0].process.memory_mb - .[1].process.memory_mb),
  "api_improvements": [
    .[0].api.endpoints | to_entries[] | 
    {
      endpoint: .key,
      before: .[0].api.endpoints[.key].avg_ms,
      after: .[1].api.endpoints[.key].avg_ms,
      improvement_ms: (.[0].api.endpoints[.key].avg_ms - .[1].api.endpoints[.key].avg_ms)
    }
  ]
}
' metrics/baseline.json liveData/config/logs/performance-metrics.json
```

## For Your PR

Include these metrics in your PR description:
- Memory reduction: X MB (Y%)
- API response time improvements
- Database query optimizations
- Connection pool efficiency gains

## Notes

- Metrics are written every 5 minutes automatically
- No performance impact (async writes)
- All data stays local
- No external telemetry or tracking
