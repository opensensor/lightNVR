# go2rtc Settings Reload Fix

## Problem

When users toggled the "Enable go2rtc (WebRTC/MSE)" setting (the `webrtc_disabled` option) in the Settings page, the change was saved to the configuration file but did not take effect until lightNVR was completely restarted. This was inconvenient and required manual intervention.

## Root Cause

The settings save handler (`handle_post_settings` in `src/web/api_handlers_settings.c`) was:
1. Saving the `webrtc_disabled` setting to the config file
2. Reloading the configuration into memory

However, it was NOT:
1. Regenerating the go2rtc configuration file (`go2rtc.yaml`) with the updated settings
2. Restarting the go2rtc process to pick up the new configuration

This meant that even though the lightNVR configuration was updated, go2rtc continued running with its old configuration, and the WebRTC/MSE functionality didn't change until a full system restart.

## Solution

Modified `src/web/api_handlers_settings.c` to:

1. **Track go2rtc-related setting changes**: Added a `go2rtc_config_changed` flag that is set when `webrtc_disabled` changes
2. **Detect actual changes**: Compare the old and new values of `webrtc_disabled` to only trigger restart when the value actually changes
3. **Restart go2rtc automatically**: After saving and reloading the configuration, if go2rtc settings changed:
   - Stop the go2rtc process
   - Regenerate the go2rtc configuration file with the new settings
   - Restart the go2rtc process
   - Re-register all active streams with go2rtc

## Changes Made

### File: `src/web/api_handlers_settings.c`

#### 1. Added includes for go2rtc modules
```c
#include "video/go2rtc/go2rtc_process.h"
#include "video/go2rtc/go2rtc_integration.h"
```

#### 2. Added change tracking
```c
bool go2rtc_config_changed = false;  // Track if go2rtc-related settings changed
```

#### 3. Modified webrtc_disabled handling to detect changes
```c
// WebRTC disabled - track old value to detect changes
cJSON *webrtc_disabled = cJSON_GetObjectItem(settings, "webrtc_disabled");
if (webrtc_disabled && cJSON_IsBool(webrtc_disabled)) {
    bool old_webrtc_disabled = g_config.webrtc_disabled;
    g_config.webrtc_disabled = cJSON_IsTrue(webrtc_disabled);
    settings_changed = true;
    log_info("Updated webrtc_disabled: %s", g_config.webrtc_disabled ? "true" : "false");
    
    // If webrtc_disabled changed, we need to restart go2rtc
    if (old_webrtc_disabled != g_config.webrtc_disabled) {
        go2rtc_config_changed = true;
        log_info("WebRTC disabled setting changed, will restart go2rtc");
    }
}
```

#### 4. Added go2rtc restart logic after config reload
After the configuration is saved and reloaded, the handler now:
- Checks if `go2rtc_config_changed` is true
- Stops the go2rtc process if running
- Regenerates the go2rtc configuration file
- Restarts the go2rtc process
- Re-registers all active streams with go2rtc

## Benefits

1. **Immediate effect**: Changes to the WebRTC/MSE setting now take effect immediately without requiring a full lightNVR restart
2. **Better user experience**: Users can toggle between HLS and WebRTC/MSE modes seamlessly
3. **Automatic stream re-registration**: All active streams are automatically re-registered with go2rtc after the restart
4. **Proper cleanup**: The old go2rtc process is cleanly stopped before starting the new one

## Testing

To test this fix:

1. Start lightNVR with go2rtc enabled (webrtc_disabled = false)
2. Navigate to Settings page
3. Toggle "Enable go2rtc (WebRTC/MSE)" checkbox
4. Click Save
5. Verify that:
   - Settings save successfully
   - go2rtc process restarts automatically
   - Streams continue working with the new mode (HLS or WebRTC/MSE)
   - No manual restart of lightNVR is required

## Future Enhancements

This pattern could be extended to handle other go2rtc-related settings that might change:
- WebRTC port changes
- STUN server configuration changes
- ICE server configuration changes
- Authentication settings changes

The same approach (detect change → regenerate config → restart process) can be applied to any go2rtc configuration parameter.

