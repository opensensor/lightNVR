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

### 2. Legacy API key (compatibility automation)

Every user has an `api_key` field. Pass it via either header:

```bash
# Preferred
curl -H "X-API-Key: <your-api-key>" http://your-lightnvr-ip:8080/api/streams

# Also accepted
curl -H "Authorization: Bearer <your-api-key>" http://your-lightnvr-ip:8080/api/streams
```

For automation (Home Assistant, NodeRED, cron jobs, etc.), create a dedicated
user with the `USER_ROLE_API` role and use that user's `api_key`. Keep admin
keys out of automation configs. New integrations should use the expiring,
action- and camera-scoped tokens described below. The single legacy key remains
available during the endpoint-enforcement migration.

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

### Action-level authorization foundation

New installations and upgrades include the Fleet 02 action catalog, reusable
roles, and camera-selector or shared-collection grants. Existing users remain in `legacy` authorization
mode until an administrator creates and previews equivalent grants, so upgrading
does not silently remove access. Users switched to `policy` mode are default-deny:
an action is allowed only when an enabled role grant contains the action and its
all-fleet, shared-collection, or camera-selector scope matches.

Existing handlers are being migrated to the central evaluator incrementally. The
current coverage and intended action for every route family are tracked in
[`docs/internal/AUTHORIZATION_ENDPOINT_INVENTORY.md`](internal/AUTHORIZATION_ENDPOINT_INVENTORY.md).

#### List authorization actions

```
GET /api/authorization/actions
```

Administrator-only. Returns the stable action key, category, description,
whether a camera resource is required, and whether the action is destructive.

Each entry also reports:

- `enforced` — whether any request handler currently routes through the
  centralized evaluator for this action. Actions are grantable ahead of their
  enforcement work, so clients must label the unenforced ones rather than imply
  a boundary that is not applied yet. See
  `docs/internal/AUTHORIZATION_ENDPOINT_INVENTORY.md` for the coverage map.
- `mask_bit` — the bit position this action occupies in a persisted API token
  `action_mask`. The position is frozen once an action ships; the daemon refuses
  to start if the stored layout in `authz_actions` disagrees with the binary.

A policy manager may only author roles, grants, and tokens whose actions are a
subset of the authority it holds itself. Requests that would widen the
requester's own authority are rejected with `403` and name the offending
actions, so `users.manage` cannot be used as a path to `system.admin`.

#### Simulate authorization

```
POST /api/authorization/simulate
```

Administrator-only and side-effect free. Camera-scoped actions require a stable
camera UUID:

```json
{
  "user_id": 7,
  "action": "recordings.export",
  "camera_uuid": "0192a7f0-4f43-4a1d-9e1c-d6947677f145"
}
```

The response reports `allowed`, the compatibility role or matching policy grant,
the evaluated policy version, and a concise explanation. Global actions such as
`users.manage` omit `camera_uuid`.

#### Manage authorization roles

```
GET    /api/authorization/roles
POST   /api/authorization/roles
PUT    /api/authorization/roles/{role_uuid}
DELETE /api/authorization/roles/{role_uuid}
```

Administrator-only. The list response includes the current `policy_version` and
each role's action keys. Built-in roles are readable but immutable. Create,
update, and delete requests must include the last observed version as
`expected_policy_version`; stale writes return `409` so concurrent policy edits
cannot silently overwrite one another.

Create and update bodies use a complete role representation:

```json
{
  "expected_policy_version": 12,
  "name": "Evidence reviewer",
  "description": "Can replay evidence without exporting it",
  "actions": ["live.view", "recordings.replay"]
}
```

A delete body contains only `expected_policy_version`. A custom role cannot be
deleted while a grant references it.

#### Manage a user's authorization policy

```
GET /api/authorization/users/{user_id}
PUT /api/authorization/users/{user_id}
```

