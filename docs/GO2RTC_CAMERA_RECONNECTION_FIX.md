# go2rtc Camera Reconnection Fix

## Problem Summary

Two related issues were identified with go2rtc stream management:

### Issue 1: Stream Settings Changes Don't Update go2rtc
When a user changes stream settings (URL, protocol, credentials) in the UI, the changes are saved to the database and the HLS stream is restarted, but go2rtc is not notified of the changes. This causes the HLS stream to fail because go2rtc still has the old stream URL registered.

**Symptoms:**
- After changing stream URL or protocol in the UI, HLS streaming fails
- Logs show "Server returned 404 Not Found" errors
- WebRTC may also fail
- Requires manual restart of lightNVR to fix

### Issue 2: Camera Reboots Cause Permanent Stream Loss
When a camera reboots or loses power (common scenario: TP-Link Tapo C320WS cameras reboot daily at 3am), go2rtc does not automatically reconnect to the camera. The HLS thread keeps retrying to connect via go2rtc's RTSP proxy, but go2rtc itself never re-establishes the connection to the upstream camera.

**Symptoms:**
- After camera reboot, HLS and WebRTC streaming stop working
- Logs show repeated "Server returned 404 Not Found" errors
- Requires manual restart of lightNVR to restore streaming
- Recordings may also be affected

## Root Cause

**go2rtc does not automatically reconnect to upstream cameras** when they go offline and come back online. This is a known limitation of go2rtc (see [issue #762](https://github.com/AlexxIT/go2rtc/issues/762)).

When a camera disconnects:
1. go2rtc loses its connection to the camera
2. The HLS thread tries to connect via go2rtc's RTSP proxy (`rtsp://localhost:8554/StreamName`)
3. go2rtc returns 404 because it can't connect to the original camera
4. The HLS thread keeps retrying, but go2rtc never attempts to reconnect to the camera

## Solution

### Fix 1: Update go2rtc When Stream Settings Change

Modified `src/web/api_handlers_streams_modify.c` to unregister and re-register streams with go2rtc when URL or protocol changes:

```c
// If URL or protocol changed, update go2rtc stream registration
if ((url_changed || protocol_changed)) {
    log_info("URL or protocol changed for stream %s, updating go2rtc registration", config.name);
    
    // Unregister the old stream from go2rtc
    if (go2rtc_stream_unregister(config.name)) {
        log_info("Unregistered stream %s from go2rtc", config.name);
    }
    
    // Re-register the stream with the new URL
    const char *username = (config.onvif_username[0] != '\0') ? config.onvif_username : NULL;
    const char *password = (config.onvif_password[0] != '\0') ? config.onvif_password : NULL;
    
    if (go2rtc_stream_register(config.name, config.url, username, password)) {
        log_info("Re-registered stream %s with go2rtc using new URL", config.name);
    }
    
    // Force restart the HLS stream thread if streaming is enabled
    if (config.streaming_enabled) {
        restart_hls_stream(config.name);
    }
}
```

### Fix 2: Implement go2rtc Health Monitor

Created a new health monitoring system that periodically checks stream health and automatically re-registers failed streams with go2rtc:

**New Files:**
- `src/video/go2rtc/go2rtc_health_monitor.c`
- `include/video/go2rtc/go2rtc_health_monitor.h`

**How It Works:**

1. **Background Thread**: Runs every 30 seconds checking all streams
2. **Failure Detection**: Monitors streams in ERROR or RECONNECTING state
3. **Threshold**: After 3 consecutive failures, triggers re-registration
4. **Cooldown**: Waits 60 seconds between re-registration attempts
5. **Automatic Recovery**: Unregisters and re-registers the stream with go2rtc, forcing it to reconnect to the camera

**Integration:**
- Health monitor is initialized when go2rtc integration starts
- Runs in the background throughout the application lifecycle
- Automatically cleans up during shutdown

## Configuration

No configuration changes are required. The health monitor uses these default settings:

- **Check Interval**: 30 seconds
- **Failure Threshold**: 3 consecutive failures
- **Cooldown Period**: 60 seconds between re-registration attempts

These values are defined in `src/video/go2rtc/go2rtc_health_monitor.c` and can be adjusted if needed:

```c
#define HEALTH_CHECK_INTERVAL_SEC 30
#define MAX_CONSECUTIVE_FAILURES 3
#define REREGISTRATION_COOLDOWN_SEC 60
```

## Testing

### Test Case 1: Stream Settings Change
1. Add a camera stream in the UI
2. Verify HLS and WebRTC streaming works
3. Change the stream URL or protocol in the UI
4. Verify streaming continues to work without manual restart

### Test Case 2: Camera Reboot
1. Add a camera stream and verify it's working
2. Disconnect power to the camera (or reboot it)
3. Wait for the camera to come back online
4. Within 1-2 minutes, streaming should automatically resume
5. Check logs for "Re-registered stream X with go2rtc" messages

### Expected Log Messages

**When stream settings change:**
```
[INFO] URL or protocol changed for stream TestCam, updating go2rtc registration
[INFO] Unregistered stream TestCam from go2rtc
[INFO] Re-registered stream TestCam with go2rtc using new URL
[INFO] Force restarting HLS stream thread for TestCam after go2rtc update
```

**When camera reconnects:**
```
[INFO] Stream TestCam has 3 consecutive failures, needs re-registration
[INFO] Attempting to re-register stream TestCam with go2rtc
[INFO] Unregistering stream TestCam from go2rtc
[INFO] Re-registering stream TestCam with go2rtc using URL: rtsp://...
[INFO] Successfully re-registered stream TestCam with go2rtc
[INFO] Successfully re-registered stream TestCam, it should recover soon
```

## Benefits

1. **Automatic Recovery**: Cameras that reboot or lose connection will automatically reconnect
2. **No Manual Intervention**: No need to restart lightNVR when cameras reboot
3. **Seamless Updates**: Stream settings can be changed without interrupting service
4. **Improved Reliability**: Handles common scenarios like scheduled camera reboots
5. **Better User Experience**: Streaming resumes automatically without user action

## Known Limitations

1. **Recovery Time**: May take up to 2 minutes for a stream to recover after camera reboot (30s check interval + retry delays)
2. **go2rtc Dependency**: Still relies on go2rtc for WebRTC and RTSP proxy functionality
3. **Network Issues**: Persistent network problems may prevent automatic recovery

## Future Improvements

Potential enhancements for future versions:

1. Make health check interval configurable via UI
2. Add manual "force reconnect" button in the UI
3. Implement exponential backoff for re-registration attempts
4. Add metrics/statistics for stream health and recovery events
5. Send notifications when streams fail and recover

## References

- go2rtc reconnection issue: https://github.com/AlexxIT/go2rtc/issues/762
- Frigate (similar project) workaround: Uses automation to restart go2rtc
- This implementation: Automatic re-registration without full restart

