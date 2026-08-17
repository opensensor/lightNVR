# LightNVR API Documentation

This document describes the REST API endpoints provided by LightNVR.

## API Overview

LightNVR provides a RESTful API that allows you to interact with the system programmatically. The API is accessible via HTTP and returns JSON responses using the cJSON library. The API is served by the libuv + llhttp web server.

## Authentication

If authentication is enabled in the configuration file, endpoints that require
authentication accept one of three mechanisms. Authentication and role-based
access checks are enforced per-endpoint. Where authentication is required, all
three mechanisms resolve to the same `user_t`, so pick whichever fits the
caller.

### 1. Session cookie (interactive UI, scripts that log in once)

Obtain a session by calling the login endpoint, then include the session
cookie on subsequent requests.

```bash
curl -c cookies.txt -X POST -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"yourpassword"}' \
  http://your-lightnvr-ip:8080/api/auth/login

curl -b cookies.txt http://your-lightnvr-ip:8080/api/streams
```

### 2. API key (automation, long-lived integrations)

Every user has an `api_key` field. Pass it via either header:

```bash
# Preferred
curl -H "X-API-Key: <your-api-key>" http://your-lightnvr-ip:8080/api/streams

# Also accepted
curl -H "Authorization: Bearer <your-api-key>" http://your-lightnvr-ip:8080/api/streams
```

For automation (Home Assistant, NodeRED, cron jobs, etc.), create a dedicated
user with the `USER_ROLE_API` role and use that user's `api_key`. Keep admin
keys out of automation configs.

### 3. HTTP Basic Auth

Also supported as a fallback for tools that only understand basic auth.

### Roles

Role-based access is enforced per-endpoint. The table below is a high-level
summary of the intended access model:

| Role     | Can read | Can write | Can administer |
|----------|:--------:|:---------:|:--------------:|
| `ADMIN`  | yes      | yes       | yes            |
| `USER`   | yes      | yes       | no             |
| `API`    | yes      | yes       | no             |
| `VIEWER` | yes      | typically no | no          |

