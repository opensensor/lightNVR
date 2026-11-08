# PRD — Event Evidence and YA-WAMF Integration

**Status**: Draft — P0 contract frozen in [OBSERVATIONS_CONTRACT.md](../integrations/OBSERVATIONS_CONTRACT.md) (C1); implementation not started
**Created**: 2026-11-08
**Owner**: TBD — lightNVR integration owner; YA-WAMF counterpart to be identified
**Scope**: Extend lightNVR with reusable observation, evidence, and classification
contracts, then refactor YetAnother-WhosAtMyFeeder (YA-WAMF) to consume those
contracts alongside its existing Frigate integration.
**Baseline**: lightNVR `0ff0d2e3`; YA-WAMF development commit
`53540f485a238e615246d2239cffe23d725903ad`. Published YA-WAMF releases may differ.

## 1. Product decision

A feeder-camera owner should be able to use lightNVR for recording and initial
bird detection, and YA-WAMF for species identification, photographs, visit
history, and notifications. Choosing lightNVR should not require rebuilding
YA-WAMF's specialist application inside the NVR.

Extend lightNVR where a capability benefits external consumers generally:
durable observations, correctly paired evidence, recoverable history, and
classification feedback. Refactor YA-WAMF where its current assumptions are
specific to Frigate: event parsing, media retrieval, source identity, lifecycle,
and feature availability.

The first integration must work with individual observations. It must not
require lightNVR to manufacture Frigate-style tracked-object visits or add a new
object tracker. YA-WAMF already groups detections into visits; its grouping
remains presentation logic and must not imply that separate photographs show
the same physical bird.

```text
Camera -> lightNVR object detector -> durable observation + matching photograph
                                  -> existing event bus / MQTT
                                             |
                                             v
                                  YA-WAMF lightNVR provider
                                             |
                                  classification / visits / notifications
                                             |
                                  optional classification write-back
                                             v
                                  lightNVR observation annotations
```

## 2. Goals and boundaries

### Goals

- Deliver the first useful outcome: a lightNVR bird observation becomes a
  species-labelled photograph in YA-WAMF, linked to the original observation.
- Recover retained observations after subscriber downtime without duplicate
  detections or repeated standard notifications.
- Preserve Frigate behavior and existing YA-WAMF installations while adding a
  second source implementation.
- Keep inference, species taxonomy, bird dashboards, and notifications in
  YA-WAMF; keep camera recording and evidence authority in lightNVR.
- Establish contracts usable by another classifier without YA-WAMF-specific
  payloads or database columns in lightNVR.

### Non-goals for the initial release

- Full Frigate API compatibility, a Frigate emulation service, or an MQTT-only
  topic-renaming bridge.
- A new lightNVR object tracker, individual-bird identity recognition, or
  synthetic `new/update/end` messages for untracked observations.
- Porting YA-WAMF's models, dashboard, BirdNET integration, or notification
  providers into lightNVR.
- Generic plugin execution, arbitrary scripts, or remote model execution in the
  lightNVR process.
- Simultaneous multi-NVR ingestion within one YA-WAMF deployment. Identity must
  permit it later, but initial configuration selects one source.
- Video analysis, best-frame search across recordings, or full-visit clips in
  the first snapshot-based integration.

## 3. Existing foundation and gaps

| Area | Current foundation | Required change |
| --- | --- | --- |
| Detection | Local and external object engines; labels, confidence, normalized boxes, optional track IDs | An eligible detector must identify `bird`; motion-only input does not satisfy this flow |
| Events | Versioned envelopes, camera UUIDs, routing, durable MQTT outbox | Persist an observation identity distinct from a delivery-message identity |
| Images | Legacy MQTT worker captures go2rtc JPEGs and serves authenticated snapshot URLs | Retain evidence from the actual inference input, independently of broker connectivity |
| History | Detection database and recording associations | Bounded, paginated observation history with durable recovery cursors and explicit retention limits |
| Feedback | Detection labels and existing authorization/audit infrastructure | Store external classifications separately from original detector results |
| YA-WAMF | Frigate ingestion, model inference, media cache, visits, corrections, notifications | Introduce a source boundary and capability-aware workflows |

