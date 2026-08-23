# lightNVR Event Contract

lightNVR emits versioned operational facts for MQTT, LightNVR Cloud, Home
Assistant, Node-RED, and customer integrations. The core event interface does
not send email, SMS, push, or other human notifications; subscribers decide how
an event should reach a person.

## Envelope

Every event uses a CloudEvents-inspired JSON envelope:

```json
{
  "specversion": "1.0",
  "id": "8a77e095-9079-44d9-8766-b733bc370631",
  "type": "io.lightnvr.camera.offline.v1",
  "source": "urn:lightnvr:11111111-1111-4111-8111-111111111111",
  "subject": "camera/22222222-2222-4222-8222-222222222222",
  "time": "2026-08-23T06:30:00Z",
  "datacontenttype": "application/json",
  "severity": "warning",
  "sensitivity": "operational",
  "data": {
    "reason": "connection_timeout",
    "consecutive_failures": 3
  }
}
```

- `source + id` is the deduplication identity and does not change on retry.
- `type` ends in a schema major version. Consumers ignore unknown data fields;
  breaking changes require a new major version.
- Camera-related subjects use the immutable `camera/<uuid>` form. Storage-wide
  events use `system/storage`.
- `time` is UTC RFC 3339. `datacontenttype` is currently always
  `application/json`.
- `severity` and `sensitivity` come from the registry, not from an individual
  producer.
- Data is limited to 64 KiB and a complete envelope to 96 KiB.
- Passwords, credentials, authorization values, API keys, tokens, cookies, and
  raw filesystem-path fields are rejected recursively. Media uses an
  authenticated logical URL or event-media reference, never a local path.

## Initial registry

| Type | Severity | Sensitivity | Default expiry | Expected rate | Media policy |
| --- | --- | --- | ---: | --- | --- |
| `io.lightnvr.detection.object.v1` | info | operational | 1 hour | high | reference allowed |
| `io.lightnvr.camera.offline.v1` | warning | operational | 1 day | low | forbidden |
| `io.lightnvr.stream.recording_gap.v1` | warning | operational | 7 days | low | reference allowed |
| `io.lightnvr.storage.pressure.v1` | critical | internal | 7 days | low | forbidden |

Expiry is internal delivery metadata and is intentionally not part of the JSON
payload. MQTT 5 delivery may carry it as a message-expiry property; the durable
outbox uses the same value.

## Data schemas

Additional fields are allowed unless they violate the privacy rules above.

### `io.lightnvr.detection.object.v1`

```json
{
  "stream_name": "loading-bay-north",
  "count": 1,
  "detections": [
    {
      "label": "person",
      "confidence": 0.94,
      "x": 0.1,
      "y": 0.2,
      "width": 0.3,
      "height": 0.4,
      "track_id": 17,
      "zone_id": "loading-bay"
    }
  ],
  "snapshot_url": "/api/events/media/event-id"
}
```

`count` must match a non-empty `detections` array. `stream_name` is mutable
display/legacy-routing metadata; consumers use the envelope's camera UUID
subject as identity. Confidence is normalized to `0.0–1.0`. A bounding box may
be omitted when a camera or external detector reports a class without spatial
metadata; when present, all four coordinates are required, normalized to
`0.0–1.0`, and width and height must be greater than zero. `track_id`, `zone_id`,
and an authorization-aware `snapshot_url` are optional.

### `io.lightnvr.camera.offline.v1`

```json
{
  "reason": "connection_timeout",
  "consecutive_failures": 3
}
```

`reason` is a stable machine-readable value and `consecutive_failures` is at
least one.

### `io.lightnvr.stream.recording_gap.v1`

```json
{
  "started_at": "2026-08-23T06:30:00Z",
  "duration_ms": 12500
}
```

`started_at` is UTC RFC 3339 and `duration_ms` is non-negative.

### `io.lightnvr.storage.pressure.v1`

```json
{
  "level": "critical",
  "used_percent": 94.5,
  "free_bytes": 1073741824
}
```

`level` is `warning`, `critical`, or `emergency`; `used_percent` is between zero
and 100. `free_bytes` is optional.

## Producer API

The installation source is a UUID generated once and persisted in
`system_settings.event_installation_uuid`. Producers use
`event_envelope_create()` with that installation URN, a registered type,
immutable subject, occurrence time, and a JSON data object. The constructor
deep-copies and validates the data, generates the immutable event ID, calculates
expiry, and formats time. `event_envelope_serialize()` revalidates before
serialization. Call `event_envelope_clear()` when finished.

Detection paths use `event_producer_publish_detection()` when they already have
the immutable camera UUID, or
`event_producer_publish_detection_for_stream()` at legacy name-only call sites.
Both normalize and enqueue only; neither performs MQTT, snapshot, or other
network work on the caller thread.

## Asynchronous in-process bus

`event_bus_publish()` revalidates an envelope and enqueues a deep copy. It never
invokes a subscriber or performs broker I/O on the producer thread. One
dedicated worker dispatches immutable events to registered subscribers in
registration order.

The queue defaults to 1,024 events and 8 MiB and can be configured to smaller
limits for constrained deployments. When it is full, ordinary events are
dropped with an observable counter. An error/critical event may evict the oldest
lower-severity queued event, preventing a detection burst from crowding out a
storage-critical fact. Accepted, dispatched, rejected, dropped, priority-shed,
callback-delivery, and callback-failure counts are available through
`event_bus_get_stats()`.

Subscribers register before bus startup and must keep callbacks/context alive
until shutdown. A callback runs off the producer thread but should still use
bounded work; durable retry and broker isolation are provided by the outbox and
MQTT delivery layer rather than by blocking the in-process worker.

## Durable delivery outbox

The SQLite outbox persists one validated, already-serialized envelope per
destination. Its identity is `source + id + destination`, so a retry reuses the
same event ID while the same event can still be sent to multiple destinations.
Rows move through this lifecycle:

```text
pending -> delivering -> delivered
   ^            |
   +------------+  retry or expired lease
                |
                +-> dead  permanent failure or expiry
```

Claims are atomic and carry a 30-second lease by default. If a worker exits
without recording an outcome, the row becomes eligible again when the lease
expires. Each claim increments `attempt_count`; retry callers supply the next
eligible time after applying their destination-specific backoff policy. An
event that reaches its registry expiry moves to `dead` and is never claimed.

The repository defaults to 10,000 rows and 64 MiB. Capacity is checked inside
the enqueue transaction. Old delivered/dead rows are reclaimed first. An
incoming error or critical event may then shed the oldest lower-severity pending
row; ordinary events never evict an equal- or higher-severity row. If neither
rule can make room, enqueue returns `EVENT_OUTBOX_FULL` without modifying the
queue. Terminal rows can also be pruned in bounded batches.

Queue statistics expose row and byte totals, state counts, due count, and the
oldest pending timestamp. This repository is the durable persistence boundary;
the MQTT delivery worker that consumes it is a separate layer. Until that
worker is enabled, the P0 MQTT compatibility subscriber described below still
publishes directly from the asynchronous event-bus worker.

## MQTT compatibility destination

The P0 MQTT subscriber publishes every normalized event to
`{topic_prefix}/v1/events/{type}/{subject-id}` as the complete envelope, with
retention disabled for transient facts. For object detections it also decodes
the envelope into the existing `{topic_prefix}/detections/{stream_name}` payload,
snapshot topic, and Home Assistant motion state. This dual publish keeps current
automations working while new consumers adopt stable UUID identity and versioned
schemas. All broker, snapshot, and Home Assistant work runs on the event-bus
worker rather than on capture or detection threads.