Administrator-only. `GET` returns the user's mode, complete grants, and a policy
version. `PUT` atomically replaces the complete grant set and mode, and requires
that version as `expected_policy_version`. An `all` scope omits a resource; a
selector scope embeds a Fleet 01 selector:

```json
{
  "expected_policy_version": 13,
  "mode": "policy",
  "grants": [
    {
      "role_uuid": "00000000-0000-4000-8000-000000000003",
      "scope": {
        "type": "selector",
        "selector": {
          "version": 1,
          "expression": {
            "op": "tag_any",
            "uuids": ["c401035a-a208-4af9-9bf5-e49da3bd4200"]
          }
        }
      }
    }
  ]
}
```

A collection scope stores a durable reference instead of copying the
collection's current selector or members:

```json
{
  "role_uuid": "00000000-0000-4000-8000-000000000003",
  "scope": {
    "type": "collection",
    "collection_uuid": "d813e24e-0c7a-48e7-960c-4f5b843466db"
  }
}
```

Only shared collections may be authorization scopes. Their current static or
smart membership is evaluated at request time, so organizational changes take
effect without rewriting every user policy. An in-use collection cannot be
made private or deleted, and membership/rule changes advance the policy version.

The server validates every selector, collection, and role before changing
anything. It also
rejects an authenticated administrator's attempt to remove their own effective
`users.manage` grant. Saving grants in `legacy` mode is supported so an
administrator can prepare policy before activating default-deny evaluation.

#### Manage scoped API tokens

```
GET    /api/authorization/users/{user_id}/tokens
POST   /api/authorization/users/{user_id}/tokens
DELETE /api/authorization/users/{user_id}/tokens/{token_uuid}
```

A user may manage their own tokens; a principal with `users.manage` may manage
another user's. Token management itself requires a session, Basic auth, or a
legacy API key—a scoped token cannot mint another token. `POST` requires a
description, an explicit expiry no more than 366 days away, one or more action
keys, and an all-fleet, selector, or shared-collection scope:

```json
{
  "description": "North garage PTZ bridge",
  "expires_at": 1819075200,
  "actions": ["live.view", "ptz.control"],
  "scope": {
    "type": "collection",
    "collection_uuid": "d813e24e-0c7a-48e7-960c-4f5b843466db"
  }
}
```

The `201` response contains the secret once as `secret` plus non-secret token
metadata. lightNVR stores only its SHA-256 hash and a short display prefix.
`GET` returns metadata, expiry, revocation, last-use time, actions, and scope but
never the secret or hash. `DELETE` revokes rather than erases the token.

Token authorization is the intersection of the token and its owning user's
current effective access, so changing the user policy can only reduce what an
existing token can do. During the incremental enforcement rollout, scoped
tokens are accepted only by handlers that immediately invoke the central action
evaluator (currently scoped PTZ, recording export, evidence protection, and
deletion paths). Other legacy handlers reject them rather than risk ignoring a
camera scope. The endpoint inventory tracks expansion of that safe surface.

Administrators can manage these credentials from **Users → Manage API access**.
The dialog exposes only actions with current scoped-token endpoint enforcement,
supports shared collections and custom selectors, and requires acknowledgment
before dismissing a newly displayed secret. The non-expiring legacy key remains
in a separate compatibility-only section.

### Audit history and correlation IDs

Every initialized HTTP request receives a correlation ID. A caller may supply
`X-Request-ID` using up to 64 letters, digits, dots, underscores, colons, or
hyphens; otherwise lightNVR generates a UUID. Normal API responses echo the ID
as `X-Request-ID`, and audit records retain it so an operator can correlate a UI
failure, reverse-proxy log, and durable security decision.

Audit records are append-only through supported APIs. Structured details are
generated by the server, and sensitive field names such as passwords, secrets,
credentials, authorization headers, cookies, API keys, and raw tokens are
redacted before persistence. The initial event coverage includes login outcomes,
central authorization decisions, policy simulation and mutations, scoped-token
creation/revocation, and retention changes.