The current normalized detection producer does not populate a snapshot reference.
The legacy snapshot path captures an image after detection, requires the default
MQTT connection, and retains at most 100 snapshots per camera. Its timestamped
image is not proof that the image and detection box came from the same frame.
The new evidence path must not depend on those assumptions.

Existing MQTT envelope IDs identify individual facts and their retries. They
must not be repurposed as tracked-object identities. Existing topics and
consumers remain compatible throughout this work.

## 4. User experience

### Setup

The owner selects **lightNVR** as the source in YA-WAMF, supplies its API URL,
camera-scoped token, and MQTT connection, and selects authorized feeder cameras.
Connection testing reports API access, event connectivity, evidence access, and
available features separately. Camera discovery must not require lightNVR
administrator settings access.

Setup explains that a bird-capable object detector is required. A diagnostic
shows the latest eligible observation and whether its photograph is ready,
missing, or expired. No detection is distinguished from a transport failure.

### Normal operation

A bird appears; lightNVR saves its observation and matching photograph. YA-WAMF
retrieves the photograph, classifies it, and displays the species and source
camera in its existing Explorer. An observation without usable evidence remains
visible in diagnostics but does not trigger classification of a newer image.

When the owner enables write-back, the species and classifier confidence also
appear in lightNVR's observation detail. The original `bird` label and detector
confidence remain available. Write-back failure does not remove the YA-WAMF
detection or cause another notification.

### Recovery and limited capabilities

After an outage, YA-WAMF imports retained observations from its saved cursor.
Historical recovery is silent by default and distinguishes expired evidence
from temporary retrieval failure. Unsupported video features are disabled with
a short explanation; they do not make the source appear unhealthy.

## 5. lightNVR requirements

### R1 — Durable observation identity

Create one stable observation ID per detected object in an accepted inference
result, associated with the immutable camera UUID and installation identity.
Several objects from one frame may reference the same evidence asset. Include
an inference batch ID and object index so retries of the same accepted batch do
not create additional observations. Independently sampled frames are distinct
observations even when they contain the same bird.

The observation contains:

| Field | Contract |
| --- | --- |
| `id`, `source`, `camera_uuid` | Immutable observation, installation, and capture-time camera identities |
| `observed_at`, `created_at` | Observation time and persistence time, explicitly distinct |
| `detector` | Engine identity and available model/version provenance |
| `label`, `confidence` | Original detector result; confidence in `0..1` |
| `box` | Optional normalized coordinates with explicit format and evidence coordinate space |
| `evidence` | Asset ID, state, capture time, dimensions, transform/alignment metadata, and authenticated media reference |
| `track` | Optional source-provided ID, namespace and session; absent when tracking is unavailable |
| `recording_reference` | Optional logical association; absence must not prevent snapshot classification |
| `revision` | Monotonic revision for later evidence or classification changes |

Renaming or removing a camera must not transfer its historical observations to
another camera. A reused detector track number after restart must not join two
unrelated tracks. Do not require a recording to exist: eligible observations
also work when detection runs without recording.

### R2 — Evidence paired with inference

For local inference, retain an owned copy or safely referenced buffer of the
actual input frame before its lifetime ends. Encode and store it outside the
capture thread. For HTTP inference, retain the submitted image and the
coordinate transform used to interpret the response. Do not fetch a second
go2rtc image and treat it as the inference frame.

Save an unannotated image with sufficient metadata to map the detection box
correctly after resizing or letterboxing. Crops may be derived from that image;
the original evidence association must remain auditable. A low-resolution
inference image is acceptable for the first release and must not be described
as a higher-resolution recording image.

