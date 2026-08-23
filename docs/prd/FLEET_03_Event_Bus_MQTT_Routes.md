# PRD — Event Bus & MQTT Routes

**Status**: P0/P1 complete; P2 routing in progress — durable route definitions,
catalog/CRUD API, selector preview, optimistic concurrency, and runtime
type/scope/predicate/schedule evaluation implemented; suppression, multiple
destinations, and UI remain
**Created**: 2026-08-22
**Owner**: TBD
**Priority**: 3 — integration foundation
**Scope**: A versioned internal event contract, durable outbox, MQTT destinations,
routing rules, delivery observability, and an operator-facing Events & Routes UI.

---

## 1. Problem

lightNVR publishes useful detection information to MQTT, but downstream consumers
need stable identity, schemas, delivery guarantees, and events beyond detections.
Adding SMTP, SMS, Telegram, and every future provider to the NVR core would couple
recording reliability to notification integrations and duplicate work better done
by LightNVR Cloud, Home Assistant, Node-RED, or customer systems.

## 2. Product position

lightNVR emits facts and reliably routes them. It does not deliver human
notifications itself.

```text
camera/core event -> normalized event bus -> route rules -> MQTT
                                                     -> LightNVR Cloud -> email/SMS/push
                                                     -> HA/Node-RED/customer service
```

## 3. Goals

- Define a stable, versioned event envelope with immutable event IDs.
- Publish health, recording, storage, security, ONVIF, and detection events.
- Route events by the Fleet 01 selector plus event-specific predicates.
- Survive broker outages without blocking capture or silently losing queued work.
- Support multiple MQTT destinations with strong TLS and credential handling.
- Expose delivery state, testing, retry, and dead-letter diagnostics.
- Preserve current Home Assistant discovery behavior during migration.

## 4. Non-goals

- Native SMTP, SMS, push, Telegram, Slack, or other provider clients.
- A general-purpose scripting runtime inside lightNVR.
- Exactly-once end-to-end delivery; consumers deduplicate using event ID.
- Uploading media to cloud storage in v1.
- Replacing the audit log with MQTT.

## 5. Event contract

Use a CloudEvents-inspired JSON envelope:

```json
{
  "specversion": "1.0",
  "id": "01K...",
  "type": "io.lightnvr.detection.object.v1",
  "source": "urn:lightnvr:installation-uuid",
  "subject": "camera/camera-uuid",
  "time": "2026-08-22T14:23:18Z",
  "datacontenttype": "application/json",
  "data": {}
}
```

Required invariants:

- `source + id` is unique and stable across retries.
- Event type includes a schema major version.
- `subject` uses Fleet 01 UUIDs, never mutable names as identity.
- Timestamps are UTC with explicit offset.
- Unknown data fields are ignored by consumers; breaking changes increment type
  version.
- Secrets and raw filesystem paths never appear in payloads.

Initial event families:

- Detection: motion asserted/cleared, object observed, zone entered.
- Camera: online/offline, authentication failed/recovered, capability changed.
- Stream: degraded/recovered, stale frames, recording started/stopped/gap.
- Storage: pressure, target unavailable/recovered, policy unmet, migration failed.
- System/security: startup/shutdown, update outcome, repeated login failure.
- ONVIF: normalized device events, digital input, relay or tamper state.

## 6. Requirements

### 6.1 Internal event bus

- C API accepts a typed event without performing network I/O on the caller thread.
- Validate required envelope fields and size limits before enqueueing.
- Critical operational events enter a persistent SQLite outbox transactionally
  where practical; high-volume observations may use explicit sampling/coalescing.
- Assign severity, sensitivity, default expiry, and media-reference policy per type.
- Prevent an event feedback loop when route-delivery health emits events.

### 6.2 Routes and filters

- A route binds enabled event types, Fleet 01 camera selector, event predicates,
  schedule, suppression settings, and one destination.