#### List audit events

```
GET /api/audit/events
```

Requires `system.admin`. Results are ordered newest first and accept these
optional query parameters:

| Parameter | Meaning |
| --- | --- |
| `page`, `page_size` | 1-based page; page size defaults to 100 and is capped at 1000 |
| `since`, `until` | Inclusive Unix timestamp bounds |
| `principal_user_id` | Exact local user ID |
| `action`, `outcome` | Exact action and outcome (`allowed`, `denied`, `success`, `failure`, or `error`) |
| `target_uuid`, `request_id` | Exact target or correlation ID |

The response contains `page`, `page_size`, page `count`, complete filtered
`total`, and an `events` array. Principal name and authentication method are
snapshotted so the history remains understandable after a user changes.

#### Export audit events

```
GET /api/audit/events/export
```

Accepts the same filters and pagination contract and returns CSV. The filtered
total is exposed as `X-Total-Count`. CSV cells are quoted and spreadsheet
formula prefixes are neutralized.

#### Manage audit retention

```
GET /api/audit/settings
PUT /api/audit/settings
```

Requires `system.admin`. `PUT` accepts an integer retention period from 1 to
3650 days and prunes already-expired records immediately:

```json
{
  "retention_days": 365
}
```

The default is 365 days. Routine audit writes perform an at-most-hourly expiry
check, so retention does not depend on a separate scheduler.

Administrators can browse this history from **Users → Audit History**. The
responsive workspace keeps filters server-side, shows structured details on
demand, exports the current filtered page, and manages retention without adding
another top-level navigation destination.

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

Returns a list of recordings. Supports query parameters for filtering by stream
name, date range, and pagination. Pass `collection_uuid` to filter recordings
by an authorized static or smart camera collection. `collection_uuid` and
`stream` are mutually exclusive; collection rules are evaluated on the server
so shared smart rules remain private and large collections do not expand into
query strings.

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

### Investigation

#### Multi-camera timeline

```
POST /api/investigations/timeline
```

Returns aligned recording tracks for up to 16 authorized camera UUIDs in a UTC
window. Each track includes recording intervals, capture methods, media
availability, and explicit gaps used by the synchronized investigation player.

#### Event and metadata search

```
POST /api/investigations/search
```

Searches persisted detection metadata with stable cursor pagination. Camera
scope is either `camera_uuids` or a Fleet selector, never both. Explicit camera
lists fail if any requested camera is unauthorized; broad selectors omit
unauthorized matches before totals, facets, and histograms are calculated.

```json
{
  "camera_uuids": ["0192a7f0-4f43-4a1d-9e1c-d6947677f145"],
  "start_time": 1787529600,
  "end_time": 1787533200,
  "filters": {
    "event_types": ["detection"],
    "labels": ["person"],
    "zones": ["loading-area"],
    "sources": ["local"],
    "capture_methods": ["continuous"],
    "recording_tags": ["reviewed"],
    "locations": ["03852a50-1254-4a0f-894c-cbc660fa6726"],
    "protected": true,
    "min_confidence": 0.75,
    "max_confidence": 1.0
  },
  "limit": 100,
  "cursor": null
}
```

The optional top-level `region` performs a metadata-only rectangular search on
one camera. Coordinates are normalized to the source image. Matching modes are
`center`, `intersects`, and `minimum_intersection`; the latter accepts a
`min_intersection` fraction greater than zero and at most one.

```json
{
  "region": {
    "camera_uuid": "0192a7f0-4f43-4a1d-9e1c-d6947677f145",
    "x": 0.1,
    "y": 0.2,
    "width": 0.4,
    "height": 0.5,
    "match": "minimum_intersection",
    "min_intersection": 0.25
  }
}
```