Evidence states are `pending`, `ready`, `unavailable`, and `expired`. Include a
bounded machine-readable reason for failure. For ONVIF or external events that
cannot supply matching media, report unavailable evidence; a separately timed
context image must never be represented as aligned detection evidence.

An evidence-ready event may be published only after the asset is readable and
its metadata is committed. Use atomic file finalization and recoverable database
state transitions; startup reconciliation must resolve interrupted writes and
clean orphaned temporary files. Commit the durable observation change and a
recoverable publication record together, then feed the existing router/outbox.
Crashing between persistence and MQTT publication must not strand the change.

### R3 — History and delivery

Keep current detection topics unchanged. Register a new versioned observation
change event through the existing event registry, routing, and outbox. Each
committed revision has one message ID preserved across delivery retries; the
payload references the stable observation ID, revision, camera, and evidence
state. Consumers deduplicate messages by `source + message_id` and saved
observations by `source + observation_id`.

History must support authorized camera selection, label and time filters,
bounded page sizes, and an opaque change cursor ordered by persistence rather
than camera time. Pending-to-ready transitions must be recoverable through this
cursor even when the original observation was read earlier. Use a bounded page
watermark so concurrent writes cannot skip or repeat work indefinitely.

Expired cursors return an explicit recovery-required response and the available
history boundary. A suppressed MQTT route is not a deletion or authorization
decision: API recovery uses the consumer's selected cameras/labels and current
permissions. Changes caused only by classification feedback must not launch
classification again.

### R4 — Proposed API surface

These paths are proposed contracts, to be finalized in P0 and represented in
the canonical OpenAPI document before implementation. Use a distinct observation
resource rather than changing existing `/api/events/catalog` semantics.

| Operation | Purpose |
| --- | --- |
| `GET /api/integrations/capabilities` | Authorized source identity, contract versions, supported operations and limits |
| `GET /api/observations` | Filtered history/change recovery with pagination and cursor |
| `GET /api/observations/{id}` | Current observation, evidence state and classifications |
| `GET /api/observations/{id}/snapshot` | Authenticated matching image; no substitution with a latest frame |
| `PUT /api/observations/{id}/classifications/{key}` | Idempotent classifier-owned annotation, gated by separate write authority |

Reuse existing authorized camera discovery where sufficient. Pending evidence
returns a documented retryable response with bounded retry guidance; expired
evidence is distinguishable from an unknown observation for authorized callers.
Never expose existence or media through another camera's authorization scope.

### R5 — Classification feedback

Annotations store classifier identity, model/version, result label, optional
taxonomy identifier, confidence, classified-at time, input evidence ID/revision,
and whether the result is automated or owner-confirmed. Keep the schema bounded;
it must support general object classification without bird-specific core fields.

The authenticated integration owns its annotation namespace. The same key and
payload are a no-op on retry; conflicting updates require an expected revision.
Reject mismatched evidence references. An integration cannot claim another
writer's identity or overwrite an owner's correction. Preserve the original
detector output and expose automated results and owner decisions separately.

P2 adds a minimal classification display to existing detection/observation
detail, including species, confidence, source, and a link back to the
observation. A dedicated bird dashboard or taxonomy browser is out of scope.

### R6 — Authorization, storage and resource limits

All reads and writes resolve capture-time camera identity through Fleet 02.
Metadata reads reuse the appropriate existing camera-view authority; image
retrieval follows `snapshot.create`. P0 must reconcile these mappings with the
current API contract. Add a narrowly scoped classification-write action if no
existing action fits; do not require administrator access or reuse camera
configuration permission for annotation writes. Audit mutations without copying
credentials or image bytes into logs.

Use per-camera opt-in, age and byte limits, plus bounded worker queues and
concurrency. Reuse existing storage primitives where applicable; evidence files
must not inherit the legacy fixed-count snapshot policy. Assets shared by several
observations are deleted only when no retained reference or protection requires
them. Recording deletion and evidence deletion are separately explicit.

