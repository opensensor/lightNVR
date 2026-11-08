# Plan: INT_01 — Event Evidence & YA-WAMF Integration (P0 + P1 + P2)

**Generated**: 2026-11-08
**Source PRD**: `docs/prd/INT_01_Event_Enrichment_YAWAMF.md`
**Baselines**: lightNVR `0ff0d2e3` (this repo); YA-WAMF `53540f48` (v2.23.1),
cloned at `/home/matteius/opensensor/YetAnother-WhosAtMyFeeder`.
**Out of scope**: P3 (recorded-media retrieval / recorded-frame refinement).

## Overview

Two codebases, three phases:

- **P0 — Contract** (both repos consume it): freeze observation / evidence /
  change-event / classification schemas, API errors, permission mappings,
  budgets and capability names. Put them in the canonical OpenAPI document and
  in shared JSON fixtures, and capture a Frigate regression baseline in YA-WAMF.
- **P1 — Snapshot integration**:
  - lightNVR adds durable observations, evidence paired with the inference
    frame, a transactional publication log feeding the existing
    router/outbox, cursor-based history, and snapshot APIs.
  - YA-WAMF extracts a source-provider boundary, migrates source identity, and
    adds a lightNVR provider with cursor recovery.
- **P2 — Classification feedback**:
  - lightNVR adds a scoped annotation API with a new authz action and a
    minimal UI display.
  - YA-WAMF adds durable write-back with owner-correction precedence.

### Key design decisions (adopted unless the reviewer or owner objects)

1. **Table naming.** `camera_observations` already exists for camera
   first/last-video tracking (`src/database/db_camera_observations.c`), and
   OPS_02 already uses the "evidence" vocabulary (`evidence.protect`,
   `getEvidence*` operation IDs). New names:
   - Tables: `object_observations`, `observation_assets`,
     `observation_asset_refs`, `observation_changes`,
     `observation_publications`, `observation_classifications`.
   - OpenAPI operation IDs: `*Observation*`, never `*Evidence*`.
2. **Transactional publication.** `event_bus_publish` is in memory, so a crash
   after the DB write loses the event. We stage it instead:
   - Each observation revision writes its `observation_changes` row and an
     `observation_publications` row in **the same SQLite transaction**.
   - A relay thread turns pending publications into envelopes and calls the
     router → outbox path. `durably_enqueue_event()` is `static` in
     `event_producers.c:91`, so L5 exports a wrapper
     `event_producer_enqueue_durable(const event_envelope_t*)` that returns
     `enqueued | duplicate | no_route | mqtt_disabled | failed`.
   - The relay marks a publication relayed only after the outbox accepts it.
     `no_route` and `mqtt_disabled` are recorded as `relayed_no_route`, and the
     cursor API is the recovery path for those. After the outbox, call
     `event_bus_publish`, as the system-health producer does.
   - Delivery is **at least once**. The outbox prunes terminal rows
     (`db_event_outbox_prune_terminal`), so a late retry may redeliver.
     Consumers dedupe on `source + message_id`.
   - At startup the relay drains anything still pending.
   - **Locking.** `begin_transaction()` holds the global non-recursive DB
     mutex until commit. No existing `db_*` helper that locks the mutex
     (for example `db_set_system_setting`) may be called inside these
     transactions.
     - Follow the `db_auth.c` pattern: lock the mutex, `BEGIN IMMEDIATE`, run
       raw `sqlite3` statements, commit or roll back, unlock.
     - Keep transactions short. The mutex is shared with recording-path
       writes, and X2 measures its hold time.
   - **Startup order** (`main.c`): run L4 reconciliation first, then start the
     L3 evidence worker, then start the L5 relay drain, then start the
     detection threads. Shut down in reverse.
3. **Message ID.** The message ID is a deterministic UUIDv5 of
   `(source, observation_id, revision)`. Retries and relay restarts reuse it;
   the outbox dedups on source + event ID + destination.
4. **Cursor.** The cursor is the monotonic `observation_changes.seq`
   (`INTEGER PRIMARY KEY AUTOINCREMENT`), ordered by persistence, never by
   camera time.
   - It is encoded opaquely (base64url of `{v, seq, epoch}`).
     - `epoch` is a **DB-generation UUID** stored in a new
       `observation_meta` table. It is created with the table and regenerated
       on restore or import. It is not `event_installation_uuid`, which
       survives a DB restore while `seq` rewinds.
   - A page reads up to a `watermark = MAX(seq)` snapshot taken at query
     start, so concurrent writes cannot cause skips or endless repeats.
   - `limit` counts **observations** after deduplicating to the latest
     revision within the page.
   - `has_more` is true when more changes with `seq > page_last_seq` exist at
     or below the watermark. `next_cursor` is always the last *change* seq
     scanned, not the last observation returned.
   - These cursors return `410` +
     `{code:"cursor_expired", oldest_cursor, oldest_observed_at}`:
     - a cursor older than the prune boundary (`observation_meta.oldest_seq`);
     - an epoch mismatch;
     - `seq > MAX(seq)`.
5. **Evidence format.** Evidence is a JPEG encoded off-thread (decision from
   2026-11-08):
   - Local inference copies the frame into a bounded, ref-counted queue
     (an owned RGB24 buffer; no retained `AVFrame`).
   - A worker encodes it with the existing
     `jpeg_encoder_cache_encode_to_memory` (`src/video/ffmpeg_utils.c:702`,
     an RGB24 input path). There is no new mjpeg helper. It writes to
     `<storage>/observations/<camera_uuid>/<yyyy-mm-dd>/<asset_uuid>.jpg.tmp`.
   - The worker then fsyncs, renames to the final name, and fsyncs the
     directory.
   - Only after that does it commit `pending→ready` in the same transaction as
     the change and publication rows.
6. **Box coordinate space.** Boxes are normalized `xywh` relative to the
   **stored evidence image**. The `transform` records `source_width/height`,
   `inference_width/height`, `scale_x/y`, `pad_x/y` and `mode`
   (`none|resize|letterbox`) so boxes stay auditable.
   - SOD pre-resize (`src/video/sod_detection.c:328-365`) and API JPEG
     submission each record their own transform.
7. **Permissions** (P0 confirms these):
   - `GET /api/observations*` metadata uses `live.view`, the same as
     `/api/detection/results`.
   - `…/snapshot` uses `snapshot.create`.
   - `GET /api/integrations/capabilities` requires an authenticated principal
     and returns only that principal's scope.
   - P2 adds `observations.annotate` (bit_index 19, camera-scoped,
     non-destructive). 0078 uses bits 15–18. Built-in Administrator gets it;
     nothing else gets it by default.
   - The action is mirrored in C: catalog order defines the bit
     (`authorization.c:22`), and the OpenAPI action enums list it too.
8. **YA-WAMF identity.**
   - `detections.frigate_event` is `UNIQUE NOT NULL`; the PK is `id`.
     Foreign keys reference that unique column, and so does
     `processing_job_state.event_id`.
   - Keep `frigate_event` as the internal event key, which avoids the
     wholesale rename the PRD forbids. lightNVR rows get
     `lnvr:<source>:<observation_id>`. It cannot collide with Frigate IDs,
     which are `<ts>.<rand>-<suffix>`.
   - New **nullable or server-defaulted** columns: `source_kind`,
     `source_instance`, `external_observation_id`, `source_camera_id`.
     - `source_camera_id` holds the lightNVR camera UUID. `camera_name` stays
       for display, and identity uses the UUID (A7 rename and name reuse).
     - Add a partial `UNIQUE(source_kind, source_instance,
       external_observation_id) WHERE source_kind IS NOT NULL`.
   - Legacy rows are backfilled to `('frigate', <configured frigate instance>,
     frigate_event)` in the migration. The new binary also lazily backfills
     NULL-source rows at startup, because old binaries can still insert.
     YA-WAMF's `_verify_schema` deliberately lets older builds run against a
     newer DB and only warns, so "older binary refuses" cannot be enforced
     for already-shipped binaries.
   - **Multi-bird mapping**: one lightNVR observation maps to one YA-WAMF
     detection row, with one `bird_observations` row at `bird_index` 0.
     - Several objects in one frame become sibling detections sharing an
       evidence asset ID, and visit grouping places them in the same visit.
     - Frigate keeps its current multi-bird-per-event model.
     - W4 and W9 must check visit counts, leaderboard counts and multi-bird
       display for siblings.
   - **Notification policy for sources without lifecycle**: the existing
     `notify_mode="final"` fires only on `event_type=="end"`, which lightNVR
     never sends.
     - For sources without `track_lifecycle`, "final" behaves as notify on
       first admitted ready classification, bounded by the existing visit
       cooldown.
     - W7 reuses or reconciles `mark_notified` rather than adding a parallel
       flag.
   - **Durable jobs**: reuse `processing_job_state(pipeline, event_id)` for
     write-back and retryable recovery work. Do not add new job tables.
9. **Migration numbering.**
   - lightNVR P1 uses `0095`; P2 uses `0096`. Main is at `0094`; re-check
     before each merge.
   - YA-WAMF P1 is one Alembic revision (source identity + `source_cursors`,
     plus any notification-intent columns W7 needs) to keep a single head. P2
     write-back reuses `processing_job_state`, so it should need no revision.
   - Tasks touching the same migration stream are serialized.

### Constraints carried from project memory

- Small PRs branched from `main`. Every task below is a PR-sized unit.
- `rust/` is generated from C by CI. Never hand-edit it. New C files only need
  CMake registration.
- Local `/usr/local` FFmpeg headers may be stale, which can cause false
  crashes. Run the mjpeg/evidence tests in the project Docker/CI toolchain if
  local results look wrong.
- Audio is untouched. If any shared settings or UI work touches audio paths,
  test with `audio_disabled` on.

## Prerequisites

- lightNVR toolchain: CMake build, `tests/unit` (Unity-style C tests),
  Playwright contract suite (`tests/contracts`, `node:sqlite` seeding),
  `scripts/contracts/generate.mjs`, `scripts/generate_embedded_migrations.py`.
- YA-WAMF toolchain: Python 3, `pytest` (`backend/tests`, pure lane
  `backend/tests/pure`), Alembic 1.20.1 (`render_as_batch=True` on SQLite),
  FastAPI 0.143.2, aiomqtt 2.5.1, httpx 0.28.1, Svelte 5 + Vite 8 UI
  (`npm run check`, `npm test`).
- A YA-WAMF fork branch (`feature/lightnvr-provider`) and a named maintenance
  owner before production release (PRD §7).
