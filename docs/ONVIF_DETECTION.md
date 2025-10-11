# ONVIF Detection

This document describes the ONVIF detection feature in LightNVR, which allows motion detection using ONVIF events from IP cameras.

## Overview

ONVIF (Open Network Video Interface Forum) is a global standard for IP-based security products. Many IP cameras support ONVIF and provide motion detection events through the ONVIF Events service. The ONVIF detection feature in LightNVR allows you to use these events for motion detection without having to process video frames.

## How It Works

1. LightNVR connects to the camera's ONVIF Events service
2. It creates a subscription for motion events
3. It periodically polls for new events
4. When a motion event is detected, it creates a detection result with a "motion" label
5. This detection result is stored in the database and can trigger recording

## Advantages

- Lower CPU usage compared to video-based motion detection
- More accurate detection since it uses the camera's built-in motion detection
- Works with any ONVIF-compliant camera that supports motion events
- No need to train or configure detection models

## Configuration

To use ONVIF detection, you need to:

1. Configure a stream with the ONVIF camera URL
2. Set the detection model to "onvif"
3. Provide ONVIF credentials (username and password) - **optional for cameras without authentication**

### Example: Camera with Authentication

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

### Example: Camera without Authentication

Some cameras don't require authentication for ONVIF events. For these cameras, you can leave the credentials empty:

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

**Note:** When credentials are empty, LightNVR will send ONVIF requests without WS-Security authentication headers. This is compatible with cameras that don't require authentication.

## Implementation Details

The ONVIF detection feature is implemented in the following files:

- `include/video/onvif_detection.h`: Header file for ONVIF detection
- `include/video/onvif_detection_integration.h`: Header file for ONVIF detection integration
- `src/video/onvif_detection.c`: Implementation of ONVIF detection
- `src/video/onvif_detection_integration.c`: Integration with the detection system

The implementation uses the following components:

- CURL for HTTP requests
- mbedTLS for cryptographic operations (SHA-1 hashing, Base64 encoding, random number generation)
- cJSON for parsing responses
- pthread for thread management

## Testing

You can test the ONVIF detection feature using the provided test script:

```bash
./test_onvif_detection.sh
```

This script creates a test stream configuration and provides instructions for testing.

## Troubleshooting

If you encounter issues with ONVIF detection:

1. Check that your camera supports ONVIF Events
2. Verify your ONVIF credentials (or try without credentials if your camera doesn't require authentication)
3. Make sure the camera is accessible from LightNVR
4. Check the logs for error messages

Common error messages:

- "Failed to create subscription": The camera may not support ONVIF Events, the credentials are incorrect, or the camera requires authentication but none was provided
- "Failed to pull messages": The subscription may have expired or the camera is not accessible
- "Failed to extract service name": The subscription address format is not recognized
- "Camera may require authentication": Try configuring ONVIF credentials in the stream settings
- "ONVIF detection failed": Check camera connectivity, ONVIF support, and credentials

### Testing Authentication Requirements

If you're unsure whether your camera requires authentication:

1. First try with empty credentials (`onvif_username: ""`, `onvif_password: ""`)
2. If that fails with authentication errors, configure the proper credentials
3. Check the logs - they will indicate whether authentication is being used

## Future Improvements

- Support for other ONVIF event types (not just motion)
- Automatic discovery of ONVIF cameras
- Configuration of motion detection parameters through ONVIF
- Support for ONVIF Analytics events