Retention may expire media while keeping observation metadata; advertise the
actual remaining availability rather than guaranteeing indefinite replay.
Broker downtime, slow consumers, and classifiers must not retain media forever.
If a protected asset cannot be evicted and the budget is exhausted, reject new
evidence work with a visible reason. Never block recording to wait for disk,
network, inference feedback, or queue space.

Report accepted/dropped observations, evidence backlog/failures, retained bytes,
oldest recoverable cursor, and publication failures through existing diagnostics.
Deployments with this feature disabled should not allocate its frame-copy queue
or perform its image writes.

## 6. YA-WAMF refactor requirements

### T1 — Introduce an NVR source boundary

Extract a small provider interface for source identity, authorized cameras,
capabilities, observation ingestion/history, media retrieval, and optional
classification write-back. Keep existing Frigate behavior in a Frigate provider;
implement a lightNVR provider using R1–R6.

The common input represents an observation plus optional lifecycle metadata.
Frigate retains its tracked-event start/update/end semantics and final-frame
refinement. lightNVR supplies independent observations and evidence-readiness
updates. The common pipeline must not invent an `end` message to make an
untracked observation classifiable.

Refactor the working ingestion and media paths first. Avoid a general plugin
framework or a wholesale rename of every historical Frigate identifier.

### T2 — Migrate source identity safely

Add source kind, source instance, and external observation ID to the identity
used by detections, cached media, durable jobs, favourites, and corrections.
Scope duplicate detection to that tuple, including existing rows.

Backfill legacy rows as belonging to the configured Frigate source through an
explicit migration. Preserve their IDs, photographs, favourites, corrections,
and queued work. Version the database migration and document backup/rollback
requirements; an older binary must not silently operate on incompatible data.
Changing the configured NVR must not reassign previous history to the new source.

### T3 — Remove Frigate assumptions from shared workflows

| Current area | Required treatment |
| --- | --- |
| `mqtt_service.py`, `event_processor.py` | Provider-specific parsing; common admission, idempotency and classification; lifecycle hooks only where supplied |
| `frigate_client.py` | Retain as Frigate implementation behind the provider interface |
| `routers/proxy.py`, `media_cache.py` | Resolve media by source identity; add neutral routes while preserving existing Frigate URLs |
| `backfill_service.py`, missing-event reconciliation | Provider history/cursor handling; distinguish missing, expired, unsupported and temporarily unreachable |
| HQ snapshot, recording snapshot and full-visit services | Invoke only for supported capabilities; preserve Frigate's current behavior |
| Notification orchestration and favourites | Build links and fetch evidence through the provider; preserve cached media and owner decisions |
| Settings, onboarding and health UI | Source selector, scoped connection test and capability-specific controls |

Audit direct Frigate HTTP calls beyond the central client, including background
jobs and notification attachments. A lightNVR configuration must never make an
accidental `/api/events/{id}/...` request because a shared workflow bypassed the
provider interface.

### T4 — Capability-aware classification and recovery

The first lightNVR provider advertises snapshots and history, with optional
write-back when P2 is available. Tracking lifecycle, recorded-frame selection,
event clips and recording-range clips remain independently declared features.
Unsupported features are not inferred from an endpoint returning an error.

Classify only ready evidence. Duplicate deliveries, evidence retries and stale
revisions must not produce duplicate detections. Multiple objects in one frame
must be classified using their own aligned regions. Use bounded admission and
existing visit-grouping controls to handle repeated observations without
claiming that temporal grouping establishes object identity.

Advance the recovery cursor only after work is durably accepted or explicitly
recorded as skipped with a reason. Persist retryable failures. Live/recovery
overlap shares the same deduplication identity. Record notification intent
durably so event replay does not schedule it again; external delivery ambiguity
must remain visible rather than promising exactly-once notifications.

## 7. Delivery phases