- Detection predicates include label, confidence threshold, and zone.
- Suppression includes debounce, cooldown, grouping window, and maximum rate.
- Preview evaluates a route against stored sample events without publishing.
- Route changes and manual retries are audited through Fleet 02 when available.

### 6.3 MQTT destinations

- Support multiple broker profiles with host, port, client ID, TLS mode, CA,
  client certificate where applicable, credentials, QoS, and topic template.
- Credentials use the existing secret-storage conventions and are never returned
  after save.
- Suggested topic: `lightnvr/v1/events/{type}/{camera_uuid}`; retained state uses
  separate state topics and transient events are not retained by default.
- Provide connect test, publish test, current state, last success/error, queue
  depth, and reconnect counters.
- MQTT 5 properties may carry content type, expiry, correlation ID, and schema
  metadata; remain compatible with MQTT 3.1.1 brokers where feasible.

### 6.4 Delivery and outbox

- Persist event envelope, destination, attempt count, next attempt, expiry, and
  final state.
- Retry with bounded exponential backoff and jitter.
- Expired or permanently failed deliveries move to a bounded dead-letter view.
- Queue capacity has explicit byte and row limits plus documented shedding order.
- Recording and detection threads never wait for a broker connection.
- Restart resumes eligible queued deliveries without changing event IDs.

### 6.5 Media references

- Event payloads may include metadata for a snapshot or logical clip.
- URLs must be authenticated or short-lived and bound to appropriate Fleet 02
  permissions when that PRD lands.
- Optional binary snapshot publishing is a separate opt-in route action with size
  limits; it is not embedded as base64 in every event.

### 6.6 Events & Routes UI

- Event catalog displays schema, sample payload, sensitivity, and expected rate.
- Destination editor includes validation and safe connection testing.
- Route builder uses camera scope, event type, predicates, schedule, and cooldown.
- Delivery dashboard shows broker state, queue depth, failures, dead letters, and
  end-to-end test results.

## 7. Phasing

| Phase | Scope |
| --- | --- |
| P0 | Envelope, event registry, async in-process bus, compatibility adapter for current detections |
| P1 | SQLite outbox, one MQTT destination, retry/expiry metrics |
| P2 | Multiple destinations, selector routes, suppression rules, UI |
| P3 | Broader health/storage/security/ONVIF event producers and media references |

P2 is split so configuration can be reviewed independently from delivery
behavior. Its first slice persists and validates route drafts, exposes the event
catalog and authorized CRUD API, audits mutations, and previews current camera
scope without publishing. The runtime slice compiles enabled routes off the
producer thread and enforces event type, Fleet selector, detection predicate,
and occurrence-time schedule before durable enqueue. The next slices persist
and enforce suppression state, add destination profiles, and then expose the
workflow in the Streams-area UI.

## 8. Acceptance criteria

- Detection, camera-offline, recording-gap, and storage-pressure fixtures validate
  against documented schemas.
- A broker outage during 1,000 generated critical events does not block recording;
  eligible events deliver after reconnection with unchanged IDs.
- Duplicate delivery is safely recognizable by `source + id`.
- A route scoped to one location never publishes another location's camera event.
- Cooldown prevents an unstable camera from flooding downstream consumers.
- No supported configuration path can add a native email/SMS provider to core;
  such delivery remains an external subscriber responsibility.

## 9. Risks and mitigations

| Risk | Mitigation |
| --- | --- |
| Event storms exhaust disk | Rate limits, coalescing, expiry, bounded outbox |
| Schema churn breaks cloud consumers | Registry, fixtures, major-versioned types |
| Snapshot URLs leak access | Short-lived authorization-aware references |
| MQTT failure impacts recording | Strict thread isolation and bounded enqueue |

## 10. Dependencies

- Fleet 01 for camera UUIDs and selectors.
- Fleet 02 for configuration authorization and audit; P0 may begin before its UI.
- Storage 01 and ONVIF PRDs add producers after this foundation exists.