Region search never decodes historical video. The response
`coverage.spatial_metadata` reports rows with and without valid normalized
bounding boxes. Rows without boxes are not searched spatially, and
`spatial_metadata_missing` appears in `incomplete_reasons` so an empty result is
not presented as proof that nothing crossed the selected area.

### Fleet Query and Selectors

#### Query Cameras

```
POST /api/fleet/cameras/query
```

Returns an authorized, server-paginated camera inventory with optional facets.
The `address` field contains only the source scheme and network authority; paths,
query strings, fragments, and embedded credentials are omitted. Existing
`allowed_tags` restrictions are applied before totals and facet counts are
calculated.

```json
{
  "selector": {
    "version": 1,
    "expression": {
      "op": "and",
      "children": [
        {"op": "location_subtree", "uuid": "location-uuid"},
        {"op": "tag_any", "uuids": ["tag-uuid"]},
        {"op": "health", "values": ["down", "degraded"]}
      ]
    }
  },
  "search": "north door",
  "collection_uuid": "optional-collection-uuid",
  "page": 1,
  "page_size": 50,
  "sort_by": "name",
  "sort_order": "asc",
  "facets": true,
  "explain": false
}
```

`page_size` is limited to 200. Supported sort fields are `name`,
`camera_uuid`, `location`, `health`, `enabled`, `recording_mode`, and
`address`. Results use the selected field plus camera UUID as a stable
tie-breaker.

`collection_uuid` is optional and composes with `selector` and `search`. The
collection must be shared, owned by the caller, or requested by an
administrator. Collection membership and the caller's `allowed_tags` scope are
applied before totals, pages, and facets are calculated. A collection that is
not visible to the caller returns `404`.

Selector version 1 supports:

- Boolean nodes: `and` with `children`, `or` with `children`, and `not` with
  `child`.
- `all`.
- `camera_uuid` with `values`.
- `location_subtree` with `uuid`.
- `tag_any`, `tag_all`, and `tag_none` with tag `uuids`.
- `enabled` with a boolean `value`.
- `recording_mode` with `values` from `off`, `continuous`, and `detection`.
- `vendor` and `model` with case-insensitive `values`. Inventory values are
  populated as ONVIF inventory support becomes available.
- `capability_any` and `capability_all` with `values` from `onvif`, `ptz`, and
  `backchannel`.
- `health` with `values` from `unknown`, `up`, `degraded`, `down`, and
  `disabled`.

Selectors are limited to 8 levels, 64 nodes, and 64 values per node.

#### Preview Selector

```
POST /api/fleet/selectors/preview
```

Accepts the same request as the query endpoint, caps pages at 50 cameras, and
adds `matched_clauses` to each returned camera. An optional `camera_uuid`
restricts the preview to one camera.

### Camera Collections

Collections are durable named camera groups. A `static` collection stores UUID
membership; a `smart` collection stores a selector v1 object and updates as
cameras, locations, tags, configuration, or health change.

#### List and Create Collections

```
GET /api/camera-collections
POST /api/camera-collections
```

Listing requires viewer access and returns only shared collections, collections
owned by the caller, or all collections for administrators. Counts are computed
after current tag RBAC. Smart selector definitions are returned only to an
administrator or the collection owner; other viewers receive `selector: null`
and `selector_redacted: true`. Creation is administrator-only.

```json
{
  "name": "Offline entrances",
  "description": "Entrance cameras requiring attention",
  "type": "smart",
  "shared": true,
  "selector": {
    "version": 1,
    "expression": {
      "op": "and",
      "children": [
        {"op": "tag_any", "uuids": ["entrance-tag-uuid"]},
        {"op": "health", "values": ["down"]}
      ]
    }
  }
}
```

#### Read, Update, and Delete a Collection

```
GET /api/camera-collections/{collection_uuid}
PUT /api/camera-collections/{collection_uuid}
DELETE /api/camera-collections/{collection_uuid}
```

Reads follow collection visibility and camera RBAC. Update and delete are
administrator-only in this initial phase. Switching a collection to `smart`
atomically removes obsolete static membership.

