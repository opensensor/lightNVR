# ONVIF Detection

This document describes camera-side ONVIF motion/smart-event detection and the
protected ONVIF license-plate-recognition (LPR/ANPR) ingestion path.

## Overview

ONVIF (Open Network Video Interface Forum) is a global standard for IP-based security products. Many IP cameras support ONVIF and provide motion detection events through the ONVIF Events service. The ONVIF detection feature in LightNVR allows you to use these events for motion detection without having to process video frames.

## How It Works

1. LightNVR connects to the camera's ONVIF Events service
2. It creates a subscription for motion events
3. It periodically polls for new events
4. When a motion event is detected, it creates a detection result with a "motion" label
5. This detection result is stored in the database and can trigger recording

License-plate notifications take a separate path. LightNVR recognizes the
Profile M `RuleEngine/Recognition/LicensePlate` topic and fixture-backed vendor
`LicensePlate`, `ANPR`, `ALPR`, and `LPR` topics, parses their structured
attributes, and writes them to protected LPR storage. An LPR notification is
not reduced to generic motion and does not trigger recording by itself.

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
- `src/video/onvif_detection.c`: Implementation of ONVIF detection
- `src/video/onvif_event.c`: Structured standard/vendor LPR parser
- `src/database/db_lpr_reads.c`: Protected persistence and bounded search
- `src/utils/lpr_crypto.c`: Encryption and keyed lookup primitives

The implementation uses the following components:

- CURL for HTTP requests
- mbedTLS for ONVIF authentication plus AES-256-GCM and HMAC-SHA-256 LPR
  protection
- cJSON for parsing responses
- pthread for thread management

## Protected LPR storage

LPR storage fails closed unless `LIGHTNVR_LPR_MASTER_KEY_HEX` contains exactly
64 hexadecimal characters (a 256-bit key). Generate and inject this value from
the deployment's secret manager; never place it in `lightnvr.ini`, a unit file
checked into source control, logs, or a database backup.

```bash
openssl rand -hex 32
```

The canonical plate value is encrypted with AES-256-GCM. Exact search uses a
domain-separated HMAC blind index. Partial search decrypts only a bounded set
of candidates inside the required camera/time range, so the database does not
contain plaintext prefix or n-gram indexes. Other recognition attributes and
recording correlation are stored in the dedicated `lpr_reads` table. Repeated
reads with the same camera, value, source, topic, object/correlation IDs, and
five-second time bucket are suppressed.

The master key must remain stable across restarts and must be backed up
separately under the site's key-management policy. A database restored without
the matching key retains ciphertext but cannot reveal or search exact plate
values. Key rotation/re-encryption is not implemented yet.

The general event bus receives only the restricted
`io.lightnvr.recognition.license_plate.v1` envelope containing `read_id`,
`stream_name`, and `source`. Event-envelope validation rejects data keys
containing `plate`, and audit logging redacts them defensively.

## LPR API

All queries are POST bodies so plate criteria never appear in URLs or access
logs. `camera_uuid`, `start_at`, and `end_at` are required; timestamps are Unix
milliseconds and the maximum range is 366 days.

```http
POST /api/lpr/search
Content-Type: application/json

{
  "camera_uuid": "11111111-1111-4111-8111-111111111111",
  "start_at": 1787920000000,
  "end_at": 1787930000000,
  "match": "exact",
  "plate": "TEST123",
  "limit": 50
}
```

`match` may be omitted, `exact`, or `partial`; partial terms canonicalize to at
least three alphanumeric characters. Search is capped at 100 results.

- `POST /api/lpr/search` requires camera-scoped `lpr.search` and `lpr.read`.
- `POST /api/lpr/export` accepts the same body, is capped at 1,000 JSON rows,
  adds a download disposition, and also requires `lpr.export`.
- `DELETE /api/lpr/reads/{read_uuid}` requires camera-scoped `lpr.delete` and
  permanently removes the protected metadata row. It does not delete video.

Searches and exports audit the match mode, camera/time scope, result count, and
a keyed query fingerprint—never the query value. Deletions audit only the
opaque read ID. The four LPR actions are granted only to the built-in
Administrator role by migration; delegated roles must receive them explicitly.

LPR metadata retention is independent of video retention. The
`lpr_retention_days` system setting defaults to 30, accepts 1–36500, is exposed
through `GET/POST /api/settings`, and is applied by the bounded retention pass.

## Multiple engines

ONVIF can coexist with local motion and object models through the per-stream
detection-engine API. See [Multiple Detection Engines](DETECTION_ENGINES.md).

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

## Current limitations

- No production camera capability is inferred from placement or stream names;
  device-specific Profile M support still requires authenticated read-only
  discovery and a sanitized event fixture.
- The implemented LPR adapter consumes PullPoint notification XML. ONVIF
  metadata RTSP tracks, ONVIF MQTT brokers, and vendor REST/CGI interfaces need
  separate fixture-backed adapters.
- LightNVR does not create or modify camera analytics rules, brokers, exposure,
  illumination, firmware, or other device configuration.
- An ONVIF engine currently runs inside the existing detection-thread
  lifecycle. It can be combined with other engines, but a standalone analytics
  collector independent of detection recording remains future work.