Write authorization is endpoint-specific. Stream creation, updates (including
privacy mode), and deletion reject `VIEWER` with `403`, as does
[`POST /api/motion/trigger`](#trigger-motion-event).

## API Endpoints

### Streams

#### List Streams

```
GET /api/streams
```

Returns a list of all configured streams.

When authentication is enabled, `VIEWER` responses contain the operational
fields needed by live view but redact camera credentials and administrative
connection settings (`onvif_username`, `onvif_password`, `admin_url`,
`sub_stream_url`, `detection_url`, `publish_url`, and source overrides). URL
credentials are stripped from `url`. `has_sub_stream` preserves the boolean
capability without revealing that URL, and `can_control_privacy` tells clients
whether to show privacy controls.

**Response:**
```json
{
  "streams": [
    {
      "id": 0,
      "name": "Front Door",
      "url": "rtsp://192.168.1.100:554/stream1",
      "enabled": true,
      "streaming_enabled": true,
      "width": 1920,
      "height": 1080,
      "fps": 15,
      "codec": "h264",
      "priority": 10,
      "record": true,
      "segment_duration": 900,
      "protocol": 0,
      "record_audio": true,
      "detection_based_recording": 0,
      "detection_model": "",
      "detection_threshold": 0.5,
      "detection_interval": 10,
      "pre_detection_buffer": 0,
      "post_detection_buffer": 3,
      "detection_api_url": "",
      "is_onvif": false,
      "onvif_username": "",
      "onvif_password": "",
      "onvif_profile": "",
      "ptz_enabled": false,
      "backchannel_enabled": false,
      "buffer_strategy": "auto",
      "retention_days": -1,
      "detection_retention_days": -1,
      "record_on_schedule": false,
      "detection_record_on_schedule": false,
      "status": "connected",
      "has_sub_stream": false,
      "can_control_privacy": true
    }
  ]
}
```

#### Get Stream

```
GET /api/streams/{name}
```

Returns information about a specific stream by name.

#### Get Full Stream

```
GET /api/streams/{name}/full
```

Returns complete stream information including all configuration fields.

Per-stream retention values are tri-state: `-1` inherits the current global
retention value, `0` is unlimited, and a positive value is a day override.
Continuous and detection-triggered recording have independent weekly schedule
toggles and 168-element Sunday-through-Saturday hour grids.

#### Manual Recording Control

```
GET /api/streams/{name}/recording
POST /api/streams/{name}/recording
```

`GET` returns `idle`, `starting`, or `recording`, along with the active
`capture_method`, recording ID, and whether the caller may use manual control.
It is available to viewers who may access the stream.

`POST` accepts `{"action":"start"}` or `{"action":"stop"}`. `ADMIN`, `USER`,
and `API` roles may call it; `VIEWER` is read-only. Manual start returns `409`
if continuous, detection, or another manual recording is active. Manual stop
returns `409` unless the active recording was itself started manually, so it
never interrupts continuous, scheduled, or detection capture.

#### Add Stream

```
POST /api/streams
```

Adds a new stream. All fields from the stream schema are accepted.

#### Update Stream

```
PUT /api/streams/{name}
```

Updates an existing stream.

#### Delete Stream

```
DELETE /api/streams/{name}
```

Deletes a stream.

#### Test Stream

```
POST /api/streams/test
```

Tests connectivity to a stream URL.

#### Refresh Stream

```
POST /api/streams/{name}/refresh
```

Forces a shared source reconnection. Duplicate requests for the same stream are
coalesced while a refresh is active and for a 30-second cooldown afterward;
coalesced requests return `202` with `coalesced: true`.

### Stream Retention

#### Get Stream Retention

```
GET /api/streams/{name}/retention
```

Returns retention settings for a specific stream.

#### Update Stream Retention

```
PUT /api/streams/{name}/retention
```

Updates retention settings for a specific stream.

### Detection Zones

#### List Zones

```
GET /api/streams/{name}/zones
```

Returns detection zones for a stream.

#### Create/Update Zones

```
POST /api/streams/{name}/zones
```

Creates or updates detection zones for a stream.

#### Delete Zones

```
DELETE /api/streams/{name}/zones
```

Deletes detection zones for a stream.

### PTZ (Pan-Tilt-Zoom)

#### Get PTZ Capabilities

```
GET /api/streams/{name}/ptz/capabilities
```

Returns PTZ capabilities for a stream's camera.

#### PTZ Move

```
POST /api/streams/{name}/ptz/move
```

Starts continuous PTZ movement.

#### PTZ Stop

```
POST /api/streams/{name}/ptz/stop
```

Stops PTZ movement.

#### PTZ Absolute Move

```
POST /api/streams/{name}/ptz/absolute
```

Moves to an absolute PTZ position.

#### PTZ Relative Move

```
POST /api/streams/{name}/ptz/relative
```

Performs a relative PTZ movement.

#### PTZ Home

```
POST /api/streams/{name}/ptz/home
```

Moves to the home position.

#### PTZ Set Home

```
POST /api/streams/{name}/ptz/set-home
```

Sets the current position as home.

#### PTZ Presets

```
GET /api/streams/{name}/ptz/presets
```

Lists PTZ presets.

```
POST /api/streams/{name}/ptz/goto-preset
```

Moves to a PTZ preset.

```
PUT /api/streams/{name}/ptz/preset
```

Creates or updates a PTZ preset.

### Motion

#### Trigger Motion Event

```
POST /api/motion/trigger
```

Drives the same motion-recording path that ONVIF events normally drive. Useful
for cameras whose ONVIF event stream is unreliable or missing — the caller
(Home Assistant, NodeRED, a shell script, a PIR sensor bridge) can post a
motion event and the target stream's detection-based recording pipeline
(pre-buffer → recording → post-buffer) handles the rest.

The trigger has two halves, and they are independent:

- **Starting a recording** requires `detection_based_recording` on the target
  stream — that is the pipeline the trigger drives. Without it there is no
  unified detection thread to arm, and the response reports
  `recording_triggered: false`.
- **Annotating the event** (`label` / `objects` / `tags`) always happens. A
  stream on 24/7 recording has nothing to trigger but still gets the detection
  written to the database, published over MQTT, and drawn on the timeline.

**Choosing the stream configuration:**

There are two sensible setups, depending on whether you want the trigger to
start recordings or just annotate them:

| Goal | `detection_based_recording` | `detection_model` |
|------|------------------------------|-------------------|
| The API event starts and stops the recording | enabled | leave **empty** |
| 24/7 recording, API events only mark the timeline | disabled | n/a |

For the first row, leave the detection model unset: the unified detection
thread runs and responds to external triggers, but performs no local inference,
so nothing is spent on a model whose output you are not using. Picking a model
here means LightNVR *also* runs that detector on the stream, which is only what
you want if you intend to combine local detection with external events.

**Authentication:** See [Authentication](#authentication). For automation use
a dedicated `USER_ROLE_API` user and its `api_key`; `USER_ROLE_VIEWER` is
rejected with 403.

**Request body:**

| Field         | Type    | Required | Notes                                                   |
|---------------|---------|----------|---------------------------------------------------------|
| `stream`      | string  | yes      | Stream name, as shown in the streams list.              |
| `action`      | string  | yes      | One of `start`, `stop`, `pulse`.                        |
| `duration_ms` | integer | no       | Only valid with `pulse`. Default 2000, max 600000 (10m).|
| `label`       | string  | no       | Single object class for the event, e.g. `person`.       |
| `objects`     | array   | no       | Several classes at once. See below.                     |
| `confidence`  | number  | no       | 0–1, default 1.0. Applies to `label` and to `objects` entries without their own. |
| `tags`        | array   | no       | Strings applied to the stream's currently-open recording. Max 16. |

**Reporting what was seen (`label` / `objects`):**

Object classes turn a bare "something moved" into the same shape as a model
detection, so they appear on the timeline, in the detections API, and in the
MQTT payload where Home Assistant can filter them
(`selectattr('label','eq','person')`) exactly like ONVIF smart events and
model output. Use the same label vocabulary the detectors use (`person`,
`vehicle`, `bicycle`, `face`, `animal`, `motion`).

`objects` accepts bare strings or per-entry confidences — forward detector
output verbatim, or keep it terse:

```json
{"stream":"Front Door","action":"pulse","objects":["person","vehicle"]}
```
```json
{"stream":"Front Door","action":"pulse",
 "objects":[{"label":"person","confidence":0.91},{"label":"vehicle","confidence":0.64}]}
```

Detections are recorded on the leading edge only (`start` and `pulse`); a
`stop` closes the persisted event interval and reports nothing new about what
was seen. The interval spans every recording segment between those edges and
is drawn at its exact start/stop time on the timeline. A trigger without
`label` or `objects` is stored as a generic `motion` event. Supplied objects
cover the whole frame, since an external trigger carries no bounding box, and
are passed through the stream's zone filter just like model detections — a
detection filtered out by zones is not stored, so `detections_stored` may be
lower than what you sent. At most 20 objects are recorded per event; extras are
ignored.

**Tags:**

`tags` are applied to the recording that is currently open for the stream. On a
24/7 stream that is the segment the event falls inside. With detection-based
recording no segment is open at the instant the trigger arrives, so the tags are
not applied and `tags_applied` comes back `0` — tag those recordings via
[`PUT /api/recordings/{id}/tags`](#recordings) once the recording exists.

**Actions:**

- `start` — motion began. Equivalent to an ONVIF motion-start event. Recording
  begins (after the pre-buffer window) and continues until a corresponding
  `stop` arrives, at which point the UDT transitions into the post-buffer
  hold and then closes the recording.
- `stop` — motion ended. Mirror of `start`.
- `pulse` — convenience: fires `start`, waits `duration_ms`, then fires `stop`.
  One request produces one complete motion event. Ideal for motion sensors that
  only report a single edge (e.g. a Home Assistant automation that fires when a
  PIR sensor goes active but does not send a separate "cleared" webhook).

**Response:** `202 Accepted`

```json
{
  "success": true,
  "stream": "Front Door",
  "action": "pulse",
  "duration_ms": 5000,
  "recording_triggered": true,
  "detections_stored": 2,
  "tags_applied": 0
}
```

When the stream has no detection-based recording, the event is still recorded
and a `warning` is included:

```json
{
  "success": true,
  "stream": "Driveway",
  "action": "pulse",
  "duration_ms": 5000,
  "recording_triggered": false,
  "detections_stored": 1,
  "tags_applied": 2,
  "warning": "Stream does not have detection-based recording enabled; the event was recorded but no recording was triggered"
}
```

**Errors:**

- `400` — missing/invalid JSON, missing `stream`/`action`, bad `duration_ms`,
  malformed `label`/`objects`/`confidence`/`tags`.
- `401` — auth enabled and caller is not authenticated.
- `403` — caller is `USER_ROLE_VIEWER`.
- `404` — `stream` does not match a configured stream.

> **Changed in 0.36.5:** a stream without `detection_based_recording` no longer
> returns `409`. The request is accepted and `recording_triggered: false` is
> reported instead, so 24/7-recording streams can annotate their timeline.

**Examples:**

Start/stop (matches how an ONVIF sensor would report):
```bash
curl -X POST http://lightnvr:8080/api/motion/trigger \
  -H "X-API-Key: <key>" -H "Content-Type: application/json" \
  -d '{"stream":"Front Door","action":"start"}'

# ...later, when motion clears:
curl -X POST http://lightnvr:8080/api/motion/trigger \
  -H "X-API-Key: <key>" -H "Content-Type: application/json" \
  -d '{"stream":"Front Door","action":"stop"}'
```

One-shot pulse (Home Assistant rest_command with a PIR sensor):
```yaml
rest_command:
  lightnvr_motion:
    url: "http://lightnvr:8080/api/motion/trigger"
    method: POST
    headers:
      X-API-Key: !secret lightnvr_api_key
    content_type: "application/json"
    payload: '{"stream":"{{ stream }}","action":"pulse","duration_ms":{{ duration | default(5000) }}}'
```

Annotated event from an external detector (what lands on the timeline):
```bash
curl -X POST http://lightnvr:8080/api/motion/trigger \
  -H "X-API-Key: <key>" -H "Content-Type: application/json" \
  -d '{"stream":"Driveway","action":"pulse","duration_ms":10000,
       "objects":[{"label":"person","confidence":0.93}],
       "tags":["frigate","front-gate"]}'
```

Cross-stream linking (`motion_trigger_source` in stream config) is honoured:
an API-driven motion event on one stream will also drive recording on any
stream that lists it as a trigger source, same as an ONVIF-driven event. This
happens regardless of whether the source stream itself has detection-based
recording enabled.

### Recordings

#### List Recordings

```
GET /api/recordings
```

Returns a list of recordings. Supports query parameters for filtering by stream name, date range, and pagination.

`capture_method` is `continuous` for always-on recording, `scheduled` when
continuous capture is gated by a weekly schedule, or `detection`, `motion`, or
`manual` for triggered capture. Responses also include `schedule_restricted`:
`true` when the capture mode was gated by a weekly schedule, `false` when it was
unrestricted, and `null` for recordings created before that metadata existed.
Legacy rows stored as `scheduled` are reported as `continuous` only when their
metadata proves they were unrestricted. Timeline segment responses expose the
same nullable field. Timeline responses also include `detection_intervals`, an
array of exact external-motion `start_timestamp`/`end_timestamp` ranges for
the requested window.

#### Get Recording

```
GET /api/recordings/{id}
```

Returns information about a specific recording.

#### Delete Recording

```
DELETE /api/recordings/{id}
```

Deletes a recording.

#### Play Recording

```
GET /api/recordings/play/{id}
```

Streams a recording for playback.

#### Download Recording

```
GET /api/recordings/download/{id}
```

Downloads a recording file.

#### Protect Recording

```
PUT /api/recordings/{id}/protect
```

Toggles protection status on a recording (protected recordings are exempt from auto-deletion).

#### Set Recording Retention

```
PUT /api/recordings/{id}/retention
```

Sets a per-recording retention override.

#### Batch Delete Recordings

```
POST /api/recordings/batch-delete
```

Deletes multiple recordings at once. Returns a job ID for progress tracking.

#### Batch Delete Progress

```
GET /api/recordings/batch-delete/progress/{job_id}
```

Returns progress for a batch delete operation.

#### Batch Protect Recordings

```
POST /api/recordings/batch-protect
```

Protects or unprotects multiple recordings at once.

#### Get Protected Recordings

```
GET /api/recordings/protected
```

Returns all protected recordings.

#### Recording File Check

```
GET /api/recordings/files/check
```

Checks if a recording file exists on disk.

#### Delete Recording File

```
DELETE /api/recordings/files
```

Deletes a recording file from disk.

#### Sync Recordings

```
POST /api/recordings/sync
```

Synchronizes the recordings database with files on disk.

### Timeline

#### Get Timeline Segments

```
GET /api/timeline/segments
```

Returns recording segments for the timeline view. Supports query parameters for stream name and date range.

#### Get Timeline Manifest

```
GET /api/timeline/manifest
```

Returns a manifest of available timeline data.

#### Timeline Playback

```
GET /api/timeline/play
```

Streams video for timeline playback at a specified point in time.

### System

#### Get System Information

```
GET /api/system
GET /api/system/info
```

Returns system information including version, uptime, CPU/memory/storage usage, and stream counts.

The response also includes a `versions.items` array summarizing runtime-detected software versions such as the base OS, LightNVR, optional services, and linked libraries.

#### Get System Status

```
GET /api/system/status
```

Returns system health status.

#### Get System Logs

```
GET /api/system/logs
```

Returns recent system log entries.

#### Clear System Logs

```
POST /api/system/logs/clear
```

Clears the system log file.

#### Restart System

```
POST /api/system/restart
```

Restarts the LightNVR service.

#### Shutdown System

```
POST /api/system/shutdown
```

Shuts down the LightNVR service.

#### System Backup

```
POST /api/system/backup
```

Creates a backup of the database.

#### Get Settings

```
GET /api/settings
```

Returns system configuration settings.

The storage fields include `mp4_directory_format`, one of `flat`,
`year_month`, or `year_month_day`.

#### Update Settings

```
POST /api/settings
```

Updates system configuration settings.

`mp4_directory_format` accepts only the three safe presets returned by the
GET endpoint; arbitrary `strftime` templates are rejected with HTTP 400.

### Health

#### Health Check

```
GET /api/health
```

Returns basic health status.

#### HLS Health Check

```
GET /api/health/hls
```

Returns HLS streaming subsystem health.

### ICE Servers

#### Get ICE Servers

```
GET /api/ice-servers
```

Returns WebRTC ICE server configuration (STUN/TURN servers).

### Authentication

#### Login

```
POST /api/auth/login
```

Authenticates a user and creates a session.

**Request Body:**
```json
{
  "username": "admin",
  "password": "yourpassword"
}
```

#### Login with TOTP

```
POST /api/auth/login/totp
```

Completes login with a TOTP code (for users with MFA enabled).

**Request Body:**
```json
{
  "token": "pending_session_token",
  "totp_code": "123456"
}
```

#### Logout

```
POST /api/auth/logout
GET /logout
```

Destroys the current session.

#### Verify Session

```
GET /api/auth/verify
```

Verifies that the current session is valid.

### User Management

#### List Users

```
GET /api/auth/users
```

Returns all users (admin only).

#### Create User

```
POST /api/auth/users
```

Creates a new user.

#### Get User

```
GET /api/auth/users/{id}
```

Returns a specific user.

#### Update User

```
PUT /api/auth/users/{id}
```

Updates a user.

#### Delete User

```
DELETE /api/auth/users/{id}
```

Deletes a user.

#### Generate API Key

```
POST /api/auth/users/{id}/api-key
```

Generates an API key for a user.

#### Change Password

```
PUT /api/auth/users/{id}/password
```

Changes a user's password.

#### Password Lock

```
PUT /api/auth/users/{id}/password-lock
```

Locks or unlocks a user's password from being changed.

### TOTP/MFA

#### Setup TOTP

```
POST /api/auth/users/{id}/totp/setup
```

Initiates TOTP setup, returns secret and QR code URI.

#### Verify TOTP

```
POST /api/auth/users/{id}/totp/verify
```

Verifies a TOTP code during setup to confirm it works.

#### Disable TOTP

```
POST /api/auth/users/{id}/totp/disable
```

Disables TOTP for a user.

#### TOTP Status

```
GET /api/auth/users/{id}/totp/status
```

Returns whether TOTP is enabled for a user.

### ONVIF Discovery

#### Discovery Status

```
GET /api/onvif/discovery/status
```

Returns ONVIF discovery service status.

#### Discover Devices

```
POST /api/onvif/discovery/discover
```

Triggers an ONVIF device discovery scan.

#### List Discovered Devices

```
GET /api/onvif/devices
```

Returns discovered ONVIF devices.

#### Get Device Profiles

```
GET /api/onvif/device/profiles
```

Returns media profiles for an ONVIF device.

#### Add Device as Stream

```
POST /api/onvif/device/add
```

Adds a discovered ONVIF device as a stream.

#### Test ONVIF Connection

```
POST /api/onvif/device/test
```

Tests connectivity to an ONVIF device.

### Detection

#### Get Detection Results

```
GET /api/detection/results/{stream_name}
```

Returns recent detection results for a stream.

Each result includes `timestamp` and `end_timestamp`. They are equal for
instantaneous model detections; external motion events use the persisted
start/stop interval.

#### List Detection Models

```
GET /api/detection/models
```

Returns available detection models.

### Motion Recording

#### Test Motion Event

```
POST /api/motion/test/{stream_name}
```

Triggers a test motion event for debugging.

### HLS Streaming

#### Direct HLS Request

```
GET /hls/{stream_name}/{filename}
```

Serves HLS playlist (.m3u8) and segment (.ts) files for live streaming.

## Error Handling

All API endpoints return appropriate HTTP status codes:

- 200: Success
- 400: Bad Request
- 401: Unauthorized
- 404: Not Found
- 500: Internal Server Error

Error responses include a JSON object with an error message:

```json
{
  "error": "Stream not found"
}
```

## Examples

### Curl Examples

Login and list streams:
```bash
# Login
curl -c cookies.txt -X POST -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"yourpassword"}' \
  http://your-lightnvr-ip:8080/api/auth/login

# List streams
curl -b cookies.txt http://your-lightnvr-ip:8080/api/streams

# Get system information
curl -b cookies.txt http://your-lightnvr-ip:8080/api/system

# Add a new stream
curl -b cookies.txt -X POST -H "Content-Type: application/json" \
  -d '{"name":"New Camera","url":"rtsp://192.168.1.103:554/stream1","enabled":true,"width":1280,"height":720,"fps":10,"codec":"h264","priority":5,"record":true}' \
  http://your-lightnvr-ip:8080/api/streams

# Trigger ONVIF discovery
curl -b cookies.txt -X POST http://your-lightnvr-ip:8080/api/onvif/discovery/discover
```