- An E2E harness host with Docker (mosquitto, lightNVR, YA-WAMF) and a recorded
  bird fixture video (two-bird and moving-bird clips).
- Context7 was unavailable. The APIs above are pinned from the repos'
  requirement files. Re-verify these against the pinned versions while
  implementing:
  - Alembic batch-mode FK behavior.
  - aiomqtt 2.x subscription and reconnect semantics.
  - The RGB24 input behaviour of `jpeg_encoder_cache_encode_to_memory` at
    detection-stream resolutions.

## Dependency Graph

```
P0  C1 (contract ADR) ─┬─ C2 (OpenAPI + codegen) ─────────────────────────┐
    C4 (perf baseline)  ├─ C3 (shared JSON fixtures) ─────────────┐        │
    W0 (Frigate regression fixtures)                              │        │
                                                                  │        │
P1  lightNVR:  C1 ── L1 (mig 0095) ── L2 (db layer) ─┬─ L3 (evidence queue/worker) ─┬─ L6 (local detector)
                                     │                ├─ L4 (reconcile/retention)    ├─ L7 (HTTP detector + ONVIF unavailable)
                                     │                ├─ L5 (publication relay + event type)
                                     │                └─ L8 (observation API) ◄── C2
                     L2+C2 ── L9 (per-camera opt-in/budgets API+UI)
                     L3,L4,L5,L8 ── L10 (diagnostics)
                     L6 ── L7 (serialized: shared signatures)   C3 ── L5, L7
                     L5..L10 ── L11 (contract+reliability tests) ── L12 (docs+recipe)

    YA-WAMF:   W0 ─┬─ W1 (provider boundary + Frigate provider) ── W3 (route all Frigate calls via provider)
                   └─ W2 (Alembic identity/cursor migration)
               W1+W2+W3+C3 ── W4 (common ingestion pipeline)
               W1+C3 ── W5 (lightNVR client/provider)
               W2+W4+W5 ── W6 (cursor recovery)     W2+W4 ── W7 (durable notification intent)
               W3+W5 ── W8 (settings/onboarding/health UI)
               W3,W6,W7,W8 ── W9 (YA-WAMF acceptance tests)

    X:  L11+W9 ── X1 (E2E P1 suite + A14 pilot recipe)
        C4+L6+L7+L10 ── X2 (A11 saturation + perf gate)

P2  L2 ── L13 (mig 0096 + db classifications + authz action)
    L13+L8+L5 ── L14 (PUT annotation API) ── L15 (minimal UI display)
    W4+W5+W7+C3 ── W10 (YA-WAMF write-back + correction precedence)
    X1+L14+L15+W10 ── X3 (P2 E2E: A12, A13, A7-annotate)
```

## Tasks

### P0 — Contract and fixtures

#### C1: Freeze the integration contract (ADR)
- **depends_on**: []
- **location**: `docs/integrations/OBSERVATIONS_CONTRACT.md` (new), `docs/EVENT_CONTRACT.md`, `docs/prd/INT_01_Event_Enrichment_YAWAMF.md` (status/links only)
- **description**: Write the normative contract that every later task implements.
  - Observation object (R1), field by field:
    - Types, nullability and bounds: `label` ≤31 chars matching
      `MAX_LABEL_LENGTH`; `confidence` 0..1; up to 20 objects per batch,
      matching `MAX_DETECTIONS`.
    - `box` format and coordinate space; the `transform` object.
    - `detector` provenance: `engine_key`, `engine_type`, `model`,
      `model_version`.
    - `track`: `{id, namespace, session}`, where `session` is a per-process
      boot UUID so a reused track ID never joins unrelated tracks.
    - `recording_reference`.
    - `revision` semantics: +1 per evidence-state or classification change.
  - Evidence asset:
    - States `pending|ready|unavailable|expired`.
    - Bounded reason enum: `queue_full`, `budget_exhausted`, `encode_failed`,
      `write_failed`, `interrupted`, `no_matching_media`, `disabled`,
      `retention_expired`, `deleted`.
    - Dimensions, `captured_at`, and the media URL template.
  - Change event `io.lightnvr.detection.observation_changed.v1`:
    - Payload: `observation_id`, `revision`, `camera_uuid`, `label`,
      `evidence_state`, and `change_cause` (`created|evidence|classification`).
      Consumers must skip `classification`-only changes for re-classification.
    - Subject: `camera/<uuid>`. Topic suffix under the existing prefix. Severity
      `info`.
  - History query parameters: `camera_uuid[]`, `label[]`, `since`/`until` on
    `observed_at`, `cursor`, and `limit` (default 100, max 500).
  - Cursor encoding, watermark rule, and the `410 cursor_expired` body.
  - Error catalog:
    - Snapshot: `202` + `Retry-After` (1–30 s) for pending; `410
      evidence_expired`; `404 evidence_unavailable` with reason; `404` for
      unknown or unauthorized observations (indistinguishable).
    - Annotation: `409 revision_conflict`, `409 owner_decision_locked`,
      `422 evidence_mismatch`.
  - Capability document: `source` identity, `contract_versions`, and
    operations (`observations.history`, `observations.snapshot`,
    `observations.annotate`, plus `track_lifecycle`, `recorded_frame`,
    `event_clip` and `range_clip` = false), with limits.
  - Classification annotation schema (R5): `key` = `<writer_namespace>/<name>`;
    `classifier`, `model_version`, `label`, `taxonomy_id?`, `confidence`,
    `classified_at`, `input_asset_id`, `input_revision`,
    `kind: automated|owner_confirmed`, and `expected_revision`.
  - Writer namespace is derived from the API token's identity, never from the
    request body.
  - Permission mapping (design decision 7) and the audit fields to record.
  - Provisional budgets: evidence off by default; per-camera queue 8 frames /
    32 MiB global; worker concurrency 1; 7 days / 2 GiB per camera; history
    retention 30 days. Record these as provisional until X2 confirms them.
  - Supported-version matrix stub.
- **validation**: A reviewer can implement every L* and W* task without
  inventing a field or status. Every PRD R1–R6 and T1–T4 bullet maps to a
  contract section (include a traceability table). The ADR is merged before L1
  and C2 start.
- **status**: Completed
- **log**:
  - Wrote the normative contract (commit `3ffc7408`). Decisions 1–9 are
    adopted; deviations D1–D11 are in contract §1. Facts were checked against
    the code: envelope validator key rules, `event_envelope_create_with_severity_and_id`,
    authz catalog order and bits 15–18 in 0078 (19 and sort_order 200 are
    free), `live.view` / `snapshot.create` on the existing detection
    endpoints, `MAX_LABEL_LENGTH` 32 / `MAX_DETECTIONS` 20, the topic format,
    and YA-WAMF `notify_mode` / `mark_notified` / `processing_job_state`.
  - `reason_not_testable`: documentation only. Static verification instead:
    a script confirmed that all 24 PRD IDs (R1–R6, T1–T4, A1–A14) appear in
    the traceability table, that 38 relative links and anchors resolve, and
    that there are no dangling § references.
  - **Gotchas for later tasks**:
    - **D1**: list filters are comma-separated, because the query getter
      returns only the first occurrence and `query_string` is 1,024 bytes.
    - **D2**: error bodies are `{error, code, …}`, so they stay valid `ApiError`.
    - **D3**: the writer namespace is the normalized username, and authority is
      pinned to the user id. It is advertised in capabilities, so W10 must not
      hard-code `yawamf`.
    - **D4**: snapshot returns `403` when `live.view` is held but
      `snapshot.create` is not.
    - **D5**: every observation has an asset row.
    - **D6 (L1)**: `observation_changes` also needs `evidence_state` and
      `evidence_reason` columns.
    - **D7 (L1/L5)**: the publication states are `pending`, `relayed`,
      `relayed_no_route`, `expired` and `failed`.
    - **D8 (L5)**: cap this type at 2,000 pending outbox rows.
    - **D9 (L4/L5/L11)**: write a `.cursor_head.json` sidecar for restore
      detection, and regenerate the epoch at both
      `restore_database_from_backup` call sites in `db_core.c`.
    - **L5** needs a UUIDv5 helper (mbedTLS SHA-1); the frozen namespace and
      a test vector are in §5.4. **L2/L8** need a base64url helper.
    - **L8** must use `httpd_evaluate_camera_identity_action` rather than the
      `httpd_authorize_*` helpers, which emit `403` and would leak existence.
    - §19 (Performance) is a placeholder for C4 to fill.
- **files edited/created**:
  - `docs/integrations/OBSERVATIONS_CONTRACT.md` (new)
  - `docs/EVENT_CONTRACT.md` (reserved-type pointer section)
  - `docs/prd/INT_01_Event_Enrichment_YAWAMF.md` (status line only)

#### C2: Add the contract to OpenAPI and regenerate projections
- **depends_on**: [C1]
- **location**: `docs/api/openapi.json`, `include/web/openapi_document_generated.h` (generated), `web/generated/*` (generated), `redocly.yaml` if lint rules need it
- **description**: Add these operations, with `x-lightnvr-authorization`
  blocks that copy the existing shape (`{version:1, kind:'camera',
  requirements:[{action, resource:'raw'}]}`):
  - `getIntegrationCapabilities`
  - `listObservations`
  - `getObservation`
  - `getObservationSnapshot`
  - `putObservationClassification` (marked P2 and enforced only once L14
    lands)

  **Coverage gate**: `tests/contracts/tooling/coverage-inventory.test.mjs`
  requires every `operationId` to map to a registration in
  `src/web/libuv_api_handlers.c`, via `docs/api/contracts/coverage.json`
  (sha256 + line numbers) and `x-lightnvr-coverage.covered_operations`.
  - C2 therefore also registers all five routes as `501 not_implemented`
    stubs in a new `src/web/api_handlers_observations.c` and regenerates
    `coverage.json`.
  - L8 and L14 replace the stubs and regenerate `coverage.json` again.

  Add schemas `Observation`, `ObservationEvidence`, `ObservationTransform`,
  `ObservationPage`, `ObservationChangeEvent`, `ObservationClassification`,
  `IntegrationCapabilities` and `ObservationError`. Run
  `node scripts/contracts/generate.mjs` and `scripts/contracts/lint.mjs`.
- **validation**: `node scripts/contracts/lint.mjs`, `generate.mjs --check`
  and `coverage-inventory.test.mjs` pass. The stub routes return `501`.
  `tests/contracts/tooling/*` passes. The generated header compiles. The existing operation-ID lists in
  `tests/contracts/*.spec.ts` stay unchanged; new IDs are not `*Evidence*`.
