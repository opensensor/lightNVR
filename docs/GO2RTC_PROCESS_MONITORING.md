# go2rtc Process Health Monitoring

## Overview

The go2rtc process health monitoring system automatically detects when the go2rtc service becomes unresponsive (zombie state) and restarts it to restore functionality. This addresses the issue where go2rtc continues running but stops responding to API requests and stream connections.

## Problem Statement

go2rtc can enter a "zombie" state where:
- The process is still running (visible in `ps`)
- The API on port 1984 becomes unresponsive
- All stream connections fail
- lightNVR remains healthy and operational

This creates a situation where streams appear to be down, but the root cause is the go2rtc service itself.

## Solution

The process health monitor uses multiple indicators to detect go2rtc failures:

### 1. API Health Check
- Periodically checks the go2rtc API on port 1984
- Uses the existing `go2rtc_stream_is_ready()` function
- Tracks consecutive API failures

### 2. Stream Consensus
- Monitors the state of all enabled streams
- If ALL streams are in ERROR or RECONNECTING state, it indicates a go2rtc issue
- Requires at least 2 streams to make a consensus decision
- Prevents false positives from individual camera failures

### 3. Automatic Restart
When both indicators suggest go2rtc is unhealthy:
1. Stops the current go2rtc process
2. Waits for cleanup (2 seconds)
3. Starts a new go2rtc process
4. Waits for the service to become ready
5. Re-registers all streams with the new process

## Configuration

The monitor uses these default settings (defined in `src/video/go2rtc/go2rtc_process_monitor.c`):

```c
#define PROCESS_HEALTH_CHECK_INTERVAL_SEC 30    // Check every 30 seconds
#define MAX_API_FAILURES 3                       // Restart after 3 consecutive failures
#define MIN_STREAMS_FOR_CONSENSUS 2              // Need at least 2 streams for consensus
#define RESTART_COOLDOWN_SEC 120                 // Wait 2 minutes between restarts
#define MAX_RESTARTS_PER_WINDOW 5                // Max 5 restarts per window
#define RESTART_WINDOW_SEC 600                   // 10-minute window
```

### Tuning Parameters

**PROCESS_HEALTH_CHECK_INTERVAL_SEC**: How often to check go2rtc health
- Lower values = faster detection
- Higher values = less overhead
- Recommended: 30-60 seconds

**MAX_API_FAILURES**: Number of consecutive failures before restart
- Lower values = faster recovery but more false positives
- Higher values = slower recovery but fewer false positives
- Recommended: 3-5 failures

**RESTART_COOLDOWN_SEC**: Minimum time between restarts
- Prevents restart loops
- Should be long enough for go2rtc to fully initialize
- Recommended: 120-180 seconds

**MAX_RESTARTS_PER_WINDOW**: Maximum restarts in the time window
- Prevents infinite restart loops
- If exceeded, manual intervention is required
- Recommended: 5 restarts per 10 minutes

## Rate Limiting

The monitor includes sophisticated rate limiting to prevent restart loops:

1. **Cooldown Period**: After each restart, waits `RESTART_COOLDOWN_SEC` before allowing another restart
2. **Window-Based Limiting**: Tracks restart history and prevents more than `MAX_RESTARTS_PER_WINDOW` restarts within `RESTART_WINDOW_SEC`
3. **Logging**: All restart attempts and rate limit blocks are logged for debugging

## Integration

The process monitor is automatically initialized when go2rtc integration starts:

```c
// In go2rtc_integration_init()
if (!go2rtc_process_monitor_init()) {
    log_warn("Failed to initialize go2rtc process health monitor (non-fatal)");
} else {
    log_info("go2rtc process health monitor initialized successfully");
}
```

And cleaned up during shutdown:

```c
// In go2rtc_integration_cleanup()
go2rtc_process_monitor_cleanup();
```

## Monitoring and Debugging

### Log Messages

**Normal Operation**:
```
go2rtc process health monitor initialized successfully
go2rtc API health check succeeded, resetting failure counter
```

**Detecting Issues**:
```
go2rtc API health check failed (consecutive failures: 1/3)
Stream consensus: 3/3 streams failed - indicates go2rtc issue
```

**Restart Process**:
```
Attempting to restart go2rtc process due to health check failure
Stopping go2rtc process...
Starting go2rtc process...
go2rtc process restarted successfully
Re-registering all streams with go2rtc after restart
go2rtc restart completed (total restarts: 1)
```

**Rate Limiting**:
```
go2rtc restart blocked: cooldown period (45 seconds remaining)
go2rtc restart blocked: too many restarts (5 in last 600 seconds)
```

### API Functions

The monitor exposes these functions for monitoring and debugging:

```c
// Check if monitor is running
bool go2rtc_process_monitor_is_running(void);

// Get restart statistics
int go2rtc_process_monitor_get_restart_count(void);
time_t go2rtc_process_monitor_get_last_restart_time(void);

// Force a health check (for testing)
bool go2rtc_process_monitor_check_health(void);
```

## Files

**New Files**:
- `include/video/go2rtc/go2rtc_process_monitor.h` - Header file
- `src/video/go2rtc/go2rtc_process_monitor.c` - Implementation

**Modified Files**:
- `src/video/go2rtc/go2rtc_integration.c` - Integration with main system
- `src/video/go2rtc/CMakeLists.txt` - Build system
- `include/video/go2rtc/go2rtc_stream.h` - Added `go2rtc_stream_get_api_port()`
- `src/video/go2rtc/go2rtc_stream.c` - Implemented `go2rtc_stream_get_api_port()`

## Testing

To test the monitor:

1. **Simulate go2rtc failure**: Kill the go2rtc process or block port 1984
2. **Monitor logs**: Watch for health check failures and restart attempts
3. **Verify recovery**: Confirm streams reconnect after restart
4. **Check rate limiting**: Trigger multiple failures to test cooldown and window limits

## Future Enhancements

Potential improvements:
- Expose restart count via API endpoint for monitoring dashboards
- Add configurable thresholds via configuration file
- Implement exponential backoff for restart attempts
- Add metrics collection for restart frequency analysis

