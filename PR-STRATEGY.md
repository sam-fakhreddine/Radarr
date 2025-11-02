# Two-PR Strategy for Performance Improvements

## Timeline

### Week 1-2: PR #1 - Telemetry Baseline
**Branch:** `telemetry-baseline`

**Goal:** Establish performance baseline with zero functional changes

**Changes:**
- Add `PerformanceMetrics.cs` - metrics collection
- Add `PerformanceMiddleware.cs` - API tracking
- Update `Startup.cs` - register middleware
- **NO database optimizations**
- **NO query changes**
- **NO connection pool changes**

**What to include:**
```
src/NzbDrone.Core/Instrumentation/PerformanceMetrics.cs
src/Radarr.Http/Middleware/PerformanceMiddleware.cs
src/NzbDrone.Host/Startup.cs (middleware registration only)
```

**PR Description:**
```
Add performance telemetry for baseline metrics

This PR adds instrumentation to collect performance metrics with zero
functional changes. Metrics are written to /config/logs/performance-metrics.json
every 5 minutes.

Tracked metrics:
- API response times (min/avg/max per endpoint)
- Database query performance
- Connection pool statistics
- Memory usage
- Thread counts

This establishes a baseline for upcoming performance optimizations.
No behavior changes, pure instrumentation.
```

**Run this for 2 weeks** to collect baseline data.

---

### Week 3-6: PR #2 - Database Optimizations
**Branch:** `performance-optimizations`

**Goal:** Implement optimizations and show measurable improvements

**Changes:**
- All your database connection pool improvements
- Query optimizations
- Caching enhancements
- Keep the telemetry from PR #1

**PR Description:**
```
Optimize database connection pooling and query performance

Building on the telemetry added in PR #XXX, this implements several
database performance optimizations.

Changes:
- [List your specific optimizations]
- Connection pool tuning
- Query optimization
- Caching improvements

Performance improvements (2 week comparison):
- Memory usage: -X MB (-Y%)
- API response times:
  - /api/v3/movie: 125ms → 45ms (-64%)
  - /api/v3/queue: 23ms → 8ms (-65%)
- Database queries:
  - Average query time: 2.3ms → 0.8ms (-65%)
  - Active connections: 5 → 2 (-60%)

Metrics collected over 2 weeks in production homelab environment.
See attached performance-metrics comparison.
```

---

## Collecting Baseline

### On PR #1 (telemetry-baseline):
```bash
# Day 1
cp /config/logs/performance-metrics.json baseline-day1.json

# Day 7
cp /config/logs/performance-metrics.json baseline-day7.json

# Day 14
cp /config/logs/performance-metrics.json baseline-day14.json
```

### On PR #2 (performance-optimizations):
```bash
# Day 1
cp /config/logs/performance-metrics.json optimized-day1.json

# Day 7
cp /config/logs/performance-metrics.json optimized-day7.json

# Day 14
cp /config/logs/performance-metrics.json optimized-day14.json
```

## Comparison Script

```bash
# Compare baseline vs optimized
jq -s '
{
  baseline: .[0],
  optimized: .[1],
  improvements: {
    memory_mb: (.[0].process.memory_mb - .[1].process.memory_mb),
    memory_percent: ((.[0].process.memory_mb - .[1].process.memory_mb) / .[0].process.memory_mb * 100)
  }
}
' baseline-day14.json optimized-day14.json > comparison.json
```

## Benefits of This Approach

1. **Maintainers can verify** - They can merge PR #1 and collect their own baseline
2. **No guessing** - Real metrics, not estimates
3. **Reproducible** - Anyone can run the same tests
4. **Low risk** - PR #1 has zero functional changes
5. **Clear impact** - PR #2 shows measurable improvements

## Current Status

- ✅ Telemetry code written
- ✅ Branches created
- ⏳ Need to commit telemetry-only changes to PR #1
- ⏳ Need to add your DB optimizations to PR #2