- **status**: Completed
- **log**:
  - Commit `a847d2b6`. Added five operations under a new `Integrations` tag
    and 20 schemas. They are the 8 required ones plus `ObservationUuid`,
    `ObservationTime`, `ObservationEvidenceState`, `ObservationEvidenceReason`,
    `ObservationBox`, `ObservationDetector`, `ObservationTrack`,
    `ObservationRecordingReference`, `ObservationClassifications`,
    `ObservationClassificationWrite` (the closed PUT body),
    `ObservationClassificationResult` and `IntegrationCapabilityOperation`.
    Ten of them are exported as web validators.
  - **PUT path**: two segments,
    `/api/observations/{id}/classifications/{namespace}/{name}`; native route
    `/api/observations/#/classifications/#/#`.
  - **Authorization**:
    - capabilities: `{version:1, kind:"authenticated"}`;
    - list and get: camera `live.view`;
    - snapshot: camera `snapshot.create`;
    - PUT: **`authenticated` for now**. `generate_native.py` rejects any
      action missing from `authorization.c`, and `observations.annotate`
      arrives in L13.
  - **Stubs**: every route calls `httpd_check_action_access`. Without a
    credential it returns `401`; otherwise it returns
    `501 {"error","code":"not_implemented"}`. Every operation documents `501`.
  - **Coverage regeneration**: no script existed (the earlier inventory was
    hand-built), so I added **`scripts/contracts/coverage.mjs`**:
    - `node scripts/contracts/coverage.mjs` refreshes line numbers, handler
      names and `source_sha256`, and keeps every reviewed field;
    - `--add-coverage <group>` also adds new registrations, each matched to
      exactly one OpenAPI operation (`{param}` becomes `#`);
    - `--check` fails on drift.

    It errors on a stale entry (route removed) and on a registration with no
    entry. On the HEAD inputs it reproduced `coverage.json` byte for byte. New
    entries use coverage group `observations`. The `notes` counts were updated
    by hand to 246 registrations and 237 operations.
  - **RED→GREEN**: `tests/contracts/tooling/observations-contract.test.mjs`
    failed 3/3 before the change (no schemas, ops, stub file or validators). It
    passes 3/3 after. `tests/unit/test_api_handlers_observations.c` first
    failed to compile (missing header) and now passes 2 tests (501 with the
    code, and 401 unauthenticated).
  - **Validation**:
    - `lint.mjs`: valid, with the same 5 pre-existing warnings as HEAD.
    - `generate.mjs --check`: OK.
    - `node --test tests/contracts/tooling/*.test.mjs`: 49/49.
    - `tsc -p tests/contracts/tsconfig.json`: clean.
    - Full `build-tls` build of `lightnvr`, plus `test_api_handlers_openapi`
      (8/8) and `test_openapi_projection` (21/21): pass.
    - Live fixture server: all 5 routes return `501` with admin credentials
      and `401` anonymously, and the admin `/api/openapi.json` projection
      lists all 5.
  - **Gotchas for later tasks**:
    - **Every** route edit in `libuv_api_handlers.c` shifts line numbers. Run
      `node scripts/contracts/coverage.mjs` afterwards, or
      `coverage-inventory.test.mjs` fails on the sha256 and line checks.
    - **L8**: keep handler names, or the script picks up a rename
      automatically. Remove the `501` response from the GET ops once they are
      implemented.
    - **L14**: change the PUT `x-lightnvr-authorization` to camera
      `observations.annotate` after L13 adds it to `authorization.c`, then
      drop the P2 note and the `501`. Update the expectation in
      `observations-contract.test.mjs` too.
    - `ObservationEvidence` and `ObservationChangeEvent` enforce the
      reason/state and `snapshot_url` consistency with `if`/`then`. The
      strict Ajv needs `properties` declared in every `required` subschema,
      or redocly warns.
    - Response schemas use `additionalProperties: true` (readers ignore
      unknown fields), and the PUT body is closed.
    - `ObservationError.code` is an enum of the §11 codes. Adding a code
      means editing OpenAPI.
    - `build/` has `BUILD_TESTS=OFF`. Use `build-tls` for unit tests.
- **files edited/created**:
  - `docs/api/openapi.json`, `docs/api/contracts/coverage.json`
  - generated: `include/web/openapi_document_generated.h`, `web/generated/api/{schema.d.ts,validators.js,validators.d.ts}`
  - `src/web/api_handlers_observations.c` (new), `include/web/api_handlers_observations.h` (new), `src/web/libuv_api_handlers.c`
  - `scripts/contracts/coverage.mjs` (new)
  - `tests/contracts/tooling/observations-contract.test.mjs` (new), `tests/unit/test_api_handlers_observations.c` (new), `tests/unit/CMakeLists.txt`

#### C3: Shared contract fixtures for both implementations
- **depends_on**: [C1]
- **location**: `tests/fixtures/integrations/observations/` (new). Vendor the same files into YA-WAMF at `backend/tests/fixtures/lightnvr/` with a `SOURCE.md` naming the lightNVR commit.
- **description**: Write canonical JSON fixtures:
  - A single-bird observation in each of the pending, ready, unavailable and
    expired states.
  - Two birds sharing one asset.
  - A letterboxed HTTP transform.
  - Change events for `created`, `evidence` and `classification`.
  - Paginated pages with cursors.
  - The `cursor_expired`, `evidence_expired` and pending-202 bodies.
  - A capabilities document.
  - Classification PUT request/response pairs, including conflict cases.

  Include a small JPEG with known box coordinates. Add a validator test that
  checks every fixture against the C2 OpenAPI schemas.
- **validation**: A new `tests/contracts/tooling/observation-fixtures.test.mjs`
  validates every fixture against `docs/api/openapi.json`. A YA-WAMF pure test
  loads the vendored copies (added in W5).
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### C4: Declare the performance fixture and capture a disabled baseline
- **depends_on**: []
- **location**: `tests/performance/observations/` (new), `docs/integrations/OBSERVATIONS_CONTRACT.md` §Performance (append after C1 merges, or hand it to the C1 author)
- **description**:
  - Record the hardware, stream count, resolutions, fps, detector cadence and
    retention budgets.
  - Build a repeatable workload: N looped RTSP fixture streams through go2rtc,
    with a local SOD detector at a fixed interval.
  - Capture a baseline with the feature absent: recording-gap count, p95
    recording-path latency (use the existing segment/packet timing metrics),
    RSS, CPU and disk write rate.
  - Script it so X2 can rerun it with the feature enabled.
- **validation**: The script runs twice on the same host with p95 variance
  under 2%. Results are committed as JSON with the host description.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W0: Capture the YA-WAMF Frigate regression baseline
- **depends_on**: []
- **location**: YA-WAMF `backend/tests/fixtures/frigate_regression/`, `backend/tests/test_frigate_regression_baseline.py` (new)
- **description**: Before any refactor, add recorded Frigate MQTT payloads and
  mocked `FrigateClient` responses. Add tests that pin current behavior:
  - initial classification on a `new` message;
  - update recovery;
  - `end` refinement (final-frame / HQ snapshot);
  - false-positive handling (`_handle_false_positive`);
  - corrections;
  - favourites;
  - backfill (`backfill_service`);
  - clip and full-visit generation calls.

  Assert the DB rows, media-cache keys, notification-dispatch calls and the
  exact Frigate HTTP paths requested.
- **validation**: `pytest backend/tests/test_frigate_regression_baseline.py`
  is green on unmodified `53540f48`. This suite is the A10 gate and must stay
  green through every W* task.
- **status**: Completed (YA-WAMF commit `c5e7fecd` on `feature/lightnvr-provider`, not pushed)
- **log**:
  - Characterization tests: they pin shipped behaviour, so there is no RED
    phase. 46 tests pass on unmodified `53540f48`. To show they bite, six
    mutations were applied to a scratch copy of `backend/app`, and five failed
    the suite on the first run:
    - live snapshot fetched with `crop=False`: 7 failures;
    - `mark_notified` dropped after dispatch: 5;
    - snapshot cache key renamed: 4;
    - false positive overriding an owner tag: 1;
    - full-visit trigger `source` changed: 1.
    - `final` mode firing on `update` survived at first, because a thumbnail
      fallback rescued the `new`. A recovery test was added, and it now
      fails 1.
  - Running it: `cd backend && python3.12 -m venv .venv && .venv/bin/pip install -r requirements-base.txt -r requirements-provider-cpu.txt -r requirements-dev.txt`,
    then `.venv/bin/pytest tests/test_frigate_regression_baseline.py`. The
    `-full` provider file pulls CUDA wheels; the CPU one is enough.
  - Full suite: 7455 passed. Twelve migration tests fail only when the venv is
    not on `PATH`, because they shell out to a bare `alembic`. With
    `PATH=.venv/bin:$PATH` they pass.
  - Harness, which W1–W4 should reuse rather than copy:
    - DB: `monkeypatch.setenv("DB_PATH", migrated_history_db)` and then the
      real `init_db()`/`close_db()`, so every module's `get_db` hits a private
      migrated copy without per-module patching.
    - Frigate: `FakeFrigate` (an `httpx.MockTransport`) is installed on
      `frigate_client._client`. The full-visit service builds its own
      `httpx.AsyncClient`, so its module-level `httpx` is swapped for a
      namespace whose `AsyncClient` uses the same transport. Assertions are
      on `METHOD path?sorted-query` strings.
    - Media cache: `SNAPSHOTS_DIR`, `CLIPS_DIR`, etc. are patched to
      `tmp_path`, and `_available` is set to True.
    - Notifications: the outbound boundary is
      `notification_service.notify_detection`, driven by a real
      `NotificationDispatcher`.
    - HQ and full-visit in-memory schedulers are stubbed, but the durable
      `processing_job_state` row is asserted.
  - Gotchas for W1–W4:
    - Live `new`/`update` requests `snapshot.jpg?crop=1&quality=95`, while a
      recovered `end` and backfill request `crop=0`.
    - The live miss chain is: cropped snapshot twice, then uncropped, then
      `thumbnail.jpg`, then the recording window
      `/api/{cam}/start/{t-2}/end/{t+8}/clip.mp4`.
    - A notification without a cached photo fetches
      `snapshot.jpg?crop=1&quality=85`.
    - The cached snapshot's `source` is `frigate_snapshot_cropped`. The HQ job
      pipeline name is `high_quality_snapshot`.
    - `detection_event_bounds` stores datetime strings.
    - In `standard` mode a recovered `end` does **not** notify; only `final`
      does. This matters for the W7 policy for sources without lifecycle.
    - Corrections and favourites make zero Frigate calls.
    - Full visit uses the timestamp in the event ID for its window and makes 2
      bounded attempts.
    - `taxonomy_service` is a shared singleton, so patch it
      input-conditionally.
    - YA-WAMF `CLAUDE.md` §10 forbids AI attribution trailers in commits, so
      the YA-WAMF commit has none. Later W* commits should follow that rule.
