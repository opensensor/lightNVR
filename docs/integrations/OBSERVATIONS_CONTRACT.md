# Observations, Evidence and Classification Contract (INT_01 ADR)

**Status**: Frozen for P1/P2 implementation (task C1). Budgets in §13 are
provisional until task X2 confirms them.
**Contract version**: `observations` 1, `classifications` 1, event
`io.lightnvr.detection.observation_changed.v1`.
**Source PRD**: [INT_01 — Event Evidence and YA-WAMF Integration](../prd/INT_01_Event_Enrichment_YAWAMF.md)
**Plan**: [INT_01 swarm plan](../../int01-event-enrichment-yawamf-plan.md)
**Baselines**: lightNVR `0ff0d2e3` (version 0.47.3); YA-WAMF `53540f48` (v2.23.1).

This document is normative. "MUST", "MUST NOT", "SHOULD" and "MAY" have their
RFC 2119 meanings. Every L\* (lightNVR) and W\* (YA-WAMF) task implements this
contract. Where a task needs a field, status, code or limit not listed here, the
contract is amended first; implementations do not invent one.

The canonical machine-readable schemas are added to
[`docs/api/openapi.json`](../api/openapi.json) by task C2, and canonical JSON
examples are added under `tests/fixtures/integrations/observations/` by task
C3. If this document and OpenAPI disagree, fix the discrepancy before merging
either.

## Contents