| Phase | Deliverable | Exit gate |
| --- | --- | --- |
| P0 — Contract and fixtures | Freeze observation/evidence schemas, API errors, permission mappings, budgets, provider capabilities and supported source versions; capture Frigate regression fixtures | Both implementations can consume the agreed fixtures; current Frigate behavior has a reproducible baseline |
| P1 — Snapshot integration | lightNVR durable observations, paired evidence, history and MQTT changes; YA-WAMF provider boundary, migration, lightNVR setup and classification | One feeder camera produces labelled photographs; disconnect/restart recovery and Frigate regression checks pass |
| P2 — Classification feedback | Scoped annotation API, YA-WAMF write-back, correction precedence and minimal lightNVR detail display | Species appears in both products; retry, authorization and correction tests pass |
| P3 — Optional recorded media | Event/range media retrieval and recorded-frame refinement only where reusable lightNVR primitives support them | Correct time/box alignment, partial coverage, retention and permissions pass dedicated media acceptance |

P0 precedes changes to either runtime contract. Within P1, extract the Frigate
provider while preserving behavior, implement lightNVR evidence/history, then
connect the lightNVR provider and run the end-to-end suite. P1 is a useful
standalone release; P2 and P3 must not block its snapshot workflow.

The target is upstreamable YA-WAMF changes. An upstream merge is an external
dependency, not an acceptance assumption. A maintained branch may host the
prototype; production release requires either upstream acceptance or a named
owner for maintaining that branch and its version compatibility. No unmodified
YA-WAMF compatibility claim is made.

## 8. Acceptance criteria

| ID | Scenario and required result | Gate |
| --- | --- | --- |
| A1 | On a declared bird-capable detector and recorded input fixture, every admitted observation has a stable source/camera identity and the expected object label/region | P1 |
| A2 | A moving-bird fixture changes position between inference and later frames; the saved photograph and region still match the inference input. Repeat for local and HTTP detectors | P1 |
| A3 | Two birds in one frame create separate observation identities with correctly aligned regions; shared evidence is retained until all applicable references expire | P1 |
| A4 | Replaying each MQTT message ten times and overlapping API recovery creates one YA-WAMF detection per source observation and one standard notification intent | P1 |
| A5 | Stop the broker, create 100 admitted observations, restart both applications and reconnect within retention: all 100 are recoverable, including pending-to-ready transitions | P1 |
| A6 | Interrupt evidence writes and publication at each persistence boundary: no ready reference points at an unfinished file, and committed changes can be recovered after restart | P1 |
| A7 | A camera-scoped token cannot list, fetch or annotate another camera's data. Rename, delete and name-reuse fixtures never transfer historical access or identity | P1/P2 |
| A8 | Expired media, expired cursors, unavailable evidence and temporary errors produce distinct documented outcomes; recovery never replaces evidence with a current frame | P1 |
| A9 | A snapshot-only source completes setup and classification; unsupported clip/refinement jobs do not run and controls explain their unavailability | P1 |
| A10 | Existing Frigate fixtures preserve initial classification, update recovery, end refinement, false-positive handling, corrections, favourites, backfill and clips across migration | P1 |
| A11 | Saturate evidence queues and storage budgets under a repeatable recording workload: limits hold, drops have reasons, recording remains healthy and producer enqueue never waits for I/O | P1 |
| A12 | Repeat write-back, send stale revisions and attempt to overwrite an owner correction: one authorized annotation is maintained, conflicts are explicit and detector output stays intact | P2 |
| A13 | Classification updates do not trigger another classification/notification cycle; feedback outage leaves the YA-WAMF result usable | P2 |
| A14 | Operator installs from the documented recipe using scoped credentials, sees a real feeder visit, exercises reconnect recovery, and records the tested source/model versions | P1 pilot |

P0 must record the performance fixture's hardware, stream count, resolutions,
frame rates, detector cadence and retention budgets. Compare integration enabled
and disabled on identical input. The proposed release threshold is no additional
recording gaps and no more than a 5% increase in p95 recording-path latency;
measure memory, CPU, disk use and queue high-water marks as well. If the threshold
cannot be met, reduce admitted evidence work or revise the design before release.
Classification latency is measured separately from NVR capture latency.