- **files edited/created**:
  - YA-WAMF `backend/tests/test_frigate_regression_baseline.py` (new)
  - YA-WAMF `backend/tests/fixtures/frigate_regression/mqtt_events.json` (new)
  - YA-WAMF `backend/tests/fixtures/frigate_regression/frigate_api.json` (new)

### P1 — lightNVR

#### L1: Migration 0095 — observation, asset, change and publication tables
- **depends_on**: [C1]
- **location**: `db/migrations/0095_add_object_observations.sql`, `include/database/db_embedded_migrations.h` (regenerate via `scripts/generate_embedded_migrations.py`), `src/database/db_schema.c` / `db_schema_cache.c` if they enumerate tables
- **description**: Create these tables, with `migrate:down`:
  - **`object_observations`**: `uuid` PK, `camera_uuid`, `stream_name_at_capture`,
    `batch_id`, `object_index`, `observed_at_ms`, `created_at_ms`,
    `engine_key`, `engine_type`, `model`, `model_version`, `label`,
    `confidence` with a CHECK, the box as four nullable REALs, track
    `id/namespace/session`, `recording_id` (FK ON DELETE SET NULL),
    `asset_uuid` (nullable), `revision`, `UNIQUE(batch_id, object_index)`.
    Indexes on `(camera_uuid, observed_at_ms)`, `(label, observed_at_ms)` and
    `created_at_ms`.
  - **`observation_assets`**: `uuid` PK, `camera_uuid`, `state` (CHECK in the
    four states), `reason`, `path`, `bytes`, `width`, `height`,
    `captured_at_ms`, `transform_json`, `protected`, `expires_at_ms`,
    `created_at_ms`, `updated_at_ms`.
  - **`observation_changes`**: `seq` INTEGER PK AUTOINCREMENT,
    `observation_uuid`, `revision`, `cause`, `created_at_ms`,
    `UNIQUE(observation_uuid, revision)`.
  - **`observation_publications`**: `change_seq` PK/FK, `message_id`,
    `state` (`pending|relayed|failed`), `attempts`, `last_error`,
    `updated_at_ms`.
  - **Per-camera settings**: add `observation_evidence_enabled` (default 0),
    `observation_labels` (CSV, default `''`, meaning all object labels), and
    `observation_max_age_days` / `observation_max_bytes` columns on `streams`.
  - **Global settings**: insert `system_settings` keys
    `observation_history_retention_days` and
    `observation_global_queue_bytes`.
  - **`observation_meta`**: a single row with `epoch_uuid` and `oldest_seq`
    (the prune boundary). It lives outside `system_settings` so it can be
    written inside observation transactions (design decision 2).

  Asset refcount is derived from `object_observations.asset_uuid`, so no
  separate refs table is needed. If C1 needs per-reference protection, add
  `observation_asset_refs`.
- **validation**: A migration up/down/up test in
  `tests/unit/test_db_migrations*.c`, or the existing migration test pattern,
  passes. The embedded header is regenerated and checked in. A fresh DB and an
  upgraded `0094` DB both reach the same schema.
- **status**: Completed
- **log**:
  - RED: `tests/unit/test_observation_migration.c` failed 4/4 before the
    migration existed ("0095 is not embedded", "no such table"). GREEN 4/4
    after it, in `build-tls` and `build-tls-asan`. The `migration|db_|schema|embedded`
    ctest subset (36 tests) and the generator pytest pass. The only failures in
    the full suite are the FFmpeg/SOD tests, which fail locally for unrelated
    reasons (stale headers, tests that need CLI args).
  - Run: `cd build-tls && make test_observation_migration && ./bin/test_observation_migration`.
  - Followed the contract where it says more than the plan:
    - `observation_changes` has `evidence_state` and `evidence_reason` (D6).
    - The publication state CHECK allows exactly the five D7 values.
    - Added `observation_assets.origin` and `media_type` (§4.1).
  - Exact names for L2:
    - `object_observations`: `uuid`, `camera_uuid`, `stream_name_at_capture`,
      `batch_id`, `object_index`, `observed_at_ms`, `created_at_ms`,
      `engine_key`, `engine_type`, `model`, `model_version`, `label`,
      `confidence`, `box_x`, `box_y`, `box_width`, `box_height`, `track_id`,
      `track_namespace`, `track_session`, `recording_id`, `asset_uuid`,
      `revision`.
    - `observation_assets`: `uuid`, `camera_uuid`, `state`, `reason`,
      `origin`, `media_type`, `path`, `bytes`, `width`, `height`,
      `captured_at_ms`, `transform_json`, `protected`, `expires_at_ms`,
      `created_at_ms`, `updated_at_ms`.
    - `observation_changes`: `seq`, `observation_uuid`, `revision`, `cause`,
      `evidence_state`, `evidence_reason`, `created_at_ms`.
    - `observation_publications`: `change_seq`, `message_id` (UNIQUE),
      `state`, `attempts`, `last_error`, `updated_at_ms`.
    - `observation_meta`: `id` (always 1), `epoch_uuid`, `oldest_seq`
      (initially 1).
    - `streams` columns: `observation_evidence_enabled`, `observation_labels`,
      `observation_max_age_days`, `observation_max_bytes`.
    - `system_settings` keys: `observation_history_retention_days` = 30 and
      `observation_global_queue_bytes` = 33554432.
  - Gotchas for L2/L4/L5:
    - Revision 1 must have cause `created`, and only revision 1 may have it
      (CHECK).
    - `reason` / `evidence_reason` must be NULL exactly when the state is
      `pending` or `ready`. Reason values are open text (1..32 chars), so
      callers validate the §4.3 enum.
    - The box is all four columns or none, and so is the track (all three).
    - `object_index` must be 0..19. `engine_type` excludes `motion`.
    - With `foreign_keys = ON` (set by `db_core.c`):
      - `asset_uuid` references `observation_assets`, so insert the asset
        before its observations and delete observations before the asset.
      - Deleting a change cascades to its publication. Deleting an
        observation cascades to its changes.
      - Deleting a recording sets `recording_id` to NULL (indexed).
    - The migration enforces no stream budget bounds beyond `> 0`. The
      provisional §13 bounds belong in the L9 API.
    - `migrate:down` drops all observation data and leaves evidence files on
      disk.
    - No `db_schema.c` / `db_schema_cache.c` changes were needed, because they
      enumerate no tables.
- **files edited/created**: `db/migrations/0095_add_object_observations.sql`, `include/database/db_embedded_migrations.h`, `tests/unit/test_observation_migration.c`, `tests/unit/CMakeLists.txt` (commit `7c2245a5`)

#### L2: Observation DB layer (idempotent writes, transitions, cursor queries)
- **depends_on**: [L1]
- **location**: `src/database/db_observations.c`, `include/database/db_observations.h` (new), `tests/unit/test_db_observations.c`, `tests/unit/CMakeLists.txt`, `CMakeLists.txt`. Also `include/core/config.h` (`stream_config_t` evidence fields) and `src/database/db_streams.c` (load and save of the 0095 stream columns), so L3, L4, L6 and L7 can read per-camera settings without waiting for L9.
- **description**: Implement the following. Every mutating call is one
  transaction using the `db_auth.c` pattern: mutex, `BEGIN IMMEDIATE`, raw
  statements, commit. It must call no mutex-taking helper (design
  decision 2). Each call writes the observation row, `observation_changes`
  and `observation_publications` together.
  - **`db_observation_insert_batch(batch_id, camera_uuid, objects[], asset_uuid|NULL, initial_asset_state)`**:
    - Inserts N observations plus the asset row.
    - Uses `INSERT OR IGNORE` on `(batch_id, object_index)`, so a retry is a
      no-op that returns the existing IDs.
    - Writes revision 1 with cause `created`.
  - **`db_observation_asset_transition(asset_uuid, from, to, reason, path, bytes, w, h)`**:
    - A compare-and-set on `state`.
    - Bumps the revision of every observation referencing the asset, with
      cause `evidence`, and adds their change and publication rows.
  - **`db_observations_list(query, cursor, limit, page_out)`**:
    - Joins `observation_changes` (`seq > cursor AND seq <= watermark`) and
      dedups to the latest revision per observation within the page.
    - Applies authorized `camera_uuid` set, label and time filters, and
      bounded `limit`.
    - Returns `next_cursor` and `has_more`.
  - **`db_observation_get(uuid, out)`**.
  - **`db_observation_pending_publications(limit)`** and
    **`db_observation_publication_mark(...)`**.
  - **`db_observations_prune(before_ms, batch)`**: prunes changes and
    observations, and records the prune boundary in
    `observation_meta.oldest_seq` inside the same transaction.

  Use the capture-time `camera_uuid` from the stream row at insert. Never join
  on stream name for authorization.
- **validation**: Unit tests cover:
  - a duplicate batch retry;
  - two objects sharing one asset;
  - CAS transition races (two transitions, exactly one wins);
  - a pending→ready transition appearing after the cursor already passed the
    `created` change (A5);
  - a watermark with concurrent inserts (no skip, no infinite repeat);
  - an expired cursor (below the oldest seq);
  - a filter that excludes another camera;
  - rollback on an injected failure between the change and publication
    inserts.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L3: Evidence frame queue and encoder worker