#### Static Collection Members

```
GET /api/camera-collections/{collection_uuid}/members
PUT /api/camera-collections/{collection_uuid}/members
```

`PUT` replaces membership atomically with a `camera_uuids` array and is limited
to 4,096 entries. `GET` omits cameras outside the caller's current scope. Smart
collections reject explicit member operations.

#### Preview a Collection

```
POST /api/camera-collections/{collection_uuid}/preview
```

Returns the authorized `matched_count` and a sample of at most 50 camera UUIDs,
names, and location paths.

### Event Routes

Event route endpoints require the global `events.configure` action (legacy
administrators have it). They expose the registered event catalog and a durable,
revisioned route control plane. The normalized MQTT publisher evaluates enabled
routes before durable enqueue; the preview endpoint never publishes.

#### Event Catalog

```
GET /api/events/catalog
```

Returns every registered event type with its family, description, severity,
sensitivity, media policy, expected rate, subject kind, and default expiry.

#### MQTT Destination Profiles

```
GET    /api/event-destinations
POST   /api/event-destinations
GET    /api/event-destinations/{destination_uuid}
PUT    /api/event-destinations/{destination_uuid}
DELETE /api/event-destinations/{destination_uuid}?revision={last_seen_revision}
```

These endpoints manage up to 64 named MQTT broker profiles. All operations
require `events.configure`. The list response also describes the unmanaged
`mqtt:default` destination backed by the existing `[mqtt]` settings.

Create requires `name` and `broker.host`. It defaults to port `8883`, system
certificate trust, QoS 1, a 60-second keepalive, a unique `lightnvr-…` client
ID, and topic template `lightnvr/v1/events/{type}/{subject_id}`.

```json
{
  "name": "Operations bridge",
  "description": "Input for the hosted notification service",
  "enabled": true,
  "type": "mqtt",
  "broker": {
    "host": "mqtt.example.net",
    "port": 8883,
    "client_id": "lightnvr-campus-a",
    "topic_template": "campus-a/{type}/{subject_id}",
    "keepalive_seconds": 60,
    "qos": 1
  },
  "authentication": {
    "username": "event-publisher",
    "password": "write-only-secret"
  },
  "tls": {
    "mode": "system"
  }
}
```

`tls.mode` is one of `disabled`, `system`, `custom_ca`, or `mutual`. Custom
certificate paths must be absolute; `custom_ca` requires `ca_file`, and
`mutual` requires `ca_file`, `cert_file`, and `key_file`.
Topic templates must contain `{type}` and `{subject_id}` and cannot contain
MQTT wildcards.

Passwords are write-only. Responses contain only
`authentication.password_configured`; they never return the credential. On
update, omitting `authentication.password` preserves it, while JSON `null` or
an empty string clears it. Updates are partial but require the last observed
positive `revision`, and deletes use the same revision as a query parameter.
Names are unique case-insensitively, as is the broker host, port, and client ID
combination. A profile referenced by an event route or an active durable outbox
row cannot be deleted. Delivered and dead history does not block deletion.
Profile create, update, and delete outcomes are written to the audit history
without credential material.

Each enabled profile has an independent reconnecting MQTT client. Routes may
use `mqtt:default` or the `mqtt:<destination_uuid>` key returned by these
endpoints. Disabling a profile pauses delivery for its durable queue without
discarding unexpired events; re-enabling or updating it rebuilds the client
from the latest revision. The password remains write-only during that reload.

#### List, Create, and Read Routes

```
GET  /api/event-routes
POST /api/event-routes
GET  /api/event-routes/{route_uuid}
```

Create accepts a complete route definition. Only `name` and `event_types` are
required; omitted fields use the defaults shown below. Unknown fields and
unknown event types are rejected. `destination` must be `mqtt:default` or the
key of an existing managed MQTT destination profile.

