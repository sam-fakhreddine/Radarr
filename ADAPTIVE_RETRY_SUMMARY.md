# Adaptive Retry Implementation for API Throttling

## Overview

Added automatic retry logic with exponential backoff to handle 503 Service Unavailable responses from the API throttling middleware.

## Implementation Details

### HTTP Client Changes
**File**: `src/NzbDrone.Common/Http/HttpClient.cs`

**New Method**: `ExecuteRequestWithRetryAsync`
- Wraps all HTTP requests with retry logic
- Only retries on 503 status code
- Uses exponential backoff strategy

**Configuration**:
```csharp
MaxRetries = 3           // Total of 4 attempts (initial + 3 retries)
BaseRetryDelayMs = 1000  // 1 second base delay
```

**Retry Delays**:
- Attempt 1: Immediate
- Attempt 2: 1 second delay (1000ms * 2^0)
- Attempt 3: 2 second delay (1000ms * 2^1)
- Attempt 4: 4 second delay (1000ms * 2^2)

**Total Max Wait Time**: 7 seconds (1s + 2s + 4s)

## Benefits

### 1. Transparent Retry
- All internal API calls automatically retry on throttling
- No code changes needed in calling code
- Works for CLI tools, scheduled tasks, and internal services

### 2. Exponential Backoff
- Prevents thundering herd problem
- Gives system time to recover
- Reduces load during high concurrency

### 3. Graceful Degradation
- After max retries, returns 503 to caller
- Caller can implement additional retry logic if needed
- Prevents infinite retry loops

## Testing

### Unit Tests
**File**: `src/NzbDrone.Common.Test/Http/HttpClientRetryFixture.cs`

**Test Coverage**:
1. `should_retry_on_503_response`: Verifies retry happens on 503
2. `should_not_retry_on_non_503_errors`: Ensures only 503 triggers retry
3. `should_give_up_after_max_retries`: Validates max retry limit

**Results**: All 3 tests passing ✓

## Example Scenarios

### Scenario 1: CLI Tool Making Rapid Requests
**Before**:
- Request 11 hits throttle → 503 error
- CLI tool fails immediately

**After**:
- Request 11 hits throttle → 503 response
- Waits 1 second, retries
- If still throttled, waits 2 seconds, retries
- If still throttled, waits 4 seconds, retries
- Success or final 503 after 7 seconds total

### Scenario 2: Scheduled Task Burst
**Before**:
- Multiple tasks start simultaneously
- Some get 503 errors
- Tasks fail and need manual retry

**After**:
- Multiple tasks start simultaneously
- Throttled tasks automatically retry with backoff
- Most succeed without intervention
- Only persistent overload results in failure

## Configuration

No configuration needed - retry logic is automatic for all HTTP requests.

To disable retry for specific requests (if needed in future):
```csharp
// Future enhancement - not currently implemented
request.DisableRetry = true;
```

## Monitoring

Retry attempts are logged at DEBUG level:
```
Request throttled (503), retrying in 1000ms (attempt 1/3)
Request throttled (503), retrying in 2000ms (attempt 2/3)
Request throttled (503), retrying in 4000ms (attempt 3/3)
```

## Performance Impact

- **Minimal overhead**: Only adds retry logic, no performance cost for successful requests
- **Reduced failures**: Automatic retry reduces failed operations
- **Better resource utilization**: Exponential backoff prevents system overload

## Breaking Changes

**None** - This is a pure enhancement:
- Existing behavior unchanged for successful requests
- Failed requests now have automatic retry
- No API contract changes
- No configuration changes required
