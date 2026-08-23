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

`count` must match a non-empty `detections` array. Confidence and bounding-box
coordinates are normalized to `0.0–1.0`; width and height must be greater than
zero. `track_id`, `zone_id`, and an authorization-aware `snapshot_url` are
optional.

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

Producers use `event_envelope_create()` with a registered type, installation
URN, immutable subject, occurrence time, and a JSON data object. The constructor
deep-copies and validates the data, generates the immutable event ID, calculates
expiry, and formats time. `event_envelope_serialize()` revalidates before
serialization. Call `event_envelope_clear()` when finished.

The next event-bus layer takes ownership of a validated envelope copy and queues
it without performing broker I/O on the producer thread.