```json
{
  "name": "North entrance people",
  "description": "External notification input",
  "enabled": true,
  "destination": "mqtt:default",
  "event_types": ["io.lightnvr.detection.object.v1"],
  "camera_scope": {
    "type": "selector",
    "selector": {
      "version": 1,
      "expression": {
        "op": "location_prefix",
        "values": ["Campus/North"]
      }
    }
  },
  "predicate": {
    "version": 1,
    "detection": {
      "labels_any": ["person"],
      "min_confidence": 0.8,
      "zone_ids_any": ["entry"]
    }
  },
  "schedule": {
    "version": 1,
    "timezone": "America/New_York",
    "windows": [
      {"days": [1, 2, 3, 4, 5], "start": "18:00", "end": "06:00"}
    ]
  },
  "suppression": {
    "debounce_seconds": 2,
    "cooldown_seconds": 30,
    "grouping_window_seconds": 10,
    "max_events_per_minute": 20
  }
}
```

An all-camera scope is `{"type":"all"}`. Defaults are enabled, all cameras,
`{"version":1}` predicate, an always-active UTC schedule, and zero for each
suppression value. A successful create returns `201` with the server-assigned
UUID, revision `1`, and timestamps. Names are unique case-insensitively and at
most 512 routes may be stored.

Timezone names must resolve under `/usr/share/zoneinfo` (with `UTC` and `GMT`
always accepted). Schedules are evaluated against event occurrence time and
support DST-aware overnight windows.

Suppression is durable and isolated by route UUID, event type, and subject:

- `debounce_seconds` suppresses a repeat inside the interval since the latest
  observation; each suppressed repeat extends the interval.
- `cooldown_seconds` starts when an event is accepted by the outbox; suppressed
  repeats do not extend it.
- `grouping_window_seconds` preserves the first event and coalesces repeats for
  the window. Version 1 does not emit an aggregate summary event.
- `max_events_per_minute` limits allowed events in a fixed 60-second window.

Checks run in that order. An allowed event advances suppression state only after
durable outbox acceptance (`ENQUEUED` or idempotent `DUPLICATE`), so queue-full
and persistence errors do not consume cooldown or rate budget. Editing a route
clears its prior suppression state, and inactive state is pruned after 30 days.

One normalized envelope is persisted for each unique destination matched by at
least one route. Multiple matching routes to the same destination do not create
duplicate publishes. The destination's topic template is expanded and frozen
when the outbox row is created, so later profile edits affect new events without
changing already accepted work. Fan-out is independent: acceptance or failure
for one destination does not consume another destination's suppression state.

With zero stored routes, normalized MQTT retains its compatibility publish-all
behavior through `mqtt:default` when the legacy MQTT setting is enabled. Once
any route is stored, only events matching at least one enabled route are
enqueued to that route's destination. Disabling all stored routes pauses
normalized enqueue; deleting the last route restores the default. Managed
destinations continue to run when the legacy/default MQTT setting is disabled.
This does not filter legacy detection or Home Assistant compatibility topics.

#### Update and Delete a Route

```
PUT    /api/event-routes/{route_uuid}
DELETE /api/event-routes/{route_uuid}?revision={last_seen_revision}
```

Update is a partial write but must include the last observed positive
`revision`. Delete carries the same value as a query parameter. A stale write
returns `409`; a successful update increments the revision. Create, update, and
delete outcomes are recorded in the audit history. Any successful update resets
the route's durable suppression history so the revised policy starts cleanly.

#### Preview a Route Draft

```
POST /api/event-routes/preview
```

Accepts the same complete body as create, validates every field, and resolves
the camera selector against the current Fleet inventory. The response includes
`matched_camera_count`, a `camera_sample` of at most 20 entries, registry
metadata for the selected event types, and `would_publish: false`. It neither
persists the draft nor enqueues or publishes an event.

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
  "totp_token": "pending_session_token",
  "code": "123456"
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