1. [Decisions and deviations](#1-decisions-and-deviations)
2. [Conventions](#2-conventions)
3. [Observation object](#3-observation-object)
4. [Evidence asset](#4-evidence-asset)
5. [Change log, revisions and publication](#5-change-log-revisions-and-publication)
6. [Change event](#6-change-event-iolightnvrdetectionobservation_changedv1)
7. [History and cursor](#7-history-and-cursor-get-apiobservations)
8. [Single observation and snapshot](#8-single-observation-and-snapshot)
9. [Capability document](#9-capability-document-get-apiintegrationscapabilities)
10. [Classification annotations (P2)](#10-classification-annotations-p2)
11. [Error catalog](#11-error-catalog)
12. [Authorization and audit](#12-authorization-and-audit)
13. [Budgets and configuration (provisional)](#13-budgets-and-configuration-provisional)
14. [Retention and storage](#14-retention-and-storage)
15. [Diagnostics](#15-diagnostics)
16. [YA-WAMF consumer rules](#16-ya-wamf-consumer-rules)
17. [Reserved identifiers](#17-reserved-identifiers)
18. [Supported-version matrix (stub)](#18-supported-version-matrix-stub)
19. [Performance](#19-performance)
20. [PRD traceability](#20-prd-traceability)

## 1. Decisions and deviations

Plan design decisions 1–9 are adopted. This section records where the contract
refines or deliberately deviates from them, with the reason.

| # | Topic | Contract | Plan text | Reason |
| --- | --- | --- | --- | --- |
| D1 | Multi-value query filters | `camera_uuid` and `label` are **comma-separated** single parameters (`camera_uuid=a,b`), max 10 cameras and 8 labels | `camera_uuid[]`, `label[]` | `http_request_get_query_param()` returns only the first occurrence of a key, and `http_request_t.query_string` is 1,024 bytes (`include/web/request_response.h:50`). Comma lists fit; repeated keys would silently drop values. |
| D2 | Error body shape | Every coded error is `{"error": "<message>", "code": "<code>", ...}` | `{code, oldest_cursor, oldest_observed_at}` | Keeps every response a valid existing `ApiError` schema (`error` is required there). |
| D3 | Writer namespace | Derived from the **authenticated user's username** (normalized), and authority is pinned to the **user id** stored on the row; the capability document advertises it | "derived from the API token's identity" | API tokens have only a UUID and free-text description (`api_token_t`). A token-UUID namespace would orphan annotations on token rotation. Usernames are unique and user ids are never reused (`AUTOINCREMENT`, migration 0006). W10 MUST use the advertised namespace rather than hard-coding `yawamf`; the recipe (L12) creates a dedicated user named `yawamf` so the namespace is `yawamf`. |
| D4 | Snapshot denial when metadata is visible | `404` when the caller cannot read the observation at all; `403 forbidden` when the caller holds `live.view` but not `snapshot.create` on the capture camera | "`404` for unknown or unauthorized" | The observation's existence is already visible to that caller via metadata, so a `403` leaks nothing and is more actionable. Callers without any read authority still get an indistinguishable `404`. |
| D5 | Every observation has an asset row | `evidence.asset_id` is never null; observations without media reference an asset in state `unavailable` with no file | L1: `asset_uuid` nullable | One evidence shape for consumers; reasons live on the asset row. L1 MAY keep the column nullable, but L2 always sets it. |
| D6 | Change rows snapshot evidence state | `observation_changes` additionally stores `evidence_state` and `evidence_reason` at that revision | not listed in L1 | The relay must publish the state *at that revision*, not the current state, so a retried message keeps the same meaning. |
| D7 | Publication states | `pending`, `relayed`, `relayed_no_route`, `expired`, `failed` | L1: `pending\|relayed\|failed`; L5 adds `relayed_no_route` | `expired` covers rows not relayed before the event's delivery expiry (cursor recovery applies). Listed once so L1's CHECK matches L5. |
| D8 | Outbox share cap | The relay stops enqueuing while this type holds ≥ 2,000 pending outbox rows | not specified | The default outbox holds 10,000 rows, and `info` events cannot evict others. Without a cap, a broker outage lets observation changes crowd out `camera.offline`-class warnings. |
| D9 | DB-restore epoch detection | Epoch is regenerated by in-process restore paths **and** at startup when a sidecar head marker shows the DB seq rewound (§7.6) | "regenerated on restore or import" | `restore_database_from_backup()` (called from `db_core.c` corruption recovery) and manual file restores both copy the old `observation_meta.epoch_uuid`. A sidecar outside the DB detects a manual restore. |
| D10 | Head cursor | `cursor=head` returns no items and a cursor at the current watermark | not specified | Lets a consumer start live-only without paging all retained history. Optional for consumers. |
| D11 | Notification cooldown name | YA-WAMF's existing bound is `settings.notifications.notification_cooldown_minutes` | "existing visit cooldown" | Name of the actual setting (`notification_service.py:108`). |

## 2. Conventions

- **Source identity.** `source` is the installation URN
  `urn:lightnvr:<installation_uuid>`, from `event_identity_get_source()`
  (`system_settings.event_installation_uuid`). It is the same value as the
  envelope `source` in [EVENT_CONTRACT.md](../EVENT_CONTRACT.md).
- **UUIDs** are lowercase canonical 36-character strings. Observation, asset
  and batch IDs are random UUIDv4. Message IDs are UUIDv5 (§5.4).
- **Times** in API bodies are UTC RFC 3339 with millisecond precision and a
  `Z` suffix, for example `2026-11-08T14:03:12.481Z`. The DB stores integer
  milliseconds (`*_ms`). Envelope `time` keeps the existing second-precision
  format.
- **Camera identity** is always the immutable capture-time `camera_uuid`
  (Fleet 01). Stream names are display metadata only and are never used for
  identity or authorization.
- **No filesystem paths** appear in any API body or event. Media is referenced
  by authenticated logical URLs whose JSON key ends in `_url`. This matches the
  envelope validator, which rejects keys named `path` or containing `_path` or
  `token`, and rejects `/`-prefixed values unless the key ends in `_url`
  (`src/core/event_envelope.c:232-306`).
- **Unknown fields.** Response readers MUST ignore unknown fields. Request
  bodies for PUT (§10) are closed: unknown top-level fields return `400`.

## 3. Observation object

One observation is one detected object in one accepted inference result. A
detector result batch with N surviving objects (after zone, object-filter and
threshold filtering) yields N observations sharing one evidence asset.
Independently sampled frames are distinct observations, even of the same bird.
Motion-only engines (`engine_type = motion`) never create observations.

### 3.1 Fields

| Field | Type | Null | Bounds and rules |
| --- | --- | --- | --- |
| `id` | uuid | no | Immutable observation ID (`object_observations.uuid`). |
| `source` | string | no | Installation URN (§2). Immutable. |
| `camera_uuid` | uuid | no | Capture-time camera identity, resolved from the stream row at insert. Never reassigned by rename, delete or name reuse. |
| `stream_name_at_capture` | string | no | Display name when observed (≤ 63 chars). Display only. |
| `batch_id` | uuid | no | Accepted inference batch. `UNIQUE(batch_id, object_index)` makes a batch retry a no-op. |
| `object_index` | integer | no | 0..19. Position in the filtered batch. |
| `observed_at` | time | no | Frame capture or inference time (camera-side). |
| `created_at` | time | no | Persistence time of revision 1. Distinct from `observed_at`. |
| `detector` | object | no | §3.3. |
| `label` | string | no | Original detector label, 1..31 bytes (`MAX_LABEL_LENGTH` 32 including NUL, `include/video/detection_result.h`). Never modified by classification. |
| `confidence` | number | no | Original detector confidence, 0.0..1.0 inclusive. |
| `box` | object | yes | §3.2. Null when the detector reports a class without spatial data (ONVIF, some external engines). |
| `evidence` | object | no | §4. |
| `track` | object | yes | §3.4. Null when the detector has no track. |
| `recording_reference` | object | yes | §3.5. |
| `revision` | integer | no | ≥ 1. §5.2. |
| `classifications` | object | no | `{ "automated": [], "owner_confirmed": [] }`, §10. Always present; both lists are empty in P1. |

A batch holds at most 20 objects (`MAX_DETECTIONS`). An object over that limit
is never stored.

### 3.2 `box` and coordinate space

```json
{ "format": "xywh_normalized", "x": 0.412, "y": 0.288, "width": 0.120, "height": 0.165 }
```

- `format` is always `xywh_normalized` in version 1. `x`, `y` are the top-left
  corner.
- Coordinates are normalized to the **stored evidence image**
  (`evidence.width` × `evidence.height`), not to the main stream, a recording,
  or the model tensor. lightNVR applies the transform before storing the box.
  A consumer crops with `px = x * evidence.width` (and so on) and never
  re-applies `transform`.
- Values are clamped to [0, 1] and rounded to 6 decimals. `width` and
  `height` are > 0. `x + width` ≤ 1 and `y + height` ≤ 1 after clamping.

### 3.3 `detector`

| Field | Type | Null | Rule |
| --- | --- | --- | --- |
| `engine_key` | string | no | `stream_detection_engines.engine_key` (migration 0077), for example `legacy-primary`. ≤ 64 chars. |
| `engine_type` | enum | no | `object`, `api`, `onvif` or `external` (the 0077 CHECK set minus `motion`). |
| `model` | string | yes | Model **basename** only, for example `ssd_mobilenet_v2.sod`. Never a path. ≤ 128 chars. |
| `model_version` | string | yes | Version or hash when the engine reports one. ≤ 64 chars. |

### 3.4 `track`

```json
{ "id": "17", "namespace": "legacy-primary", "session": "9b0c7d1e-5f43-4b8e-9d0e-0b6c1f2a3d4e" }
```

- `id` is the detector's track ID as a decimal string (1..64 chars). It is
  omitted when `track_id < 0`, in which case `track` is null.
- `namespace` is the `engine_key` that issued the ID.
- `session` is a UUIDv4 generated once per lightNVR process start (a boot
  session, not the host boot ID). A reused track number after restart
  therefore never joins two unrelated tracks. Consumers MUST compare all three
  fields together.
- A track is a hint only. It does not establish physical-object identity, and
  no lifecycle (`new/update/end`) is implied.

### 3.5 `recording_reference`

```json
{ "recording_id": 81234 }
```

This is an optional logical association with the recording open when observed
(`recording_id` FK `ON DELETE SET NULL`). Absence never blocks snapshot
classification, and recording deletion never deletes evidence (§14). P3 adds
offsets; version 1 has none.

### 3.6 Example (ready, single bird)

```json
{
  "id": "55555555-5555-4555-8555-555555555555",
  "source": "urn:lightnvr:11111111-1111-4111-8111-111111111111",
  "camera_uuid": "22222222-2222-4222-8222-222222222222",
  "stream_name_at_capture": "feeder-east",
  "batch_id": "77777777-7777-4777-8777-777777777777",
  "object_index": 0,
  "observed_at": "2026-11-08T14:03:12.481Z",
  "created_at": "2026-11-08T14:03:12.530Z",
  "detector": { "engine_key": "legacy-primary", "engine_type": "object",
                "model": "ssd_mobilenet_v2.sod", "model_version": null },
  "label": "bird",
  "confidence": 0.87,
  "box": { "format": "xywh_normalized", "x": 0.412, "y": 0.288, "width": 0.12, "height": 0.165 },
  "evidence": {
    "asset_id": "88888888-8888-4888-8888-888888888888",
    "state": "ready",
    "reason": null,
    "origin": "inference_input",
    "media_type": "image/jpeg",
    "width": 640, "height": 360, "bytes": 48211,
    "captured_at": "2026-11-08T14:03:12.481Z",
    "expires_at": "2026-11-15T14:03:12.481Z",
    "transform": { "mode": "none", "source_width": 640, "source_height": 360,
                   "inference_width": 640, "inference_height": 360,
                   "scale_x": 1.0, "scale_y": 1.0, "pad_x": 0, "pad_y": 0 },
    "snapshot_url": "/api/observations/55555555-5555-4555-8555-555555555555/snapshot"
  },
  "track": null,
  "recording_reference": { "recording_id": 81234 },
  "revision": 2,
  "classifications": { "automated": [], "owner_confirmed": [] }
}
```

## 4. Evidence asset

An evidence asset is the unannotated image that was the actual inference input
(local) or the exact submitted image (HTTP API). One asset MAY be shared by
every observation in its batch.

### 4.1 Fields (`evidence`)

| Field | Type | Null | Rule |
| --- | --- | --- | --- |
| `asset_id` | uuid | no | `observation_assets.uuid`. Shared by sibling observations (D5). |
| `state` | enum | no | `pending`, `ready`, `unavailable` or `expired`. |
| `reason` | enum | yes | Null for `pending` and `ready`; required for `unavailable` and `expired` (§4.3). |
| `origin` | enum | yes | `inference_input` (local decoded frame), `submitted_image` (HTTP detector request body, or the go2rtc snapshot fetched **as** the inference input), or null when there is no media. A low-resolution inference image MUST NOT be labelled as a recording frame. |
| `media_type` | string | yes | `image/jpeg` when media exists or existed. |
| `width`, `height` | integer | yes | Pixel size of the stored image. Set once `ready`, and kept after `expired`. |
| `bytes` | integer | yes | File size. Set once `ready`. |
| `captured_at` | time | yes | Capture time of the frame. Equals the observation's `observed_at` for local inference. |
| `expires_at` | time | yes | Current estimate of media expiry from the age budget (byte pressure can expire it earlier). Advertises actual remaining availability; it is not a guarantee. |
| `transform` | object | yes | §4.4. Null when there is no media. |
| `snapshot_url` | string | yes | `/api/observations/{id}/snapshot`. Present only when `state = ready`. Relative, authenticated, never a filesystem path. |

### 4.2 State machine

```text
            +--> ready --+--> expired      (retention_expired | deleted)
            |            +--> unavailable  (write_failed: file missing at startup)
pending ----+
            +--> unavailable               (encode_failed | write_failed | interrupted | disabled)

(created directly) unavailable             (queue_full | budget_exhausted | no_matching_media)
```

- Transitions are compare-and-set on the current state. Two racing
  transitions from the same state: exactly one wins.
- `unavailable` and `expired` are terminal. `pending` never moves back.
- `ready` is committed only after the file is fsynced, renamed from
  `*.jpg.tmp` to its final name, and the directory is fsynced. The `ready`
  transition, its change rows and publication rows are one SQLite transaction
  (§5).
- Expiry keeps the metadata (dimensions, transform, captured_at) and drops the
  media.

### 4.3 Reason enum (bounded)

| Reason | Valid with state | Meaning |
| --- | --- | --- |
| `queue_full` | unavailable | Admission dropped the frame: per-camera or global queue full. |
| `budget_exhausted` | unavailable | The byte budget is exhausted and only protected assets remain to evict. |
| `encode_failed` | unavailable | The JPEG encode failed. |
| `write_failed` | unavailable | A write, fsync or rename failed, or a `ready` file was missing at reconciliation. |
| `interrupted` | unavailable | Shutdown or crash before finalization; resolved by startup reconciliation. |
| `no_matching_media` | unavailable | The source (ONVIF, external events) cannot supply the matching frame. A separately timed context image is never substituted. |
| `disabled` | unavailable | Evidence was disabled for the camera between admission and encode. |
| `retention_expired` | expired | Age or byte retention evicted the media. |
| `deleted` | expired | Explicit evidence deletion by policy or an operator. |

Adding a reason is a minor contract change: consumers MUST treat an unknown
`unavailable` reason as `unavailable` and an unknown `expired` reason as
`expired`.

### 4.4 `transform`

| Field | Type | Meaning |
| --- | --- | --- |
| `mode` | enum | `none`, `resize` (independent x/y scale, no padding) or `letterbox` (uniform scale plus padding). |
| `source_width`, `source_height` | integer | Decoded camera frame the detector input was derived from (detection-stream resolution, not main stream). |
| `inference_width`, `inference_height` | integer | Image or tensor the model consumed. |
| `scale_x`, `scale_y` | number | `inference / stored` along each axis (> 0). |
| `pad_x`, `pad_y` | number | Inference-space padding in pixels on the left and top (≥ 0; 0 unless `letterbox`). |

The mapping from an inference-space pixel `(u, v)` to a stored-image pixel is
`x = (u - pad_x) / scale_x`, `y = (v - pad_y) / scale_y`. The transform is
recorded for audit. Boxes are already mapped (§3.2).

- Local SOD/TFLite: stored image = full decoded frame. `mode = none`, or
  `resize` when SOD pre-resizes (`src/video/sod_detection.c`).
- HTTP API: stored image = the exact submitted JPEG bytes. `mode` is `none`
  unless the response declares a server-side letterbox.

### 4.5 Storage layout

Files are stored at
`<storage_path>/observations/<camera_uuid>/<yyyy-mm-dd>/<asset_uuid>.jpg`,
with a `.jpg.tmp` suffix while being written. The layout is internal and never
appears in the API. Evidence does not use the legacy MQTT 100-per-camera
snapshot policy.

## 5. Change log, revisions and publication

### 5.1 Tables (implemented by L1/L2)

- `observation_changes(seq INTEGER PRIMARY KEY AUTOINCREMENT, observation_uuid,
  revision, cause, evidence_state, evidence_reason, created_at_ms,
  UNIQUE(observation_uuid, revision))`.
- `observation_publications(change_seq PK/FK, message_id, state, attempts,
  last_error, updated_at_ms)`. `state` is one of `pending`, `relayed`,
  `relayed_no_route`, `expired` or `failed` (D7).
- `observation_meta(epoch_uuid, oldest_seq)` holds a single row (§7.5).

### 5.2 Revision semantics

- Revision 1 is written at creation with cause `created`. Its
  `evidence_state` is `pending`, or `unavailable` for drops and
  `no_matching_media`.
- Each evidence state change adds +1 with cause `evidence`, applied to
  **every** observation referencing the asset.
- Each annotation create or update that changes stored data (not a no-op
  retry) adds +1 with cause `classification` (P2).
- Revisions are monotonic per observation, never reused, and never skipped.
- The observation row, its change row and its publication row are written in
  **one** transaction, using the `db_auth.c` locking pattern (plan decision 2).
  No mutex-taking `db_*` helper is called inside it.

### 5.3 `change_cause`

| Value | Consumer action |
| --- | --- |
| `created` | New observation. Classify only if `evidence_state = ready`. |
| `evidence` | Evidence changed. Classify when it becomes `ready`; mark media expired or unavailable otherwise. |
| `classification` | Annotation feedback. Consumers MUST NOT re-classify or re-notify. Display only. |

### 5.4 Message ID

`message_id = UUIDv5(NS, source + "|" + observation_id + "|" + revision)`, where
`revision` is the decimal string and NS is the frozen namespace
`0bc01005-a431-522b-9124-a114f67d187a`. NS itself is
`UUIDv5(NAMESPACE_URL, "https://lightnvr.io/ns/observation-change/v1")`.

Test vector: source `urn:lightnvr:11111111-1111-4111-8111-111111111111`,
observation `55555555-5555-4555-8555-555555555555`, revision `2` →
`43ebdf02-b2d5-5403-a0ef-2a4e7d58ada9`.

The ID is computed once at change insert, stored in
`observation_publications.message_id`, and passed to
`event_envelope_create_with_severity_and_id()`. Relay restarts and outbox
retries reuse it. lightNVR has no UUIDv5 helper today; L5 adds one in
`src/utils/uuid.c` using mbedTLS SHA-1, which `src/core/totp.c` already links.

### 5.5 Relay delivery

- The relay enqueues pending publications in `seq` order through
  `event_producer_enqueue_durable()`. L5 exports this wrapper around the
  static `durably_enqueue_event` (`src/core/event_producers.c:91`).
- `enqueued` and `duplicate` → `relayed`. `no_route` and `mqtt_disabled` →
  `relayed_no_route`. `full` or an error → stays `pending` with backoff.
- A pending row older than the type's delivery expiry (§6.1) → `expired`. An
  envelope validation failure → `failed`.
- In-process `event_bus_publish` runs after the outbox step.
- Delivery is **at least once**. Consumers dedupe on `source + message_id`.
  Outbox pruning means a late retry may redeliver.
- A suppressed route is not deletion or an authorization decision. The cursor
  API (§7) is the recovery path for every non-`relayed` row.
- Outbox share cap (D8): while ≥ 2,000 outbox rows of this type are pending,
  the relay pauses and leaves publications `pending`.

## 6. Change event `io.lightnvr.detection.observation_changed.v1`

### 6.1 Registry entry (added by L5 to `EVENT_TYPES[]` in `src/core/event_envelope.c`)

| Property | Value |
| --- | --- |
| `type` | `io.lightnvr.detection.observation_changed.v1` |
| `family` | `detection` |
| `description` | `A durable object observation was created or changed` |
| `severity` | `EVENT_SEVERITY_INFO` (static) |
| `sensitivity` | `EVENT_SENSITIVITY_OPERATIONAL` |
| `media_policy` | `EVENT_MEDIA_REFERENCE_ALLOWED` |
| `expected_rate` | `EVENT_RATE_HIGH` |
| `subject_kind` | `EVENT_SUBJECT_CAMERA`, subject `camera/<camera_uuid>` |
| `default_expiry_seconds` | `3600` (matches `detection.object.v1`; the cursor is the recovery path) |

The envelope `id` is the message ID (§5.4). `time` is the change row's
`created_at_ms` truncated to seconds, so a retry has an identical envelope.

### 6.2 MQTT topic

The topic uses the existing normalized form
`{topic_prefix}/v1/events/io.lightnvr.detection.observation_changed.v1/<camera_uuid>`
(`src/core/mqtt_client.c:902`). Retain is disabled. Existing
`detection.object.v1`, `{topic_prefix}/detections/{stream_name}`, snapshot and
Home Assistant topics are unchanged.

### 6.3 Data payload

```json
{
  "observation_id": "55555555-5555-4555-8555-555555555555",
  "revision": 2,
  "camera_uuid": "22222222-2222-4222-8222-222222222222",
  "label": "bird",
  "observed_at": "2026-11-08T14:03:12.481Z",
  "change_cause": "evidence",
  "evidence_state": "ready",
  "evidence_reason": null,
  "asset_id": "88888888-8888-4888-8888-888888888888",
  "snapshot_url": "/api/observations/55555555-5555-4555-8555-555555555555/snapshot"
}
```

| Field | Required | Rule |
| --- | --- | --- |
| `observation_id` | yes | UUID. |
| `revision` | yes | Integer ≥ 1. |
| `camera_uuid` | yes | UUID equal to the subject's UUID. |
| `label` | yes | 1..31 bytes. |
| `observed_at` | yes | RFC 3339. |
| `change_cause` | yes | `created`, `evidence` or `classification`. |
| `evidence_state` | yes | State **at this revision** (D6). |
| `evidence_reason` | yes (nullable) | Reason enum (§4.3); non-null iff the state is `unavailable` or `expired`. |
| `asset_id` | yes | UUID; siblings share it. |
| `snapshot_url` | no | Present iff `evidence_state = ready`. |

The event is a change notification. Box, transform and classifications are
fetched with `GET /api/observations/{id}`, which is authoritative. The L5
validator rejects missing required fields, a subject/`camera_uuid` mismatch, an
unknown cause or state, and an inconsistent reason or `snapshot_url`. The
generic privacy rules still apply.

## 7. History and cursor (`GET /api/observations`)

### 7.1 Query parameters

| Param | Type | Default | Rule |
| --- | --- | --- | --- |
| `camera_uuid` | comma list of uuid | all authorized | ≤ 10 values (D1). Each must be authorized for `live.view`, otherwise `403 forbidden`. Unknown UUIDs are treated as unauthorized. |
| `label` | comma list | all | ≤ 8 values, each 1..31 bytes, exact ASCII case-insensitive match on the original label. |
| `since` | RFC 3339 | none | Inclusive lower bound on `observed_at`. |
| `until` | RFC 3339 | none | Exclusive upper bound on `observed_at`. `since < until` when both are given, otherwise `400`. |
| `cursor` | string | start of retained history | Opaque cursor (§7.4), or the literal `head` (D10). |
| `limit` | integer | 100 | 1..500. Counts **observations** after in-page dedup. |

Without a camera filter, the query is implicitly limited to cameras where the
principal holds `live.view`. Observations whose capture camera is deleted are
excluded, and single fetches of them return `404`. This follows the existing
deleted-camera behaviour of timeline/playback.

### 7.2 Response

```json
{ "items": [ /* Observation */ ], "next_cursor": "eyJ2IjoxLCJzZXEiOjEwNDIs...", "has_more": false }
```

`items` hold **current** observation state at read time, ordered by the seq of
the change that included them. An item's `revision` may exceed the change that
selected it. Consumers dedupe by `(source, id, revision)`.

### 7.3 Page algorithm (normative)

1. Decode the cursor to `S` (§7.4) and validate it (§7.5). An absent cursor
   means `S = oldest_seq - 1`.
2. Take the watermark `W = head` once, at query start. `head` =
   `max(MAX(seq), oldest_seq - 1)`.
3. Scan changes with `S < seq ≤ W` in ascending `seq`, applying authorization
   and filters. Filters use immutable observation fields only.
4. Add each matching change's observation if it is not already in the page.
   Stop **before** a change that would add observation number `limit + 1`.
   Also stop after scanning 5,000 change rows (scan cap, provisional).
5. `next_cursor` = the seq of the last change **scanned**, whether included or
   filtered out, or `S` if none were scanned. It is never derived from the last
   returned item.
6. `has_more` = whether any change exists with `next_cursor < seq ≤ W`.

Consequences:

- An empty `items` with `has_more = true` is valid (scan cap or filters), and
  clients MUST continue.
- A pending→ready transition is a new change with a higher seq, so it is
  returned even after the `created` change was read.
- Writes after `W` are seen on a later request, so pages never loop.
- `cursor=head` returns `items: []`, `next_cursor` at `W` and
  `has_more: false`.

A cursor is valid only for the principal and filter set that produced it. A
consumer that changes cameras or labels, or gains permissions, MUST restart from
an absent cursor to see older changes for the new scope.

### 7.4 Cursor encoding

The cursor is base64url without padding of the compact JSON
`{"v":1,"seq":<int>,"epoch":"<epoch_uuid>"}`, for example
`{"v":1,"seq":1042,"epoch":"66666666-6666-4666-8666-666666666666"}` →
`eyJ2IjoxLCJzZXEiOjEwNDIsImVwb2NoIjoiNjY2NjY2NjYtNjY2Ni00NjY2LTg2NjYtNjY2NjY2NjY2NjY2In0`.

Clients MUST treat it as opaque. Malformed input, `v ≠ 1`, a negative seq or
over 256 chars returns `400 invalid_cursor`. lightNVR has no base64url helper
today (`include/utils/base64.h` is standard alphabet); L2 or L8 adds one.

### 7.5 Expiry rules (`410 cursor_expired`)

`observation_meta.oldest_seq` is the lowest seq that may still be retained
(last pruned seq + 1; initially 1). A cursor `S` returns `410` when any of
these hold:

- `S < oldest_seq - 1`: a change after `S` was pruned.
- `epoch ≠ observation_meta.epoch_uuid`: DB restore, import or recreation.
- `S > head`: seq rewound without an epoch change.

```json
{
  "error": "Cursor is older than retained observation history",
  "code": "cursor_expired",
  "oldest_cursor": "eyJ2IjoxLCJzZXEiOjAsImVwb2NoIjoiNjY2NjY2NjYtNjY2Ni00NjY2LTg2NjYtNjY2NjY2NjY2NjY2In0",
  "oldest_observed_at": "2026-10-09T00:00:04.112Z"
}
```

`oldest_cursor` encodes `oldest_seq - 1` in the **current** epoch.
`oldest_observed_at` is the minimum retained `observed_at`, or null when
history is empty. The consumer records "recovery required" and resumes from
`oldest_cursor`. Changes in the gap are reported as unrecoverable, never
silently skipped.

### 7.6 DB-generation epoch (D9)

- `epoch_uuid` is created with the table (L1) and regenerated by:
  1. any in-process restore or import, including both
     `restore_database_from_backup()` call sites in `src/database/db_core.c`
     and any future import tool, via `observation_meta_regenerate_epoch()`;
  2. startup rewind detection. A sidecar
     `<storage_path>/observations/.cursor_head.json` holds `{epoch, head_seq}`,
     written atomically (tmp + rename) when the head advances, at most once
     per second by the relay. At startup, if the sidecar epoch equals the DB
     epoch but the DB `head < head_seq`, the DB was restored, so the epoch is
     regenerated. A missing sidecar is recreated without regenerating.
- The epoch is not `event_installation_uuid`, which survives a restore while
  `seq` rewinds.
- Known residual window: a restore to a backup newer than the last sidecar
  write is caught only by `S > head`. This is accepted and covered by L11's
  restore test.

## 8. Single observation and snapshot

### 8.1 `GET /api/observations/{id}`

Returns `200` with one Observation (§3), including classifications. An unknown
ID, an unauthorized ID (no `live.view` on the capture camera) and a deleted
capture camera are indistinguishable: `404 observation_not_found`.

### 8.2 `GET /api/observations/{id}/snapshot`

| Evidence state | Status | Headers | Body |
| --- | --- | --- | --- |
| `ready` | `200` | `Content-Type: image/jpeg`, `Cache-Control: private, max-age=300`, `ETag: "<asset_id>"` | JPEG bytes, unannotated |
| `pending` | `202` | `Retry-After: <1..30>` | `{error, code:"evidence_pending", retry_after_seconds}` |
| `unavailable` | `404` | | `{error, code:"evidence_unavailable", reason}` |
| `expired` | `410` | | `{error, code:"evidence_expired", reason, expired_at}` |
| (no read authority / unknown) | `404` | | `{error, code:"observation_not_found"}` |
| (`live.view` but no `snapshot.create`) | `403` | | `{error, code:"forbidden"}` (D4) |

- It never substitutes a latest or go2rtc frame, and never returns a
  different asset.
- `If-None-Match` matching the ETag returns `304`.
- `Retry-After`: `1` when the asset is under 5 s old, otherwise
  `min(30, 2 × queue items ahead)`. It is always an integer from 1 to 30.
  lightNVR has no per-asset queue position, so it uses the whole evidence
  backlog (frames queued for encoding) as an upper bound on the items ahead.
  Clients honour it and give up polling after 120 s total, relying on the
  `evidence` change event or the cursor instead.

## 9. Capability document (`GET /api/integrations/capabilities`)

This endpoint requires an authenticated principal (OpenAPI kind
`authenticated`). It returns only that principal's scope. Unsupported features
are declared explicitly; consumers MUST NOT infer support from an endpoint
returning an error.

```json
{
  "contract_versions": {
    "observations": 1,
    "classifications": 1,
    "observation_event": "io.lightnvr.detection.observation_changed.v1"
  },
  "source": {
    "id": "urn:lightnvr:11111111-1111-4111-8111-111111111111",
    "kind": "lightnvr",
    "installation_uuid": "11111111-1111-4111-8111-111111111111",
    "product_version": "0.48.0"
  },
  "principal": { "writer_namespace": "yawamf" },
  "operations": {
    "observations.history":  { "supported": true,  "authorized": true },
    "observations.snapshot": { "supported": true,  "authorized": true },
    "observations.annotate": { "supported": false, "authorized": false, "reason": "not_available_in_this_version" },
    "track_lifecycle":       { "supported": false, "authorized": false, "reason": "not_implemented" },
    "recorded_frame":        { "supported": false, "authorized": false, "reason": "not_implemented" },
    "event_clip":            { "supported": false, "authorized": false, "reason": "not_implemented" },
    "range_clip":            { "supported": false, "authorized": false, "reason": "not_implemented" }
  },
  "limits": {
    "history_page_default": 100,
    "history_page_max": 500,
    "history_filter_cameras_max": 10,
    "history_filter_labels_max": 8,
    "max_objects_per_batch": 20,
    "label_max_bytes": 31,
    "snapshot_retry_after_max_seconds": 30,
    "annotations_per_observation_max": 8
  },
  "history": {
    "retention_days": 30,
    "oldest_cursor": "eyJ2IjoxLCJzZXEiOjAs...",
    "oldest_observed_at": "2026-10-09T00:00:04.112Z"
  },
  "delivery": {
    "mqtt_enabled": true,
    "topic": "lightnvr/v1/events/io.lightnvr.detection.observation_changed.v1/{camera_uuid}"
  },
  "cameras": [
    { "camera_uuid": "22222222-2222-4222-8222-222222222222",
      "evidence_enabled": true,
      "actions": ["live.view", "snapshot.create"] }
  ]
}
```

- `supported` means the installation implements the operation. `authorized`
  means the principal holds the required action on at least one camera.
  `reason` is present when either is false. Its values are
  `not_implemented`, `not_available_in_this_version`, `not_authorized`,
  `invalid_writer_namespace` and `reserved_writer_namespace`.
- `observations.annotate` was `supported: false` (`not_available_in_this_version`)
  before L14. Since L14 it is `supported: true`, and `authorized` is true when
  the principal has a writer namespace and holds `observations.annotate` on at
  least one camera. Otherwise `reason` is `reserved_writer_namespace` or
  `invalid_writer_namespace` (no namespace; this wins) or `not_authorized`.
- `principal.writer_namespace` is null when the principal has no valid
  namespace (§10.2).
- `cameras` lists at most 1,024 cameras where the principal holds any of
  `live.view`, `snapshot.create` or `observations.annotate`. `actions` is the
  subset held. Full camera discovery stays on the existing per-camera-filtered
  `GET /api/streams` (`listStreams`), which returns `camera_uuid` and needs no
  administrator settings access.
- `history.*` and `delivery.*` reflect current configuration. `topic` uses
  the configured prefix, with `{camera_uuid}` left as a literal placeholder.

## 10. Classification annotations (P2)

### 10.1 Key

The path key is `<writer_namespace>/<name>`, sent URL-encoded as one path
segment (`%2F`) or as two segments, `…/classifications/{namespace}/{name}`.
C2 picks one form and OpenAPI records it; the two-segment form is preferred.
Both parts match `^[a-z0-9][a-z0-9_.-]{0,31}$`.

### 10.2 Writer namespace and identity

- The namespace is **derived by the server** from the authenticated principal:
  the user's username, lowercased, with every character outside `[a-z0-9_.-]`
  replaced by `-`. It must then match the pattern above, or the principal has
  no namespace (`invalid_writer_namespace`). It is never taken from the
  request body (D3).
- Reserved namespaces cannot be derived. They are `lightnvr`, `system`,
  `owner` and `local` (`reserved_writer_namespace`). `lightnvr` is held for
  future native owner decisions.
- Each row stores `writer_user_id` and the namespace at creation. A write to an
  existing key is allowed only when `writer_user_id` matches, so a later user
  reusing a username cannot overwrite (`403 namespace_forbidden`).
- A key whose namespace is not the principal's derived namespace returns
  `403 namespace_forbidden`. The same applies if the body contains a
  `writer_namespace` or `writer` field naming another namespace; when it names
  the principal's own namespace, the field is accepted and ignored.

### 10.3 Annotation object

| Field | Type | Null | Rule |
| --- | --- | --- | --- |
| `key` | string | no | `<namespace>/<name>` (response only). |
| `writer_namespace` | string | no | Response only. |
| `classifier` | string | no | 1..64 bytes, for example `yawamf`. |
| `model_version` | string | yes | ≤ 64 bytes. |
| `label` | string | no | 1..128 bytes UTF-8, for example `Black-capped Chickadee`. General object classes are allowed; there are no bird-specific fields. |
| `taxonomy_id` | string | yes | ≤ 64 bytes, for example `inat:144815`. Opaque to lightNVR. |
| `confidence` | number | no | 0.0..1.0. |
| `classified_at` | time | no | Classifier's own time. |
| `input_asset_id` | uuid | no | Must equal the observation's `evidence.asset_id`. |
| `input_revision` | integer | no | 1 ≤ value ≤ the observation's current revision. |
| `kind` | enum | no | `automated` or `owner_confirmed`. |
| `annotation_revision` | integer | no | Response only. Starts at 1 and increases by 1 per stored change. |
| `created_at`, `updated_at` | time | no | Response only. |

A PUT body holds exactly the request fields above (`classifier`,
`model_version`, `label`, `taxonomy_id`, `confidence`, `classified_at`,
`input_asset_id`, `input_revision`, `kind`) plus an optional
`expected_revision` (integer ≥ 0; 0 or absent means "must not exist").
Unknown fields return `400`. The original detector `label`, `confidence` and
`box` are never modified.

### 10.4 PUT rules, evaluated in order

1. Authenticate and resolve the observation's capture camera.
   - No `live.view` and no `observations.annotate` on it → `404
     observation_not_found`.
   - `live.view` without `observations.annotate` → `403 forbidden`.
2. Namespace checks (§10.2) → `403 namespace_forbidden`.
3. Body validation → `400 invalid_request`.
4. Evidence check: `input_asset_id` ≠ the observation's asset, `input_revision`
   out of range, or the evidence state not in {`ready`, `expired`} → `422
   evidence_mismatch`.
5. **Idempotent retry.** The key exists and every request field equals the
   stored value (confidence compared to 1e-6) → `200`, `changed: false`. This
   holds regardless of `expected_revision`, so a retry after a lost response
   is a no-op. Nothing is written and the revision is not bumped.
6. **Owner precedence.** The stored row is `owner_confirmed` and the request is
   `automated` → `409 owner_decision_locked`. An `owner_confirmed` request MAY
   replace either kind, subject to rule 7.
7. **Revision.** If the key exists, `expected_revision` must equal
   `annotation_revision`; if absent, `expected_revision` must be 0 or omitted.
   A mismatch → `409 revision_conflict`.
8. Otherwise store the annotation (create → `201`, update → `200`). In the same
   transaction, bump the observation revision with cause `classification` and
   write its change and publication rows.

Response for `200` and `201`:

```json
{ "changed": true, "observation_revision": 4,
  "annotation": { "key": "yawamf/species", "writer_namespace": "yawamf", "classifier": "yawamf",
                  "model_version": "convnext-v2-2026.10", "label": "Black-capped Chickadee",
                  "taxonomy_id": "inat:144815", "confidence": 0.93,
                  "classified_at": "2026-11-08T14:03:15.002Z",
                  "input_asset_id": "88888888-8888-4888-8888-888888888888", "input_revision": 2,
                  "kind": "automated", "annotation_revision": 1,
                  "created_at": "2026-11-08T14:03:15.120Z", "updated_at": "2026-11-08T14:03:15.120Z" } }
```

The `409` bodies carry `current` (the stored annotation) so the writer can
reconcile.

### 10.5 Reads and display

`classifications.automated` and `classifications.owner_confirmed` list
annotations by `kind`, ordered by `updated_at` descending. Each key appears in
exactly one list. Displays (L15) show owner-confirmed results over automated
ones, and always keep the original detector label and confidence visible.

There are at most 8 keys per observation. Creating a ninth key returns
`409 annotation_limit` (§11). This check runs at step 8 of §10.4, before the
insert.

## 11. Error catalog

Every error body has the form `{"error": "<human message>", "code": "<code>",
...extra}` (D2). Clients branch on `status` + `code` and never on the message.

| Status | `code` | Endpoints | Extra fields | Consumer outcome class |
| --- | --- | --- | --- | --- |
| 400 | `invalid_request` | all | `field` (optional) | permanent; fix the request |
| 400 | `invalid_cursor` | list | | restart without a cursor |
| 401 | (existing auth layer body) | all | | credentials invalid |
| 403 | `forbidden` | list (unauthorized `camera_uuid`), snapshot (D4), PUT | | permission missing |
| 403 | `namespace_forbidden` | PUT | | permanent; misconfigured writer |
| 404 | `observation_not_found` | get, snapshot, PUT | | `Missing` (unknown or not visible) |
| 404 | `evidence_unavailable` | snapshot | `reason` | `Unavailable` (permanent) |
| 202 | `evidence_pending` | snapshot | `retry_after_seconds`; `Retry-After` header | retry later |
| 410 | `evidence_expired` | snapshot | `reason`, `expired_at` | `Expired` (permanent) |
| 410 | `cursor_expired` | list | `oldest_cursor`, `oldest_observed_at` | `RecoveryRequired` |
| 409 | `revision_conflict` | PUT | `current` | refetch and reconcile |
| 409 | `owner_decision_locked` | PUT | `current` | drop the automated write |
| 409 | `annotation_limit` | PUT | `limit` | permanent |
| 422 | `evidence_mismatch` | PUT | `observation_asset_id`, `evidence_state` | drop with reason |
| 501 | `not_implemented` | C2 stubs (the PUT until L14; no route returns it since) | | capability absent |
| 503 | `service_unavailable` | all | `Retry-After` header | `TemporarilyUnreachable` |
| 500 | `internal_error` | all | | `TemporarilyUnreachable` |

Network failures and timeouts on the consumer side also map to
`TemporarilyUnreachable`. `202`, `404 evidence_unavailable`, `410
evidence_expired`, `404 observation_not_found` and `410 cursor_expired` are the
distinct A8 outcomes.

## 12. Authorization and audit

### 12.1 Mapping (confirms plan decision 7)

| Operation | OpenAPI `x-lightnvr-authorization` | Action | Notes |
| --- | --- | --- | --- |
| `getIntegrationCapabilities` | `{version:1, kind:"authenticated"}` | none | Body scoped to the principal. |
| `listObservations` | `{version:1, kind:"camera", requirements:[{action:"live.view", resource:"raw"}]}` | `live.view` | Same as `getEvidenceDetections` (`GET /api/detection/results/{name}`). The set is evaluated per capture camera. |
| `getObservation` | camera, `live.view` | `live.view` | |
| `getObservationSnapshot` | camera, `snapshot.create` | `snapshot.create` | Same as `getEvidenceSnapshot`. Plus `live.view` for the D4 distinction. |
| `putObservationClassification` | camera, `observations.annotate` | `observations.annotate` | P2. |

- Authorization always resolves the observation's capture-time `camera_uuid`
  through Fleet 02. Handlers use `httpd_evaluate_camera_identity_action()`,
  not the `httpd_authorize_*` helpers that write `403` responses themselves.
  Those helpers would turn an opaque-ID miss into a `403` and leak existence.
- `observations.annotate` (P2, L13):
  - It is appended to the C enum in `include/core/authorization.h` and to
    `action_catalog[]` in `src/core/authorization.c`, as
    `{AUTHZ_OBSERVATIONS_ANNOTATE, "observations.annotate", "Integrations",
    "Annotate detection observations with classifier results", true, false,
    true}`. It is camera-scoped and non-destructive.
  - Migration 0096 inserts it with `sort_order 200` and `bit_index 19`. 0052
    defined bits 0–14 (via 0056) and 0078 bits 15–18; no other migration
    touches `authz_actions`.
  - Only the built-in Administrator role receives it (the 0078 pattern).
  - It is listed in the three OpenAPI action enums.
  - It is never required for reads, and `camera.configure` and admin are never
    required for writes.

### 12.2 Audit

| Event | Mechanism | Recorded |
| --- | --- | --- |
| Metadata reads | Existing read-decision modes (`audit_log_authorization`) for `live.view` | as today |
| Snapshot fetch | `audit_log_authorization` for `snapshot.create`, with `target_type = "observation"` and `target_uuid` = observation ID | camera_uuid, asset_id, outcome |
| Annotation PUT (every outcome) | `audit_log_operation` with action `observations.annotate`, operation `annotate`, `target_type = "observation"` | camera_uuid, key, kind, `result` (`created\|updated\|noop\|revision_conflict\|owner_decision_locked\|evidence_mismatch\|namespace_forbidden`, plus `forbidden`, `observation_not_found`, `invalid_request`, `annotation_limit` and `internal_error`), annotation_revision before/after (null when unknown); the principal and `api_token_uuid` come from the standard audit columns. The audit `outcome` is `success` for created/updated/noop, `denied` for forbidden, namespace_forbidden and a hidden observation's 404, `error` for 500 and `failure` otherwise. A malformed observation ID (400 before lookup) and 401 are not audited. |

Image bytes, request bodies beyond the fields listed, tokens, cookies and
authorization headers are never logged or audited.

## 13. Budgets and configuration (provisional)

All values are provisional until X2 measures them on the C4 fixture (§19).

| Setting | Scope | Default | Bounds | Storage |
| --- | --- | --- | --- | --- |
| Evidence enabled | camera | **off** | bool | `streams.observation_evidence_enabled` |
| Eligible labels | camera | `''` (all object labels) | CSV, ≤ 16 labels | `streams.observation_labels` |
| Media max age | camera | 7 days | 1..90 | `streams.observation_max_age_days` |
| Media max bytes | camera | 2 GiB | 64 MiB..1 TiB | `streams.observation_max_bytes` |
| History retention | global | 30 days | 1..365 | `system_settings.observation_history_retention_days` |
| Global queue bytes | global | 32 MiB | 4..512 MiB | `system_settings.observation_global_queue_bytes` |
| Per-camera queue depth | constant | 8 frames | | compile-time |
| Drop-record ring | constant | 256 records | | compile-time |
| Encoder worker concurrency | constant | 1 | | compile-time |
| Relay batch | constant | 64 publications | | compile-time |
| Relay outbox share cap | constant | 2,000 pending rows | | compile-time (D8) |
| History scan cap per page | constant | 5,000 changes | | compile-time |

- With evidence disabled on every camera, no queue is allocated, no worker
  thread starts, and no images are written.
- Admission never blocks. It returns `ACCEPTED`, `DROPPED_QUEUE_FULL`,
  `DROPPED_BUDGET` or `DISABLED` immediately.
- Settings are changed with `camera.configure` (L9) and take effect without a
  restart.

## 14. Retention and storage

- Media expiry is per camera. The oldest unprotected assets beyond the age or
  byte budget become `expired(retention_expired)`, and their files are deleted
  only when no retained observation references them and they are not
  `protected`.
- A shared asset survives until its last reference is pruned.
- History prune deletes change rows in ascending seq while their `created_at`
  is older than the history retention. It then deletes observations with no
  remaining change rows, and assets with no remaining observations. It sets
  `oldest_seq` = last pruned seq + 1 in the same transaction.
- Media retention (7 days) is shorter than metadata retention (30 days), so an
  `expired` observation stays queryable.
- If the byte budget is exhausted and only protected assets remain, new work is
  rejected as `budget_exhausted` and the flag is surfaced (§15).
- Recording deletion never deletes evidence, and evidence deletion never
  deletes recordings.
- Startup reconciliation runs before the worker and relay:
  - delete `*.jpg.tmp` files;
  - `pending` assets without a final file → `unavailable(interrupted)`;
  - `ready` assets with a missing file → `unavailable(write_failed)`;
  - delete orphan final files.
- Rollback: disable evidence per camera; evidence and annotations are kept. The
  0095 and 0096 down-migrations drop data, so take a backup first.

## 15. Diagnostics

L10 exposes these through the existing metrics and health surfaces. Names are
final in L10; the semantics here are fixed.

`GET /api/metrics` (Prometheus, `system.admin`) families, all present with
zero values while evidence is unused:

| Item | Metric |
| --- | --- |
| Accepted | `lightnvr_observation_accepted_total` |
| Dropped by reason | `lightnvr_observation_dropped_total{reason="queue_full"\|"budget_exhausted"\|"disabled"\|"untracked"}` |
| Backlog | `lightnvr_observation_evidence_backlog_items`, `_backlog_bytes`, `_drop_ring_depth` |
| High-water marks | `lightnvr_observation_evidence_high_water_items`, `_high_water_bytes` |
| Queue exists | `lightnvr_observation_evidence_queue_allocated`, `_worker_running` (0 when unused) |
| Outcomes | `lightnvr_observation_evidence_ready_total`, `_unavailable_total`, `_bytes_written_total` |
| Failures by reason | `lightnvr_observation_evidence_failures_total{reason="encode_failed"\|"write_failed"\|"db_error"\|"disabled"\|"interrupted"}` |
| Retained bytes | `lightnvr_observation_evidence_retained_bytes{camera_uuid}`, `_budget_bytes{camera_uuid}` |
| Budget exhausted | `lightnvr_observation_evidence_budget_exhausted{camera_uuid}`, `_budget_exhausted_cameras` |
| Oldest cursor | `lightnvr_observation_history_oldest_seq` (cursor = seq − 1), `_head_seq`, `_oldest_observed_timestamp_seconds`, `_observations` |
| Publications | `lightnvr_observation_publications{state="pending"\|"failed"\|"expired"}` |
| Relay | `lightnvr_observation_relay_lag_seconds`, `_relay_running`, `_relay_events_total{outcome}`, `lightnvr_observation_history_epoch_regenerations_total` |

Per-camera rows cover cameras holding `ready` media as of the last L4
retention pass (at most 5 minutes old). The same snapshot is returned as
`observation_pipeline` in `GET /api/system/info` (OpenAPI schema
`ObservationPipelineDiagnostics`) and shown on the System page;
`active: false` there means the feature is unused.

- Accepted observations total.
- Dropped observations by reason (`queue_full`, `budget_exhausted`,
  `disabled`), plus `dropped_untracked` when the drop ring is full.
- Evidence backlog (items and bytes), queue high-water marks, and failures by
  reason.
- Retained evidence bytes per camera.
- Oldest recoverable cursor seq and `oldest_observed_at`.
- Publications `pending`, `failed` and `expired`, and relay lag in seconds (age
  of the oldest pending publication).
- The `budget_exhausted` flag per camera.
- Setup probe: `GET /api/observations?camera_uuid=<uuid>&limit=1`.
  - `200` with empty `items` means "no eligible detections yet".
  - Transport or auth failures surface as HTTP or network errors and are never
    reported as an empty list.

## 16. YA-WAMF consumer rules

These rules bind W1–W10 and refine plan decision 8.

### 16.1 Identity mapping

| YA-WAMF column | lightNVR value |
| --- | --- |
| `detections.frigate_event` (`UNIQUE NOT NULL`, the internal event key) | `lnvr:<installation_uuid>:<observation_id>` (78 chars; cannot collide with Frigate's `<ts>.<rand>-<suffix>`) |
| `source_kind` | `lightnvr` (Frigate rows: `frigate`) |
| `source_instance` | `installation_uuid` from `capabilities.source.installation_uuid` |
| `external_observation_id` | observation `id` |
| `source_camera_id` | `camera_uuid` (identity) |
| `camera_name` | `stream_name_at_capture` (display only) |

Other tables (`bird_observations`, media cache, `processing_job_state`,
favourites, corrections) keep keying on `frigate_event`.

### 16.2 Admission

1. Dedupe deliveries on `source + envelope id`.
2. Drop `change_cause = classification` messages for classification and
   notification. They may refresh the display only.
3. Before acting, fetch current state with `GET /api/observations/{id}`, which
   is authoritative; the payload is only a trigger. Admission is idempotent on
   the observation key. If a detection row exists, a later change never creates
   another detection or notification.
4. Classify only when the current `evidence.state = ready`.
   - `pending` → do nothing durable. The ready transition arrives later as its
     own change, live or by cursor.
   - `unavailable` or `expired` before admission → record "skipped: <reason>"
     in diagnostics. No detection row is created.
5. Crop each observation with its own box against the evidence image
   dimensions (§3.2). Siblings are classified separately.
6. Live and recovery paths share steps 1–5.

### 16.3 Multi-bird mapping

- One observation maps to one detection row with one `bird_observations` row
  at `bird_index = 0`.
- Several objects in one frame become sibling detections that share
  `evidence.asset_id`. The full image is cached once per asset; crops are per
  observation.
- Visit grouping (camera + time window) places siblings in the same visit and
  never claims identity.
- Intended difference: Frigate counts a two-bird event as one detection with
  two `bird_observations`, while lightNVR yields two detections. W4 and W9
  must test visit counts, leaderboard counts and multi-bird display for
  siblings, and document any intended difference.

### 16.4 Notification policy for sources without `track_lifecycle`

For these sources, each admitted observation is a single-shot lifecycle:

| `notify_mode` | Frigate (unchanged) | lightNVR |
| --- | --- | --- |
| `silent` | never | never |
| `final` | on `end` only | on the **first admitted ready classification** of a detection |
| `standard` | `new`, or `update` that inserted | first admitted ready classification |
| `realtime` | `new` + updates | first admitted ready classification; never on `classification` changes |
| default | `notify_on_insert` / `notify_on_update` | `notify_on_insert` at first admission |

- All modes stay bounded by `settings.notifications.notification_cooldown_minutes`
  (D11).
- W7 records the intent durably in the detection-save transaction by
  extending the existing `mark_notified` / `notified_at` state. It does not
  add a parallel flag.
- Items admitted by history recovery get the intent `suppressed_recovery` (no
  standard notification) by default.
- Ambiguous external delivery is recorded as `unknown` and surfaced, never
  retried blindly.

### 16.5 Recovery and cursor

- `source_cursors(source_kind, source_instance, cursor, epoch, updated_at,
  last_error)` stores the cursor together with a fingerprint of the camera and
  label filter. A changed filter resets it to absent (§7.3).
- Advance the cursor only after every item in the page is durably admitted or
  recorded as skipped with a reason.
- `410 cursor_expired` → store "recovery required", resume from
  `oldest_cursor`, and report the gap in health.
- Retryable failures go to `processing_job_state` with a dedicated `pipeline`.
  Write-back uses `pipeline = 'lightnvr_writeback'`. No new job tables.

### 16.6 Snapshot outcomes

| Response | Outcome |
| --- | --- |
| `202` | `Pending`; honour `Retry-After`, give up after 120 s |
| `404 evidence_unavailable` | `Unavailable(reason)` |
| `410` | `Expired(reason)` |
| `404 observation_not_found` | `Missing` |
| network error, `5xx` | `TemporarilyUnreachable` |

Recovery never replaces evidence with a current frame.

### 16.7 Write-back (W10)

- PUT to `<principal.writer_namespace>/species` with `kind = automated` for a
  model result, or `owner_confirmed` for an owner correction.
- Send `expected_revision` from the last stored response.
- `409 revision_conflict` → refetch and reconcile; a local owner correction
  wins.
- `409 owner_decision_locked` → drop the automated write.
- `422` → drop with a reason.
- Write-back never creates detections or notifications, and an outage leaves
  the YA-WAMF result usable and marked "sync pending".

## 17. Reserved identifiers

| Item | Value | Owner task |
| --- | --- | --- |
| lightNVR migration (P1) | `0095_add_object_observations.sql` (main is at `0094`; re-check before merge) | L1 |
| lightNVR migration (P2) | `0096_add_observation_classifications.sql` | L13 |
| Authz action | `observations.annotate`, `bit_index 19`, `sort_order 200` | L13 |
| Event type | `io.lightnvr.detection.observation_changed.v1` | L5 |
| Message-ID namespace | `0bc01005-a431-522b-9124-a114f67d187a` | L5 |
| Tables | `object_observations`, `observation_assets`, `observation_changes`, `observation_publications`, `observation_meta`, `observation_classifications` (`observation_asset_refs` only if per-reference protection is needed) | L1, L13 |
| Operation IDs | `getIntegrationCapabilities`, `listObservations`, `getObservation`, `getObservationSnapshot`, `putObservationClassification` (never `*Evidence*`) | C2 |
| OpenAPI schemas | `Observation`, `ObservationEvidence`, `ObservationTransform`, `ObservationPage`, `ObservationChangeEvent`, `ObservationClassification`, `IntegrationCapabilities`, `ObservationError` | C2 |
| Storage paths | `<storage_path>/observations/…`, `.cursor_head.json` | L3, L4, L5 |
| Reserved writer namespaces | `lightnvr`, `system`, `owner`, `local` | L14 |
| YA-WAMF key prefix | `lnvr:` | W2, W4 |
| YA-WAMF job pipeline | `lightnvr_writeback` (others named by W6) | W6, W10 |

## 18. Supported-version matrix (stub)

X1 and X3 fill this in with the tested versions; L12 publishes it. The
published matrix, with what each component was tested by, is in the
[YA-WAMF deployment recipe](YA_WAMF.md#supported-version-matrix); the rows
below stay the release targets.

| Component | Version | Contract | Status |
| --- | --- | --- | --- |
| lightNVR | first release containing L1–L11 (≥ 0.48.0, TBD) | observations 1, event v1 | not yet released |
| lightNVR (P2) | first release containing L13–L15 (TBD) | classifications 1 | not yet released |
| YA-WAMF | fork branch `feature/lightnvr-provider` on v2.23.1 (`53540f48`) | observations 1 | prototype; upstream acceptance or a named maintainer required (PRD §7) |
| Frigate (YA-WAMF source) | as supported by YA-WAMF v2.23.1 | n/a | unchanged |
| MQTT broker | Mosquitto 2.x (E2E harness) | | TBD by X1 |
| Bird-capable detector / model | TBD by X1 (pinned model fixture) | | TBD |

## 19. Performance

Task C4 appends the declared performance fixture (hardware, stream count,
resolutions, fps, detector cadence, retention budgets) and the disabled
baseline here. X2 compares enabled and disabled runs on the same input. The
release threshold is zero additional recording gaps and ≤ 5% higher p95
recording-path latency. X2 also measures memory, CPU, disk and queue high-water
marks, plus producer submit latency, which must never wait on I/O. X2 replaces
the provisional values in §13.

## 20. PRD traceability

| PRD ID | Requirement (summary) | Contract section | Implementing tasks |
| --- | --- | --- | --- |
| R1 | Stable observation ID per object, batch ID + object index, camera UUID and installation | §2, §3.1, §5.2 | L1, L2, L6, L7 |
| R1 | Field table: id/source/camera, observed vs created, detector, label/confidence, box, evidence, track, recording_reference, revision | §3.1–§3.5, §4.1 | L1, L2, L8 |
| R1 | Rename/delete never transfers history; reused track ID never joins; recording not required | §2, §3.4, §3.5, §7.1 | L2, L6, L11 |
| R2 | Owned copy of local inference frame, encoded off-thread; HTTP retains submitted image + transform; no second go2rtc fetch | §4.1 (`origin`), §4.4, §13 | L3, L6, L7 |
| R2 | Unannotated image + transform metadata; low-res not described as recording image | §3.2, §4.1, §4.4 | L3, L6, L7 |
| R2 | States pending/ready/unavailable/expired + bounded reason; ONVIF unavailable | §4.2, §4.3 | L2, L3, L7 |
| R2 | Ready only after readable + committed; atomic finalize; startup reconciliation; transactional publication record | §4.2, §5.1, §5.2, §14 | L2, L3, L4, L5 |
| R3 | Existing topics unchanged; new versioned event via registry/routing/outbox | §6 | L5 |
| R3 | One message ID per revision, stable across retries; dedupe by source + message_id / observation_id | §5.4, §5.5, §16.2 | L5, W4 |
| R3 | Authorized camera, label, time filters; bounded pages; opaque persistence-ordered cursor; watermark | §7.1–§7.4 | L2, L8 |
| R3 | Expired cursor → recovery-required + boundary; route suppression is not deletion; classification-only changes do not reclassify | §5.3, §5.5, §7.5, §7.6 | L2, L5, L8, W4, W6 |
| R4 | Capabilities, list, get, snapshot, PUT classification; distinct resource; pending retry guidance; expired vs unknown; no cross-scope leak | §7, §8, §9, §10, §11 | C2, L8, L14 |
| R5 | Annotation fields; bounded, general (no bird-specific core fields) | §10.3 | L13, L14 |
| R5 | Writer owns namespace; same payload no-op; expected revision; reject mismatched evidence; cannot claim another writer or overwrite owner; detector output preserved; automated vs owner separate | §10.2, §10.4, §10.5 | L13, L14, W10 |
| R5 | Minimal classification display in lightNVR | §10.5 | L15 |
| R6 | Fleet 02 capture-time identity; `live.view` metadata; `snapshot.create` images; narrow annotate action; audit without credentials/bytes | §12 | L8, L13, L14 |
| R6 | Per-camera opt-in, age/byte limits, bounded queues/concurrency; no legacy fixed-count policy; shared assets; recording vs evidence deletion explicit | §4.5, §13, §14 | L3, L4, L9 |
| R6 | Retention advertises real availability; budget exhaustion visible; never block recording | §4.1 (`expires_at`), §9 (`history`), §13, §14 | L3, L4, L8, X2 |
| R6 | Diagnostics; disabled deployments allocate nothing | §13, §15 | L3, L10 |
| T1 | Provider interface; Frigate provider preserved; lightNVR provider; no synthetic `end`; no plugin framework | §9, §16.2, §16.4 | W1, W3, W5 |
| T2 | Source kind/instance/external ID identity; legacy backfill; versioned migration; source change never reassigns history | §16.1, §17 | W2 |
| T3 | Provider-specific parsing; media by source identity; history/cursor outcomes; capability-gated services; no `/api/events/` from lightNVR config | §9, §11, §16.2, §16.6 | W3, W4, W6, W8 |
| T4 | Capability-declared features (not inferred from errors); classify ready only; per-object regions; cursor advances after durable accept/skip; durable notification intent | §9, §16.2–§16.5 | W4, W6, W7, W10 |
| A1 | Stable identity + expected label/region | §3, §16.1 | L6, L11, X1 |
| A2 | Moving bird: saved photo and region match inference input (local + HTTP) | §3.2, §4.1, §4.4 | L6, L7, X1 |
| A3 | Two birds → separate identities, aligned regions; shared evidence retained until all refs expire | §3, §14, §16.3 | L2, L4, W4, W9 |
| A4 | Replay ×10 + overlapping recovery → one detection, one notification intent | §5.4, §16.2, §16.4 | W4, W6, W7, W9 |
| A5 | Broker down, 100 observations, restart → all recoverable incl. pending→ready | §5.5, §7.3 | L2, L5, L11, W6 |
| A6 | Interrupted writes/publication: no ready ref to unfinished file; committed changes recovered | §4.2, §5.2, §5.5, §14 | L3, L4, L5, L11 |
| A7 | Camera-scoped token isolation for list/fetch/annotate; rename/delete/name reuse | §7.1, §8, §10.2, §12 | L8, L11, L14 |
| A8 | Distinct outcomes for expired media, expired cursor, unavailable, temporary | §8.2, §11, §16.6 | L8, L11, W5 |
| A9 | Snapshot-only source completes setup; unsupported jobs do not run and explain why | §9 | W3, W8 |
| A10 | Frigate regression preserved across migration | §16.4 (Frigate column), §16.1 | W0, W1–W9 |
| A11 | Saturation: limits hold, drops have reasons, recording healthy, enqueue never waits | §13, §15, §19 | L3, X2 |
| A12 | Write-back retry, stale revisions, owner overwrite: one annotation, explicit conflicts, detector output intact | §10.4, §11 | L13, L14, W10, X3 |
| A13 | Classification updates do not retrigger; feedback outage leaves result usable | §5.3, §16.2, §16.7 | W4, W10, X3 |
| A14 | Operator recipe with scoped credentials; reconnect recovery; versions recorded | §9, §12, §18 | L12, X1 |
