# Radarr API Performance Fixes

## Critical Issues Causing API Overwhelm

### 1. Database Connection Management
**Problem**: No connection pooling, low busy timeout, lock contention
**Fix**: 
- Increase SQLite busy timeout from 100ms to 5000ms
- Implement connection pooling
- Use WAL mode consistently (already partially implemented)

### 2. Controller Inefficiencies  
**Problem**: Synchronous operations, N+1 queries, no pagination
**Fix**:
- Implement proper async/await patterns
- Add request rate limiting middleware
- Use batch queries for statistics and translations
- Add pagination to large result sets

### 3. Missing Concurrency Controls
**Problem**: Multiple CLI calls can overwhelm single-threaded SQLite
**Fix**:
- Add request queuing middleware
- Implement semaphore-based throttling
- Use async patterns instead of Task.Run()

### 4. Query Optimization
**Problem**: Inefficient database access patterns
**Fix**:
- Batch related queries (movies + stats + translations)
- Implement proper caching for frequently accessed data
- Use streaming for large datasets

## Immediate Actions

1. **Increase SQLite timeout** in ConnectionStringFactory.cs:
   ```csharp
   BusyTimeout = 5000  // Change from 100 to 5000ms
   ```

2. **Add request throttling middleware** to limit concurrent API calls

3. **Optimize MovieController.AllMovie()** to use batch queries

4. **Implement connection pooling** for database operations

5. **Add proper async patterns** throughout the API layer