- **depends_on**: [L2]
- **location**: `src/core/observation_evidence.c`, `include/core/observation_evidence.h` (new), `tests/unit/test_observation_evidence.c`. Reuse `jpeg_encoder_cache_encode_to_memory` from `src/video/ffmpeg_utils.c`; add no new encoder.
- **description**:
  - **Producer API**:
    `observation_evidence_submit(camera_uuid, batch, owned_rgb_buffer, w, h, transform)`.
    - It is non-blocking. It takes ownership of an RGB24 buffer the caller
      already made (`frame_to_rgb24`), with no copy, or makes one bounded
      copy. It does not retain an `AVFrame`, which avoids pinning decoder or
      hwaccel surfaces.
    - The submitted item carries the observation batch, and the worker
      performs `db_observation_insert_batch`. The capture and detection
      threads never touch the DB mutex for this feature.
    - It returns `ACCEPTED`, `DROPPED_QUEUE_FULL`, `DROPPED_BUDGET` or
      `DISABLED` immediately. It never waits on I/O or locks held by the
      worker; use a try-lock or atomics for admission.
    - There is also a variant that takes already-encoded JPEG bytes (for L7),
      which skips encoding.
  - **Queue**: bounded by count and bytes, globally and per camera.
  - **Worker**:
    - Encodes the JPEG via the shared encoder (skipped for pre-encoded
      bytes) and writes `.tmp`.
    - Calls `fsync`, renames, and fsyncs the parent directory.
    - Then calls `db_observation_asset_transition(pending→ready)`.
    - On any failure it unlinks the temp file and transitions to
      `unavailable` with a reason.
  - **Producer-side drops**: when the queue is full or over budget, the
    producer pushes a small fixed-size drop record (batch metadata, no
    image) onto a separate bounded ring.
    - The worker inserts that batch directly as
      `unavailable(queue_full|budget_exhausted)`, so admitted observations
      stay visible in diagnostics.
    - If the drop ring is also full, increment a counter (`dropped_untracked`)
      and lose the observation. That is acceptable, because it is reported in
      L10.
  - **Startup order**: start only after L4 reconciliation completes (design
    decision 2).
  - **Lifecycle**: lazy start. No queue allocation or thread while every
    camera has evidence disabled (PRD R6). Integrate with
    `shutdown_coordinator.c` and drain or abandon on shutdown. Abandoned items
    become `interrupted` at reconciliation (L4).
  - **Counters**: accepted, dropped by reason, queue high-water, bytes
    written.
- **validation**:
  - Unit tests: queue saturation returns a drop without blocking (time-bound
    each submit under 1 ms); the encoded JPEG decodes back to the same
    dimensions; the atomic rename never leaves a final-named partial file
    (inject a failure between write and rename); disabled mode allocates
    nothing (assert via counters or a test hook); shutdown with a full queue
    does not leak (ASan build `build-asan`).
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L4: Startup reconciliation, retention and eviction
- **depends_on**: [L2]
- **note**: L4 must not run its startup sweep while the L3 worker runs. Use
  `main.c` ordering, then let periodic eviction skip assets in `pending`
  state.
- **location**: `src/core/observation_retention.c`, `include/core/observation_retention.h` (new), hook into the existing storage maintenance loop (`src/storage/*` / `db_maintenance.c`), `tests/unit/test_observation_retention.c`
- **description**:
  - **Startup**:
    - Delete `*.jpg.tmp` under `<storage>/observations`.
    - Transition every `pending` asset without a finalized file to
      `unavailable(interrupted)`.
    - Transition every `ready` asset whose file is missing to
      `unavailable(write_failed)`.
    - Delete orphan final files that have no asset row.
  - **Periodic** (bounded batch per tick):
    - Per-camera age and byte budgets.
    - Delete an asset file only when no retained observation references it and
      it is not `protected`.
    - Transition the asset to `expired(retention_expired)` and keep the
      metadata.
    - Prune observation history past `observation_history_retention_days` and
      advance the oldest cursor.
  - **Budget exhausted with only protected assets left**: set a flag that makes
    L3 admission return `DROPPED_BUDGET`, and surface it via L10.
  - Never touch recordings, and never inherit the legacy 100-per-camera MQTT
    snapshot policy. Recording deletion does not delete evidence; only
    explicit evidence policy does.
- **validation**: Unit tests use a temp storage directory and seeded rows:
  - an interrupted write is reconciled;
  - a shared asset survives until its last reference expires (A3);
  - a protected asset blocks eviction and flips the budget flag;
  - the prune advances `observation_meta.oldest_seq`;
  - recording deletion leaves assets intact.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L5: Publication relay and the new event type
- **depends_on**: [L2, C3]
- **location**: `src/core/event_envelope.c` (registry entry + validator for `io.lightnvr.detection.observation_changed.v1`), `src/core/event_producers.c` (export `event_producer_enqueue_durable`, wrapping the static `durably_enqueue_event`, with a result that distinguishes `no_route` and `mqtt_disabled`), `src/core/observation_relay.c` + header (new), `src/core/main.c` (start/stop), `tests/unit/test_observation_relay.c`, `tests/unit/test_event_envelope.c`
- **description**:
  - The relay thread wakes on a condvar signaled after commit, plus a 1 s
    poll.
  - It reads pending publications in seq order (bounded batch) and builds an
    envelope with the deterministic message ID (design decision 3).
  - It sends each envelope through `event_producer_enqueue_durable` (router
    → outbox), then marks the publication `relayed` or `relayed_no_route`.
    Then it calls `event_bus_publish` for in-process subscribers.
  - If the outbox is `FULL` or errors, it backs off and leaves the row pending.
    Count the failure for L10.
  - Rows with `cause=classification` still publish, so consumers can display
    them.
  - It drains on startup before steady state.
  - Existing detection topics and `event_producer_publish_detection` stay
    unchanged.
- **validation**:
  - Unit tests: a crash simulation (insert a publication, do not relay,
    re-init, relay drains it); a duplicate relay of the same seq is an outbox
    `DUPLICATE` that is still marked relayed; the envelope validator accepts
    the C3 fixtures and rejects missing fields; route suppression (Fleet 03)
    applies and does not delete history.
  - `test_event_router.c` and `test_event_outbox.c` still pass.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L6: Local detector integration (SOD/TFLite/multi-engine)
- **depends_on**: [L2, L3]
- **location**: `src/video/unified_detection_thread.c`:
  - `detect_on_decoded_frame` around line 3221, after zone, object and
    threshold filtering;
  - `detect_with_runtime_engines` / `append_engine_result` around line 3033,
    where the multi-engine path already builds an `rgb` buffer;
  - the detection-stream background inference thread around line 1100, and
    its result hand-off to the packet thread around line 2396.

  Also `src/video/sod_detection.c` (expose the pre-resize transform) and a
  sidecar `include/video/detection_evidence.h` (engine key, transform and an
  owned RGB handle). Do not change the `detection_result_t` layout, which the
  generated Rust and tests use.
- **description**:
  - **Where to capture**: capture evidence **inside
    `detect_on_decoded_frame`**, while the decoded frame or RGB buffer is
    alive.
    - `run_detection_on_frame` frees the frame (around lines 2993–2995)
      before `report_detections`.
    - The detection-stream path infers on a background thread whose result is
      reported later on the packet thread, when no frame exists.
  - **Eligibility**: when the camera has evidence enabled and any surviving
    detection's label is eligible (`observation_labels` empty, or a match):
    - generate a `batch_id`;
    - hand the owned RGB buffer, the surviving objects and the transform to
      `observation_evidence_submit`. The L3 worker inserts the batch.
  - **Detection-stream path**: the transform is recorded against the
    **detection stream's** resolution, not the main stream's. The evidence
    handle travels with the result slot, or the submit happens on the
    inference thread.
  - Keep the existing `store_detections_in_db_for_camera` and
    `event_producer_publish_detection` calls untouched.
  - Track IDs: emit `track.session` = boot UUID and `namespace` = engine key;
    omit the track when `track_id < 0`.
  - Motion-only engines never create object observations.
  - Box coordinates are already normalized to the full decoded frame. The
    evidence is that full frame, so `transform.mode = none`, or `resize` when
    SOD pre-resizes; document it.
- **validation**:
  - A unit or integration test with a synthetic decoded frame plus a fake
    engine result produces N observations sharing one asset whose JPEG
    matches the frame. With evidence disabled there are no inserts and no
    queue allocation.
  - The A2 moving-bird fixture: the saved image equals the inference frame,
    not a later frame. Compare a hash of the decoded inference frame against
    the decoded evidence (PSNR above a threshold).
  - Existing `tests/test_stream_detection.c` and `test_detection_*` pass.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L7: HTTP API detector integration and ONVIF unavailable evidence
- **depends_on**: [L2, L3, L6, C3]
- **note**: Serialized after L6, because both change call signatures in
  `unified_detection_thread.c`.
- **location**: `src/video/api_detection.c` (`perform_detection_request` around line 681, `finalize_detections` around line 798, the go2rtc-snapshot path `api_detection_should_use_go2rtc_snapshot` around line 124), `src/video/onvif_detection.c`, `src/video/detection_integration.c`, `tests/unit/test_api_detection.c`
- **description**:
  - **HTTP**: today `jpeg_data` is freed before `finalize_detections`
    (`api_detection.c:996` and `:1093`). `finalize_detections` receives only
    `stream_name`, calling `store_detections_in_db(stream_name, …)` with no
    `camera_uuid`.
    - Change the signatures so the submitted JPEG bytes (ownership
      transferred) and `camera_uuid` reach `finalize_detections`. Update its
      callers in `unified_detection_thread.c`.
    - Retain those exact bytes, plus the transform from source frame size to
      submitted JPEG size and any server-side letterbox the response
      declares.
    - Insert the batch, then call the encoded-bytes variant of L3 (no
      re-encode).
    - When the API path fetched a go2rtc snapshot *as the inference input*,
      that snapshot **is** the evidence and is acceptable.
    - Never fetch a second image after inference.
  - **ONVIF and external events**: create observations only for object-class
    events (not motion), with asset `unavailable(no_matching_media)`.
- **validation**:
  - Unit tests: `test_api_detection.c` asserts that the asset bytes are
    byte-identical to the request body image; a letterboxed transform fixture
    maps boxes correctly (C3 fixture); ONVIF produces `unavailable` with the
    reason.
  - The A2 HTTP variant passes.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L8: Observation and capability API handlers
- **depends_on**: [L2, C2]
- **location**: `src/web/api_handlers_observations.c` (new), `include/web/api_handlers.h`, `src/web/libuv_api_handlers.c` (C2 already registers the routes; replace the GET stubs and keep the PUT stub at 501), `docs/api/contracts/coverage.json` (regenerate), `src/web/openapi_projection.c` if routes must be listed, `tests/unit/test_api_handlers_observations.c`
- **description**:
  - **Authorization**: resolve each observation's capture-time `camera_uuid`,
    then authorize via the existing Fleet 02 camera-scoped helpers
    (`src/core/authorization.c`). Use `live.view` for metadata and
    `snapshot.create` for images.
    - List queries intersect the requested `camera_uuid[]` with the
      principal's authorized set. Unauthorized explicit cameras return `403`,
      matching the investigation precedent. An unknown or unauthorized
      single ID returns `404`.
  - **Snapshot**: `ready` streams the file with `Content-Type: image/jpeg`,
    `Cache-Control: private` and an ETag of the asset UUID. `pending` returns
    `202` + `Retry-After`. `expired` returns `410`. `unavailable` returns
    `404` with a reason. It never substitutes a latest frame.
  - **Capabilities**: returns the installation source (`event_identity`),
    contract versions, operations and limits. `annotate` is false until L14.
  - **Audit**: snapshot fetches go through the existing audit path where
    sensitive-access auditing exists. Never log bytes or tokens.
