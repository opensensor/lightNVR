# ONVIF Credentials Fix - Support for Cameras Without Authentication

## Problem

Some IP cameras support ONVIF events but don't require authentication. Previously, LightNVR required non-empty ONVIF credentials, which prevented these cameras from working with ONVIF-based motion detection.

The issues were:
1. **Validation too strict**: Code checked for non-empty username and password strings before attempting ONVIF detection
2. **Always sent WS-Security headers**: Even when credentials were empty, the code would try to create authentication headers
3. **Unclear error messages**: Users couldn't tell if the issue was missing credentials or a connection problem

## Solution

The fix allows ONVIF detection to work with or without credentials:

### 1. Modified SOAP Request Creation (`src/video/onvif_detection.c`)

- **Before**: Always created SOAP requests with WS-Security authentication headers
- **After**: Checks if credentials are provided (non-empty strings)
  - If credentials are empty: Creates SOAP request without WS-Security headers
  - If credentials are provided: Creates SOAP request with WS-Security authentication

```c
// Check if credentials are provided (non-empty strings)
bool has_credentials = (username && strlen(username) > 0 && password && strlen(password) > 0);

if (!has_credentials) {
    // Create SOAP request without WS-Security headers
    log_info("Creating ONVIF request without authentication (no credentials provided)");
    // ... simple SOAP envelope without security headers
} else {
    // Create SOAP request with WS-Security headers
    log_info("Creating ONVIF request with WS-Security authentication");
    // ... full SOAP envelope with authentication
}
```

### 2. Updated Validation Logic (`src/video/onvif_detection.c`)

- **Before**: Rejected requests if username or password were NULL
- **After**: 
  - Still rejects NULL pointers (for safety)
  - Allows empty strings (for cameras without authentication)
  - Logs credential status for debugging

```c
// Validate parameters - allow empty credentials (empty strings) but not NULL pointers
if (!onvif_url || !username || !password || !result) {
    log_error("Invalid parameters for detect_motion_onvif (NULL pointers not allowed)");
    return -1;
}

// Log credential status for debugging
if (strlen(username) == 0 || strlen(password) == 0) {
    log_info("ONVIF Detection: Using camera without authentication (empty credentials)");
} else {
    log_info("ONVIF Detection: Using camera with authentication (username: %s)", username);
}
```

### 3. Improved Detection Flow (`src/video/detection_stream_thread_helpers.c`)

- **Before**: Only called ONVIF detection if URL, username, AND password were all non-empty
- **After**: Calls ONVIF detection if URL is present (credentials are optional)

```c
// Call ONVIF detection if we have a valid URL (credentials are optional for some cameras)
if (url[0] != '\0') {
    if (username[0] != '\0' && password[0] != '\0') {
        log_info("[Stream %s] Calling ONVIF detection with URL: %s, username: %s",
                thread->stream_name, url, username);
    } else {
        log_info("[Stream %s] Calling ONVIF detection with URL: %s (no credentials - camera may not require authentication)",
                thread->stream_name, url);
    }
    
    // Call the ONVIF detection function
    result = detect_motion_onvif(url, username, password, &result_struct, thread->stream_name);
    // ...
}
```

### 4. Better Error Messages (`src/video/detection_stream_thread_helpers.c`)

- **Before**: Generic error message
- **After**: Context-aware error messages that help users diagnose the issue

```c
if (username[0] == '\0' || password[0] == '\0') {
    log_error("[Stream %s] ONVIF detection failed (error code: %d). "
             "Camera may require authentication. Please configure ONVIF credentials in stream settings.",
             thread->stream_name, result);
} else {
    log_error("[Stream %s] ONVIF detection failed (error code: %d). "
             "Check camera connectivity, ONVIF support, and credentials.",
             thread->stream_name, result);
}
```

### 5. Updated Documentation (`docs/ONVIF_DETECTION.md`)

Added examples and clarification:
- Example configuration for cameras **with** authentication
- Example configuration for cameras **without** authentication
- Troubleshooting guide for testing authentication requirements
- Clear note that credentials are optional

## Usage

### For Cameras Without Authentication

Configure the stream with empty ONVIF credentials:

```json
{
  "name": "onvif_camera_no_auth",
  "url": "rtsp://camera_ip:554/stream",
  "enabled": true,
  "detection_model": "onvif",
  "onvif_username": "",
  "onvif_password": "",
  "is_onvif": true
}
```

### For Cameras With Authentication

Configure the stream with ONVIF credentials:

```json
{
  "name": "onvif_camera",
  "url": "onvif://username:password@camera_ip",
  "enabled": true,
  "detection_model": "onvif",
  "onvif_username": "username",
  "onvif_password": "password",
  "is_onvif": true
}
```

## Testing

1. **Test without credentials first**: If you're unsure whether your camera requires authentication, try with empty credentials
2. **Check the logs**: The logs will clearly indicate whether authentication is being used
3. **If it fails**: Try adding credentials if the error message suggests authentication is required

## Benefits

1. **Broader camera compatibility**: Works with cameras that don't require ONVIF authentication
2. **Better user experience**: Clear error messages help users diagnose issues
3. **Flexible configuration**: Users can choose to use authentication or not
4. **Backward compatible**: Existing configurations with credentials continue to work

## Files Modified

1. `src/video/onvif_detection.c` - Core ONVIF detection logic
2. `src/video/detection_stream_thread_helpers.c` - Detection thread integration
3. `docs/ONVIF_DETECTION.md` - Documentation updates

## Technical Details

### SOAP Request Format

**Without Authentication:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope">
  <s:Header/>
  <s:Body xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
    <!-- Request body -->
  </s:Body>
</s:Envelope>
```

**With Authentication:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope" 
            xmlns:wsse="..." xmlns:wsu="...">
  <s:Header>
    <wsse:Security s:mustUnderstand="1">
      <wsse:UsernameToken wsu:Id="UsernameToken-1">
        <wsse:Username>username</wsse:Username>
        <wsse:Password Type="...">digest</wsse:Password>
        <wsse:Nonce EncodingType="...">nonce</wsse:Nonce>
        <wsu:Created>timestamp</wsu:Created>
      </wsse:UsernameToken>
    </wsse:Security>
  </s:Header>
  <s:Body xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns:xsd="http://www.w3.org/2001/XMLSchema">
    <!-- Request body -->
  </s:Body>
</s:Envelope>
```

The key difference is the presence or absence of the `<wsse:Security>` header in the SOAP envelope.