Species accuracy depends on detector/model/image quality. Integration acceptance
checks the expected result on pinned model fixtures and records real-camera
outcomes; it does not promise a universal species-accuracy percentage.

## 9. Rollout and compatibility

- Ship lightNVR evidence collection disabled by default; enable it for selected
  cameras with explicit storage and admission budgets.
- Preserve existing detection APIs, MQTT topics, Home Assistant behavior and
  legacy snapshot consumers. Add contract fixtures to prevent regressions.
- Keep Frigate as the default for existing YA-WAMF configurations. Source
  changes require explicit configuration; failures never switch providers.
- Publish a supported-version matrix and a minimal deployment recipe covering
  API permissions, broker routes, bird detection, retention and diagnostics.
- Rollback disables the new ingestion/provider path without deleting retained
  evidence or annotations. Database rollback follows the documented migration
  strategy; it is not assumed safe merely because a feature flag is off.
- P3 must reuse recording/storage authority and report partial or unavailable
  coverage. It must not fabricate a full-visit clip from incomplete recordings.

## 10. Risks and decisions to close

| Risk or decision | Required resolution |
| --- | --- |
| No persistent tracks from a selected detector | Support observations first; keep visit grouping in YA-WAMF and document its limits |
| Excess classifications from frequent frames | P0 sets admission budgets; YA-WAMF uses bounded work queues and visit controls, with visible skip reasons |
| Evidence expires before recovery | Expose actual retention boundaries, cache admitted photographs in YA-WAMF, and report unrecoverable history |
| Frigate dependencies remain outside the provider | Audit proxy, background, health, notification and archive paths; test that lightNVR invokes only supported operations |
| Native inference buffer ownership is unsafe | Require owned/ref-counted evidence and exercise shutdown, restart and queue-saturation paths |
| Upstream YA-WAMF declines the integration | Identify a maintenance owner before production release; do not substitute broad Frigate emulation by default |
| Annotation authority and correction precedence | Freeze action mappings and writer/owner rules in P0 before exposing mutation endpoints |
| Resource defaults are unmeasured | P0 chooses explicit bounded defaults from the declared performance fixture; defaults are a release prerequisite |

## 11. Dependencies and references

- [Fleet 01 — Camera Identity](FLEET_01_Camera_Identity_Organization.md),
  [Fleet 02 — Scoped Authorization](FLEET_02_Scoped_Authorization_Audit.md), and
  [Fleet 03 — Event Bus & MQTT Routes](FLEET_03_Event_Bus_MQTT_Routes.md).
- [Engineering API contracts](ENG_01_Safety_Contracts_TypeScript.md),
  [event contract](../EVENT_CONTRACT.md),
  [MQTT integration](../MQTT_INTEGRATION.md), and
  [detection-engine boundaries](../DETECTION_ENGINES.md).
- Current lightNVR [observation producer](../../src/core/event_producers.c),
  [legacy MQTT snapshots](../../src/core/mqtt_client.c), and
  [snapshot retrieval](../../src/web/api_handlers_detection_results.c).
- YA-WAMF [integration contract](https://github.com/Jellman86/YetAnother-WhosAtMyFeeder/blob/53540f485a238e615246d2239cffe23d725903ad/docs/integrations/frigate.md),
  [event processing](https://github.com/Jellman86/YetAnother-WhosAtMyFeeder/blob/53540f485a238e615246d2239cffe23d725903ad/backend/app/services/event_processor.py),
  [Frigate client](https://github.com/Jellman86/YetAnother-WhosAtMyFeeder/blob/53540f485a238e615246d2239cffe23d725903ad/backend/app/services/frigate_client.py), and
  [visit grouping](https://github.com/Jellman86/YetAnother-WhosAtMyFeeder/blob/53540f485a238e615246d2239cffe23d725903ad/backend/app/services/visit_grouping.py).