- **validation**:
  - Unit tests per status code.
  - Contract coverage in L11.
  - OpenAPI projection lint shows each new route has exactly the declared
    authorization requirement.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L9: Per-camera opt-in and budget configuration (API + UI)
- **depends_on**: [L2, C2]
- **location**: L2 already adds the `stream_config_t` and `db_streams` fields. `src/web/api_handlers_streams_get.c`, `src/web/api_handlers_streams_modify.c`, `docs/api/openapi.json` (stream schema fields; coordinate with C2's author or follow on), `web/js/components/preact/StreamConfigModal.tsx`, `web/js/types/*`, i18n strings
- **description**:
  - Expose evidence enable, eligible labels, max age and max bytes per stream,
    gated by `camera.configure`.
  - The UI section is "Observation evidence for integrations". It notes that a
    bird-capable object detector is required and that it is off by default.
  - Validate bounds server-side.
  - The detection thread reads these via the stream config reload path; a
    changed setting takes effect without restart.
- **validation**:
  - `test_db_streams` or the stream API unit tests round-trip the fields.
  - `web` jest/Playwright: the toggle persists and the default is off.
  - `tests/contracts/streams.api.spec.ts` still passes, with schema additions
    accepted.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L10: Diagnostics and metrics
- **depends_on**: [L3, L4, L5, L8]
- **location**: `src/web/api_handlers_metrics.c`, `src/web/api_handlers_system_health.c` / `fleet_health.c`, `web/js/components/preact/SystemView.tsx` (or the system health panel)
- **description**: Report the following through the existing
  metrics/diagnostics:
  - accepted and dropped observations by reason;
  - evidence backlog and failures;
  - retained bytes per camera;
  - the oldest recoverable cursor and its time;
  - publication pending, failed and lag;
  - the budget-exhausted flag.

  Add a per-camera "latest eligible observation + evidence state" probe for
  YA-WAMF setup diagnostics. It is exposed via
  `GET /api/observations?camera_uuid=X&limit=1` (no new endpoint), and
  distinguishes "no detections" from transport failure.
- **validation**: Unit tests on metric serialization. The metrics endpoint
  shows the counters move under an L3 saturation test. When the feature is
  disabled, the values are zeros and no queue exists.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L11: lightNVR contract and reliability tests (A1–A3, A5–A8 lightNVR side)
- **depends_on**: [L5, L6, L7, L8, L9, L10, C3]
- **location**: `tests/contracts/observations.api.spec.ts` (new), `tests/contracts/fixtures/observations/seed.ts`, `tests/contracts/permissionCases.ts` (add cases), `tests/reliability/observations_crash_boundaries.*` (new), `tests/contracts/eventsDelivery.api.spec.ts` (extend with `mqtt-sink.ts`)
- **description**:
  - **Playwright API contracts**:
    - Camera-scoped token isolation on list, get and snapshot (A7).
    - Rename, delete and name-reuse fixtures (reuse
      `seedHistoricalNameReuse`) never transfer access.
    - Distinct outcomes for pending, expired, unavailable, unknown and expired
      cursor (A8).
    - Cursor pagination with pending→ready recovery.
    - MQTT sink receives one message ID per revision, stable across redelivery.
  - **Reliability**: kill the process (SIGKILL) at each boundary, then restart
    and assert that no `ready` asset points at an unfinished file and that
    every committed change is eventually relayed (A6). The boundaries are:
    - after the batch insert;
    - after tmp write;
    - after rename, before commit;
    - after commit, before relay;
    - after relay, before outbox delivery.

    Also assert the startup order: reconciliation, then worker, then relay.
  - **DB restore**: a cursor taken before a restore returns `410`
    (epoch or seq check).
  - **Broker down**: stop the broker, generate 100 observations, restart, and
    verify all 100 are recoverable via the outbox and the cursor (A5,
    lightNVR half).
- **validation**: The new specs pass in CI next to the existing contract
  suite, and the existing `evidence.*.spec.ts` lists of operation IDs are
  untouched.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L12: lightNVR documentation and deployment recipe
- **depends_on**: [L11]
- **location**: `docs/EVENT_CONTRACT.md`, `docs/MQTT_INTEGRATION.md`, `docs/DETECTION_ENGINES.md`, `docs/integrations/YA_WAMF.md` (new: the recipe covering scoped token creation, broker routes, a bird-capable detector, retention/budgets and diagnostics), `docs/prd/README.md` (status), `docs/prd/INT_01_Event_Enrichment_YAWAMF.md` (status)
- **description**: Document the new event type and topic, the observation
  API, evidence semantics (low-resolution inference image, not a recording
  frame), retention and cursor expiry, and rollback (disable per camera;
  evidence and annotations are kept; the 0095 down-migration drops data, so
  take a backup first). Write the supported-version matrix.
- **validation**: Doc links resolve. The recipe is executed verbatim in X1/A14.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### P1 — YA-WAMF (repo `../YetAnother-WhosAtMyFeeder`, branch `feature/lightnvr-provider`)

#### W1: Source-provider boundary and Frigate provider extraction
- **depends_on**: [W0]
- **location**: `backend/app/services/sources/__init__.py`, `base.py` (the `NvrSourceProvider` Protocol + `Capabilities` dataclass), `frigate_provider.py`, `registry.py` (new); `backend/app/services/frigate_client.py` (unchanged API, wrapped); `backend/app/main.py` (construct the provider from config); `backend/tests/pure/test_source_capabilities.py`
- **description**:
  - The provider interface (PRD T1) covers:
    - `identity()` → `(kind, instance)`;
    - `list_cameras()`;
    - `capabilities()`;
    - `parse_live_message(topic, payload)` → a list of `SourceObservation`
      (common input carrying optional lifecycle `new|update|end` only when
      supplied);
    - `history(cursor, filters)`;
    - `fetch_snapshot(external_id, crop_region?)`;
    - optional `fetch_clip`, `fetch_recording_frame` and `write_classification`
      operations guarded by capabilities.
  - `FrigateProvider` delegates to `FrigateClient` and preserves the exact
    current behavior.
  - Keep `FrigateClient` as is. No plugin framework.
- **validation**: W0 is green. Pure tests cover the capability declarations.
  `pytest` and the pure lane pass.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W2: Alembic migration — source identity, recovery cursor, notification intent
- **depends_on**: [W0]
- **location**: `backend/migrations/versions/<rev>_add_source_identity.py`, `backend/app/db_schema.py`, repositories under `backend/app/repositories/*` that look up by `frigate_event`, `backend/tests/test_source_identity_migration.py`, `MIGRATION.md`
- **description**:
  - **New columns** (nullable or server-defaulted, so older binaries' inserts
    still succeed; see design decision 8): add `source_kind`,
    `source_instance`, `external_observation_id` and `source_camera_id` to
    `detections`.
    - Other tables (media cache, `processing_job_state`, favourites,
      corrections, `bird_observations`) keep keying on `frigate_event`, which
      is `UNIQUE NOT NULL`; the PK is `id`.
    - They only gain source columns where a lookup does not go through
      `detections`.
  - **Backfill**: legacy rows get `('frigate', <frigate instance>,
    frigate_event)`. The instance is derived from the configured
    `frigate_url` host, normalized; document the derivation.
    - Add a startup lazy backfill for rows inserted with a NULL source by an
      older binary after the upgrade.
  - **Constraints**: add a partial `UNIQUE(source_kind, source_instance,
    external_observation_id) WHERE source_kind IS NOT NULL`. Keep the
    `frigate_event` unique column and its FK web intact.
  - **New table** `source_cursors(source_kind, source_instance, cursor,
    epoch, updated_at, last_error)`. Notification state reuses or extends the
    existing `mark_notified` mechanism (W7), and durable work reuses
    `processing_job_state`. Neither gets a new table.
  - **Schema check**: `_verify_schema` deliberately lets older builds run
    against a newer DB and only warns. Do not claim refusal. Document in
    `MIGRATION.md` that downgrading the binary after enabling lightNVR is
    unsupported without restoring the backup.
  - **Batch mode**: use batch mode and keep `PRAGMA foreign_keys` handling
    consistent with existing migrations.
  - Document backup and rollback in `MIGRATION.md`.
- **validation**:
  - Migration tests: `upgrade → downgrade → upgrade` is a no-op; a seeded
    legacy DB keeps IDs, photos, favourites, corrections and queued jobs.
  - Duplicate `(frigate, inst, id)` is rejected.
  - An insert by the old code path (no source columns) still succeeds and is
    lazily backfilled.
  - Changing the configured source does not rewrite old rows.
  - Single Alembic head check passes. W0 is green.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W3: Route every Frigate call through the provider and gate by capability
- **depends_on**: [W1]
- **location**: every current importer of `frigate_client`. Routers:
  - `routers/ai.py`, `routers/events.py`, `routers/proxy.py`,
    `routers/settings.py`.

  Services:
  - `archive_service.py`, `auto_video_classifier_service.py`,
    `backfill_service.py`, `classification_input_provenance.py`;
  - `full_visit_clip_service.py`, `high_quality_snapshot_service.py`,
    `recording_snapshot_input.py`, `video_*`, `visit_film_service.py`;
  - `garden_automatic_review.py`, `garden_capture_service.py`,
    `garden_crop_service.py`, `garden_identification.py`,
    `garden_media_cache.py`, `garden_motion.py`;
  - `inaturalist_service.py`, `media_integrity_scan.py`, `media_probe.py`,
    `notification_orchestrator.py`, `timezone_repair_service.py`.

  Also `media_cache.py`, and a new test `backend/tests/pure/test_no_direct_frigate_calls.py`.
- **description**:
  - Replace direct `FrigateClient` and `/api/events/...` usage in shared
    workflows with provider calls.
  - Media cache keys are resolved by source identity; legacy Frigate keys stay
    readable.
  - Add neutral media routes `/api/media/{source_kind}/{external_id}/snapshot`
    (and thumbnail) while preserving existing Frigate proxy URLs.
  - HQ snapshot, recording snapshot, clip, full-visit, garden-capture and
    video-classifier services run only when the provider declares that
    capability. Otherwise they record a "skipped: unsupported by source"
    reason and never infer support from an HTTP error.
  - Add a static test (AST or import scan) that fails if any module outside
    `services/sources/frigate_provider.py` and `frigate_client.py` imports
    `frigate_client` or builds `/api/events/` paths.
- **validation**: W0 stays green (A10). The static test passes. With a fake
  snapshot-only provider, a test asserts that zero clip, HQ or
  recording-snapshot jobs run and that no `/api/events/` request is made (A9
  backend half).
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W4: Common ingestion pipeline (provider parsing → shared admission/idempotency)
- **depends_on**: [W1, W2, W3, C3]
- **location**: `backend/app/services/mqtt_service.py`, `backend/app/services/event_processor.py` (`process_mqtt_message`, `_process_event_payload`, `_save_detection`, `_handle_terminal_event_enrichment`), `classification_admission.py`, `backend/tests/pure/test_observation_admission.py`
- **description**:
  - **MQTT**: `mqtt_service` subscribes to the topics the active provider
    declares and dispatches to `provider.parse_live_message`. Frigate
    availability and stall logic stays Frigate-only.
  - **Common pipeline**:
    - Dedupe by `source + message_id` (persist a recent-message table or
      reuse a durable job key).
    - Admission by `source + observation_id`; a stale revision or a
      `change_cause=classification` is ignored.
    - Classify only when `evidence_state=ready`.
    - Per-object aligned region crop from box plus transform; multiple objects
      in one frame are classified separately (A3).
  - **Lifecycle hooks**: Frigate `new/update/end` and end-refinement run only
    when lifecycle metadata is present. Never synthesize `end`.
  - **Visits**: visit grouping is unchanged; it uses a camera-and-time window
    only and never claims identity.
  - **Multi-bird**: implement the decision 8 mapping (one observation, one
    detection; sibling detections share an asset). Verify that visit counts
    and the species leaderboard do not double-count siblings relative to the
    Frigate multi-bird model, and document any intended difference.
  - **Camera identity**: store `source_camera_id` (the lightNVR camera UUID).
    `camera_name` is display-only for lightNVR rows.
- **validation**: Pure tests cover admission (duplicate, stale revision,
  pending, classification-only) and per-object crop math against the C3
  fixture. W0 is green.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W5: lightNVR client and provider
- **depends_on**: [W1, C3]
- **location**: `backend/app/services/sources/lightnvr_client.py`, `lightnvr_provider.py` (new), `backend/app/config_models.py` (`LightNvrSettings`: `api_url`, `token` (secret, redacted), `mqtt` topic prefix, `camera_uuids`, `source_instance` override), `backend/tests/fixtures/lightnvr/` (vendored C3), `backend/tests/test_lightnvr_provider.py`
- **description**:
  - Implement `identity` from `/api/integrations/capabilities` `source`.
  - Implement `list_cameras` with existing lightNVR camera discovery,
    non-admin.
  - Implement `capabilities`, mapping from the capability doc.
  - Implement `parse_live_message` for `observation_changed.v1`.
  - Implement `history(cursor)`: on `410 cursor_expired`, return a
    `RecoveryRequired(oldest_cursor)` result. This includes the case where
    lightNVR's DB was restored and the epoch changed.
  - Implement `fetch_snapshot`:
    - `202` → retry with bounded backoff honoring `Retry-After`;
    - `410` → `Expired`;
    - `404` with reason → `Unavailable`;
    - network failure → `TemporarilyUnreachable`.
  - Implement `write_classification` as a stub that raises Unsupported until
    W10.
  - Reuse httpx with timeouts. Redact the token in logs.
  - Sanitize IDs before use in any cache path, per YA-WAMF CLAUDE.md §1.
- **validation**: Tests against a mocked httpx transport serving the C3
  fixtures cover every outcome class. The secret is redacted in the settings
  API response. The pure parser test passes on every fixture.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W6: Cursor recovery and backfill for providers
- **depends_on**: [W2, W4, W5]
- **location**: `backend/app/services/backfill_service.py`, `frigate_event_absence.py` / `frigate_missing_policy.py` (keep these Frigate-only), new `source_recovery_service.py`, `backend/app/routers/backfill.py`, tests
- **description**:
  - On start and reconnect, page through `provider.history(saved_cursor)`.
  - Feed each page through the same admission path as live messages, so
    live/recovery overlap shares dedup.
  - Advance `source_cursors` only after each item is durably accepted or
    recorded as skipped with a reason.
  - Persist retryable failures in `processing_job_state` with a dedicated
    `pipeline` value.
  - Recovery is silent: no standard notifications for historical items by
    default, enforced via notification intent.
  - Distinguish missing, expired, unsupported and temporarily unreachable in
    stored outcomes and the backfill UI messages.
- **validation**: Tests cover:
  - 100 fixture observations with interleaved pending→ready recover to
    exactly 100 detections;
  - a crash mid-page resumes without duplicates;
  - an expired cursor yields a recovery-required state shown in health;
  - live+recovery overlap with each message replayed 10× gives one detection
    and one intent (A4, A5 YA-WAMF half).
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W7: Durable notification intent
- **depends_on**: [W2, W4]
- **location**: `backend/app/services/notification_orchestrator.py`, `notification_dispatcher.py`, `notification_links.py` (build links through the provider), tests
- **description**:
  - Reconcile the existing `mark_notified` / `notify_mode` mechanism rather
    than adding a parallel one.
    - Record the notification intent durably in the same transaction as the
      detection save, by extending the existing notified state with an
      intent and outcome. Dispatch only when the intent is newly recorded.
  - **Sources without lifecycle**: `notify_mode="final"` currently fires
    only on `event_type=="end"`, which lightNVR never sends.
    - For providers without `track_lifecycle`, "final" means the first
      admitted ready classification, bounded by the existing cooldown or
      visit rules (decision 8).
    - Add a test that lightNVR users on "final" are notified.
  - Mark the outcome after the delivery attempt. An ambiguous external
    delivery state (timeout after send) is recorded as `unknown` and surfaced
    rather than retried blindly.
  - Notification attachments fetch evidence through the provider.
  - Preserve existing Frigate notification behavior (W0).
- **validation**: A replay test sends the same observation 10 times and gets
  one intent and one dispatch. A simulated crash after intent and before
  dispatch is recoverable and visible. W0 is green.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W8: Settings, onboarding and health UI for source selection
- **depends_on**: [W3, W5]
- **location**: `backend/app/routers/settings.py`, `routers/setup.py`, `routers/diagnostics.py`, `backend/app/services/setup_state_service.py`, `apps/ui/src/**` (settings and onboarding Svelte views, health panel), `backend/locales/*` and UI i18n
- **description**:
  - Add a source selector; the default stays Frigate for existing configs, and
    a failure never switches providers.
  - For lightNVR: API URL, scoped token and MQTT connection, then a camera
    picker using lightNVR discovery.
  - The connection test reports API access, event connectivity, evidence
    access and available features as separate results.
  - A diagnostic shows the latest eligible observation and its photo state
    (ready, missing or expired), and states "no bird detections yet" as
    distinct from an error.
  - Explain that a bird-capable detector is required.
  - Unsupported video features are disabled with a short explanation and do
    not affect health status.
- **validation**: Backend API tests for the connection test outcomes. UI:
  `npm run check`, `npm test`, and a browser test that a snapshot-only source
  shows disabled clip controls with an explanation (A9 UI half).
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W9: YA-WAMF acceptance suite (A4, A9, A10, A1/A3 consumer side)
- **depends_on**: [W3, W6, W7, W8]
- **location**: `backend/tests/test_lightnvr_acceptance.py`, `backend/tests/test_frigate_regression_baseline.py`
- **description**: Combine W0 with fake-lightNVR scenarios driven by the C3
  fixtures and a fake MQTT broker:
  - a single bird is classified and labelled with the source camera;
  - two birds become two detections with correct crops;
  - 10× replay plus overlapping recovery gives one detection and one intent;
  - a snapshot-only source runs no clip jobs;
  - every Frigate regression case passes post-migration.
- **validation**: The full `pytest`, `npm run check` and `npm test` pass on
  the branch.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### P1 — Cross-repo gates

#### X1: End-to-end P1 suite and A14 pilot recipe
- **depends_on**: [L11, L12, W9]
- **location**: `tests/integration/yawamf_e2e/` in lightNVR (docker-compose: mosquitto + lightNVR + YA-WAMF branch image + fixture RTSP via go2rtc), plus the README
- **description**: Run the documented recipe (L12) verbatim with a recorded
  feeder fixture and a pinned bird-capable detector model:
  - A1: identity and labels.
  - A2: moving-bird alignment, for local and HTTP detectors.
  - A3: two birds.
  - A4: replay ×10.
  - A5: broker stopped for 100 observations, then both apps restarted.
  - A6: kill points.
  - A8: expired, unavailable and temporary outcomes, end to end.

  Record the tested source and model versions into the supported-version
  matrix. A14 is the operator pilot on a real camera using this recipe; it is
  a manual checklist that records outcomes.
- **validation**: The compose suite passes in CI or nightly. The A14 checklist
  is completed and committed with versions.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### X2: Saturation and performance release gate (A11)
- **depends_on**: [C4, L6, L7, L10]
- **location**: `tests/performance/observations/`
- **description**:
  - Rerun the C4 workload with evidence enabled on every stream at maximum
    cadence.
  - Saturate the queue and byte budgets. Assert that limits hold, drops carry
    reasons, there are no new recording gaps, p95 recording-path latency
    rises by no more than 5%, and producer enqueue never waits on I/O
    (submit-latency histogram).
  - Record memory, CPU, disk and queue high-water marks.
  - Finalize the C1 provisional defaults from the results. If the threshold
    fails, reduce admission (lower the per-camera cap, the queue bytes or the
    batch commit rate) and rerun.
- **validation**: A committed results JSON meets the thresholds, and the C1
  defaults are updated from "provisional" to final.
- **status**: Not Completed
- **log**:
- **files edited/created**:

### P2 — Classification feedback

#### L13: Migration 0096, classification DB layer and authz action
- **depends_on**: [L2]
- **location**: `db/migrations/0096_add_observation_classifications.sql`, regenerated `include/database/db_embedded_migrations.h`, `src/database/db_observations.c` (or a new `db_observation_classifications.c`), `tests/unit/test_db_observation_classifications.c`. The authz action is mirrored in C and catalog order defines the bit, so also append to:
  - the enum in `include/core/authorization.h`;
  - the catalog in `src/core/authorization.c` (around line 22);
  - the OpenAPI action enums in `docs/api/openapi.json` (three places), then
    regenerate;
  - the token-policy UI and locales (`web/tests/apiTokenPolicy.spec.js`);
  - `tests/unit/test_authorization.c`.
- **description**:
  - **Table** `observation_classifications`:
    - Columns: `observation_uuid`, `key`, `writer_principal`, `classifier`,
      `model_version`, `label`, `taxonomy_id`, `confidence`, `classified_at`,
      `input_asset_uuid`, `input_revision`, `kind`, `annotation_revision`,
      `created_at`, `updated_at`.
    - Primary key `(observation_uuid, key)`.
  - **Authz action** `observations.annotate`: bit 19, camera-scoped,
    Administrator only, same pattern as 0078.
  - **DB upsert**:
    - Same payload → no-op that returns the current row.
    - Different payload → requires `expected_revision == annotation_revision`.
    - Reject when `input_asset_uuid` differs from the observation's asset.
    - Writer mismatch on an existing key is rejected.
    - An `owner_confirmed` row from the owner path cannot be overwritten by
      `automated`.
    - Each change bumps the observation revision with cause `classification`,
      plus the change and publication rows, in the same transaction.
  - Original detector fields are never modified.
- **validation**: Unit tests for each rule, plus a migration up/down/up test
  where bit 19 does not collide with existing bit indexes. Query
  `authz_actions` for the max `bit_index` first; if anything after 0078 used
  19, take the next free bit.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L14: PUT annotation API
- **depends_on**: [L13, L8, L5]
- **location**: `src/web/api_handlers_observations.c`, `docs/api/openapi.json` (remove the P2 stub note, add the authz requirement `observations.annotate`; the action enum already lists it via L13), regenerated projections and `docs/api/contracts/coverage.json`, `tests/contracts/observations.api.spec.ts`, `tests/contracts/permissionCases.ts`
- **description**:
  - Replace the 501 stub: parse and bound the body, then authorize with
    `observations.annotate` on the capture-time camera.
  - Derive the writer namespace from the token or principal; a body claiming
    another namespace returns `403`.
  - Map the DB outcomes to the C1 error catalog.
  - Audit the mutation, without credentials or bytes.
  - `GET /api/observations/{id}` includes classifications, with automated and
    owner decisions separated.
  - Capabilities now advertise `annotate`.
- **validation**: Contract tests for A12 (retry no-op, stale revision `409`,
  owner overwrite `409`, evidence mismatch `422`) and A7-annotate (another
  camera's observation gives `404`/`403` and no row is written). The MQTT sink
  sees a `classification` change whose message ID is stable.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### L15: Minimal classification display in lightNVR
- **depends_on**: [L14]
- **location**: the detection-detail surfaces that already render detections: `web/js/components/preact/timeline/TimelinePlayer.tsx`, `web/js/components/preact/recordings/recordingsAPI.tsx`, `web/js/types/timeline.ts`, `web/js/types/core.ts`; i18n strings; web tests
- **description**:
  - Where a detection links to an observation, show the species label,
    classifier confidence, classifier/source and an owner-confirmed badge,
    with a link to the observation.
  - Keep the original `bird` label and detector confidence visible.
  - No dashboard and no taxonomy browser.
  - If detections do not carry `observation_uuid` today, add a light join in
    the detection results API: match by `camera_uuid` + timestamp + label,
    or better, store `observation_uuid` on the detection row in 0096. Decide
    in L13.
- **validation**: Web unit tests and a Playwright UI spec show both the
  original and classified labels. Run i18n literal baseline checks.
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### W10: YA-WAMF classification write-back with correction precedence
- **depends_on**: [W4, W5, W7, C3]
- **location**: `backend/app/services/sources/lightnvr_provider.py` (`write_classification`), new `classification_writeback_service.py`, durable jobs via the existing `processing_job_state` with `pipeline='lightnvr_writeback'` (no new table, ideally no Alembic revision), settings toggle (UI and backend), corrections flow hook, tests
- **description**:
  - After a lightNVR detection is classified, or an owner correction is saved,
    enqueue a durable write-back job.
  - The worker PUTs to `…/classifications/yawamf/species`, sending
    `expected_revision` and `kind`:
    - an owner correction sends `owner_confirmed`;
    - an automated result never overwrites a local owner correction;
    - `409` → refetch and reconcile;
    - `422` → drop with a reason.
  - Retries never create detections or notifications.
  - A write-back outage leaves the YA-WAMF result usable and visible as
    "sync pending".
  - Incoming `classification` changes are ignored for classification (W4
    rule).
  - Off by default, with a settings toggle.
- **validation**: Tests against the C3 PUT fixtures cover retry idempotency, a
  `409` reconcile, owner precedence, and an outage that creates no
  re-notification (A13).
- **status**: Not Completed
- **log**:
- **files edited/created**:

#### X3: P2 end-to-end
- **depends_on**: [X1, L14, L15, W10]
- **location**: `tests/integration/yawamf_e2e/` (extend)
- **description**:
  - With write-back enabled, the species appears in both products.
  - Replays and stale revisions keep exactly one annotation.
  - An owner correction in YA-WAMF propagates as `owner_confirmed`, and a
    later automated result does not override it.
  - The classification change does not trigger another classification or
    notification cycle.
  - A lightNVR API outage leaves the YA-WAMF detection intact.
- **validation**: A12 and A13 pass end to end. The supported-version matrix is
  updated.
- **status**: Not Completed
- **log**:
- **files edited/created**:

## Parallel Execution Groups

| Wave | Tasks | Can Start When |
|------|-------|----------------|
| 1 | C1, C4, W0 | Immediately |
| 2 | C2, C3, L1, W1, W2 | C1 (C2, C3, L1); W0 (W1, W2) |
| 3 | L2, W3, W5 | L1 (L2); W1 (W3); W1+C3 (W5) |
| 4 | L3, L4, L5, L8, L9, L13, W4, W8 | L2 (L3, L4, L13); L2+C3 (L5); L2+C2 (L8, L9); W1+W2+W3+C3 (W4); W3+W5 (W8) |
| 5 | L6, L10, L14, W6, W7 | L2+L3 (L6); L3+L4+L5+L8 (L10); L13+L8+L5 (L14); W2+W4+W5 (W6); W2+W4 (W7) |
| 6 | L7, L15, W9, W10 | L6+C3 (L7); L14 (L15); W3+W6+W7+W8 (W9); W4+W5+W7+C3 (W10) |
| 7 | L11, X2 | L5–L10+C3 (L11); C4+L6+L7+L10 (X2) |
| 8 | L12 | L11 |
| 9 | X1 | L11+L12+W9 |
| 10 | X3 | X1+L14+L15+W10 |

These pairs touch the same files, so give them to one agent or merge them in
sequence to avoid conflicts:

| Shared file(s) | Tasks |
| --- | --- |
| `src/web/api_handlers_observations.c` | L8 → L14 |
| `db_observations.c` | L2 → L13 |
| `event_processor.py` | W3 → W4 |
| `docs/api/openapi.json` | C2 → L8 / L9 → L13 → L14 |
| `src/web/libuv_api_handlers.c` + `docs/api/contracts/coverage.json` (hash/line-pinned; regenerate on every edit) | C2 → L8 → L14 |
| `src/video/unified_detection_thread.c` | L6 → L7 |
| `include/core/config.h` / `db_streams.c` | L2 → L9 |
| Alembic head | W2 (W10 only if it ends up needing a revision) |

## Testing Strategy

- **Unit tests** (`tests/unit`, ASan build for L3/L4/L5/L6): each L* task ships
  its own tests (idempotency, CAS transitions, cursor/watermark, atomic
  finalization, non-blocking admission, relay crash recovery).
- **Contract tests** (`tests/contracts`): OpenAPI projection lint; C3 fixtures
  validated against schemas; Playwright API specs for authorization isolation,
  name reuse and distinct error outcomes; MQTT sink message-ID stability.
- **YA-WAMF**: W0 Frigate regression baseline green at every step (A10); pure
  tests for admission, crop math and capability gating; static "no direct
  Frigate calls" test; migration up/down/up test.
- **Reliability**: SIGKILL at each persistence boundary (A6); broker outage
  with 100 observations (A5).
- **Performance**: C4 baseline vs X2 enabled run (A11, ≤5% p95, zero new
  gaps).
- **E2E**: X1/X3 docker-compose suite running the documented recipe; A14
  manual pilot with recorded versions.

## Risks & Mitigations

- **Detection-thread latency.** The L3 worker performs every observation
  insert. The detection thread only hands off an RGB buffer it already owns
  (with no extra copy where possible). X2 confirms this.
- **Memory held by queued frames.**
  - Queue items are owned RGB24 buffers, not `AVFrame` references, so no
    decoder or hwaccel surfaces are pinned.
  - Bound the queue by bytes.
  - Run ASan and a shutdown test.
- **Stale local FFmpeg headers give false crash signals** (project memory).
  Run evidence/encoder tests in the project Docker toolchain.
- **Global DB mutex contention.** Evidence worker commits share the
  non-recursive mutex with recording-path writes. Keep transactions small and
  batched, and have X2 measure mutex hold time.
- **Migration number or authz bit collisions with concurrent PRs.** Reserve
  0095/0096 and bit 19 in C1, and re-check before merging L1 and L13.
- **The YA-WAMF identity migration rebuilds large tables in batch mode.**
  - Document the backup step.
  - Test on a large seeded DB for runtime.
  - The schema guard refuses older binaries.
- **Hidden Frigate calls remain.** The W3 static test plus the fake
  snapshot-only provider test, which asserts zero `/api/events/` requests.
- **Evidence expires before recovery.**
  - Advertise the oldest cursor and remaining availability.
  - YA-WAMF caches admitted photos.
  - Recovery reports expired items as distinct outcomes.
- **Upstream YA-WAMF declines the integration.**
  - Keep the changes upstreamable: small, provider-scoped, preserving Frigate
    behavior.
  - Name a fork maintenance owner before production release.
- **Annotation authority is ambiguous.** C1 freezes writer namespace
  derivation and owner precedence before L13 and L14 start.
- **Motion-only or non-bird detectors.** Setup diagnostics (L10, W8) report
  "no eligible observations" distinctly from failures, and the docs require a
  bird-capable detector.
