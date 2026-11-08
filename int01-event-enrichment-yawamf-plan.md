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
- **status**: Completed
- **log**:
  - lightNVR commit `d968b4d5`; YA-WAMF commit `466d5a28` (vendored copy,
    `SOURCE.md` names `d968b4d5` and lists SHA-256 sums).
  - **Inventory** (46 files, all listed in `manifest.json` with kind,
    schema, request/status/headers or MQTT topic):
    - One scenario, one change log (`manifest.changes`, seq 1001–1013).
    - `observations/` (10): single bird A at revisions 1–5
      (pending, ready (= contract §3.6), ready + automated annotation,
      owner-confirmed, expired); B `unavailable(queue_full)`; two birds
      C0/C1 sharing asset `9999…`; D, an HTTP letterbox (1280x720 → 640x640,
      scale 0.5, pad_y 140, with the inference-space box in the manifest);
      E, ONVIF `no_matching_media` with a null box.
    - `events/` (8): complete envelopes in `event_envelope.c` key order for
      `created`, `evidence` (ready and expired) and `classification`. IDs are
      UUIDv5 per §5.4. A r2 is the contract test vector
      `43ebdf02-…`.
    - `message-ids.json`: the namespace derivation and every vector.
    - `pages/` (4): three chained `limit=2` pages at watermark 1010
      (A reappears at r3; `next_cursor` is the last *scanned* change), plus
      `cursor=head`.
    - `errors/` (9): `cursor_expired`, `invalid_cursor`, `evidence_pending`
      (202 + `Retry-After`), `evidence_unavailable`, `evidence_expired`,
      `observation_not_found`, `forbidden` (D4), `not_implemented`,
      `service_unavailable`.
    - `capabilities/` p1 and p2.
    - `classifications/` (8 exchanges, each with `observation_file` = the
      precondition): 201 create, 200 no-op retry, 409 `revision_conflict`,
      200 owner confirm, 409 `owner_decision_locked`, 422
      `evidence_mismatch` (wrong asset; pending), 403
      `namespace_forbidden`.
    - `media/two-birds-160x80.jpg` + box manifest (pixel coords, colors,
      sha256).
  - **Generator**: `scripts/contracts/observation-fixtures.mjs`
    (`--check`, `--jpeg`, `--vendor <ya-wamf>`, `--check-vendor <ya-wamf>`).
    Its helpers `uuidv5`, `messageId` and `encodeCursor` are exported.
  - **RED→GREEN**: `observation-fixtures.test.mjs` failed 9/10 with 1 skip
    before the fixtures existed. After adding them it passes 9/9 with the
    vendor check skipped, and 10/10 once vendored. Negative cases: reason on
    ready, `snapshot_url` on pending, ready without `snapshot_url`, expired
    without reason, a 32-byte label, `object_index` 20, a closed PUT body
    and an event reason on ready are all rejected. Tampering with an event
    revision fails the message-ID and generator checks.
    `node --test tests/contracts/tooling/*.test.mjs`: 59 tests, 58 pass,
    1 skip (the vendor check, before vendoring).
  - **Re-sync**: after committing fixture changes in lightNVR, run
    `node scripts/contracts/observation-fixtures.mjs --vendor ../YetAnother-WhosAtMyFeeder`.
    It refuses if the fixtures are dirty, unless `--allow-dirty`. Then
    commit in YA-WAMF. The lightNVR tooling test compares the copy byte for
    byte when the sibling checkout exists or `LIGHTNVR_YAWAMF_DIR` is set,
    and skips otherwise.
  - **Gotchas for later tasks**:
    - **JPEG bytes**: they are canonical and are re-encoded only with
      `--jpeg`, because sharp/libjpeg output can drift. The JSON `bytes` and
      `sha256` derive from the file on disk.
    - **Envelope**: OpenAPI has no envelope schema. The test checks envelope
      shape locally, and validates `data` against `ObservationChangeEvent`.
    - **Two fixture-level conventions** not stated in the contract:
      - pending evidence keeps `origin`, `media_type`, `captured_at` and
        `transform`, and leaves `width`, `height`, `bytes` and `expires_at`
        null;
      - evidence that never had media has every media field null.

      L2 and L8 should match them or amend the contract.
    - **Fixture vs. contract wording**: the expired fixture keeps
      `expires_at` at its estimate, and the `evidence_expired` error carries
      the actual `expired_at`.
    - **W10**: the PUT fixtures use user `yawamf`. Read the namespace from
      `capabilities.principal.writer_namespace`; the test asserts that each
      exchange's namespace is advertised.
    - **W5**: the vendored `README.md` links to
      `../../../../docs/...`, which resolves only in lightNVR.
- **files edited/created**:
  - `tests/fixtures/integrations/observations/**` (new: 46 files plus `README.md`)
  - `scripts/contracts/observation-fixtures.mjs` (new)
  - `tests/contracts/tooling/observation-fixtures.test.mjs` (new)
  - YA-WAMF: `backend/tests/fixtures/lightnvr/**` (new, with `SOURCE.md`)

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
- **status**: Completed (partial validation: the DB-probe series failed the
  2% repeatability check, see the log)
- **log**:
  - **Harness.** `run-workload.mjs` sets up the workload:
    - a standalone go2rtc with one always-on ffmpeg publisher per stream,
      using a deterministic `testsrc2` clip;
    - an isolated lightNVR from `fixture-server.mjs` on ext4/NVMe, never
      tmpfs;
    - continuous recording with 10 s segments;
    - the SOD detector `tiny20.sod` every 1 s on each stream.
    It then samples a 300 s window after 30 s of warm-up and writes the
    results JSON (host, build, workload, latency, gaps, resources, raw
    samples).
  - **Recording-path latency proxies.** No packet-write latency metric
    exists, so the harness uses two black-box proxies:
    - `segment_interval_ms`, from segment-file creation intervals seen by
      inotify;
    - `db_probe_ms`, timing `GET /api/recordings?limit=1` with a session
      cookie, which takes the global DB mutex.
  - **Gaps** come from three sources: the `lightnvr_recording_gaps_total`
    delta, the catalog-row gaps, and interval gaps.
  - **Pure helpers** are in `metrics.mjs`, covered by 10 node:test cases
    (all passing). `compare.mjs repeat|gate` implements the C4 and X2
    checks.
  - **Baseline**, run twice (feature disabled, 4×720p at 15 fps):
    - 0 gaps in every source.
    - 120/120 segments.
    - Detector at 1.003 inferences per stream per second.
    - CPU about 145% of one core.
    - RSS high-water about 875 MB.
    - Disk writes 2.33 MB/s.
  - **Repeatability:**
    - `segment_interval_ms` p95 was 10035.3 / 10037.9 ms (0.03%, pass).
    - `db_probe_ms` p95 was 11.95 / 12.32 ms (**3.02%, fail**). The host was
      shared with parallel agents, and the load average reached 9.25 during
      run 2.
- **gotchas / handoff to X2**:
  - Synthetic content produces no VOC detections, so no observations would
    be created. X2 must use a detection-producing clip (`--source`), and must
    rerun disabled mode on that same clip and the same binary before running
    `compare.mjs gate`.
  - Enable the feature only through `--stream-extra` and `--config-extra`
    (the new `fixture-server` `extraConfig` option). Scrape the L10
    queue/drop/enqueue histograms into `results.feature_metrics`.
  - Re-check DB-probe variance on a quiet host with a 600 s or longer window.
    If it stays above 2%, gate only on segment interval plus gaps and record
    that in C1.
  - The go2rtc `exec:` lazy sources raced the first recorder/detector
    connect, which is why the harness uses always-on publishers.
  - Basic auth runs PBKDF2 per request, which is why the harness uses a
    session cookie.
  - The local build needs `PKG_CONFIG_LIBDIR` to point at the system FFmpeg
    because of the stale `/usr/local` headers.
  - The binary was built from `c46bff1c`. The `build.revision` field reads
    `3c61e235` because docs-only commits landed between the build and the
    run.
  - C1 should link `tests/performance/observations/README.md` from
    §Performance. C4 did not edit `OBSERVATIONS_CONTRACT.md`.
- **files edited/created**: `tests/performance/observations/{run-workload.mjs,metrics.mjs,compare.mjs,workload.json,README.md}`,
  `tests/performance/observations/results/2026-11-08-disabled-{run1,run2,repeatability}.json`,
  `tests/contracts/tooling/observation-perf-metrics.test.mjs`,
  `tests/contracts/fixture-server.mjs` (optional `extraConfig`)

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
- **status**: Completed
- **log**:
  - RED: `test_observation_codec` and `test_db_observations` failed to link
    (every new symbol undefined). `test_db_streams` failed 2/42 on the new
    evidence-setting tests ("Expected 7 Was 0"). GREEN: codec 6/6,
    observations 15/15 (stable over repeated runs), streams 42/42, `test_uuid`
    3/3, in `build-tls` and `build-tls-asan` (ASan clean). The
    `db_|stream|config|migration|uuid|observation|schema|embedded|api_handlers|event_|detection`
    ctest subset passes 94/94. The full suite fails only the known
    FFmpeg/SOD/audio tests (stale local headers).
  - Run: `cd build-tls && make test_db_observations test_observation_codec && ./bin/test_db_observations && ./bin/test_observation_codec`.
  - API for L3/L4/L5/L8/L13 (`include/database/db_observations.h`). Results
    are `db_obs_result_t`: `OK`, `DUPLICATE`, `CAS_LOST`, `ERROR`, `INVALID`,
    `NOT_FOUND`, `INVALID_CURSOR`, `CURSOR_EXPIRED` and `CONFLICT`.
    - `db_observation_insert_batch(const observation_batch_input_t*, const observation_asset_input_t*, observation_insert_result_t*)`.
      A retry of the same `batch_id` returns `DUPLICATE` with the existing
      IDs and writes nothing. A reused `batch_id` on another camera returns
      `CONFLICT`. An unknown `camera_uuid` returns `NOT_FOUND`. The stream
      name defaults to the stream row's name.
    - `db_observation_asset_transition(asset_uuid, from, to, reason, const observation_asset_media_t*, observation_transition_result_t*)`:
      a compare-and-set that returns `CAS_LOST`. The allowed edges are
      pending→ready, pending→unavailable(`encode_failed|write_failed|interrupted|disabled`),
      ready→unavailable(`write_failed`) and ready→expired(`retention_expired|deleted`).
      Insert-only reasons (`queue_full`, `budget_exhausted`,
      `no_matching_media`) are rejected as transitions.
    - `db_observation_bump_revision_locked(sqlite3*, source, observation_uuid, cause, now_ms, &seq, &revision)`
      is for L13. The caller holds `get_db_mutex()` inside `BEGIN IMMEDIATE`.
      It snapshots the current asset state into the change row.
    - `db_observation_get`, `db_observation_asset_get`, and
      `db_observations_list(const observation_list_query_t*, cursor, observation_page_t*)`
      (free it with `db_observation_page_free`). The query takes an already
      authorized camera set (`camera_uuids`/`camera_count`, or `all_cameras`),
      up to 8 labels, `since`/`until` and `limit` (0 means 100; 1..500). The
      cursor is NULL, `"head"` or opaque. On `CURSOR_EXPIRED`, the page
      carries `oldest_cursor` and `oldest_observed_at_ms`.
    - `db_observation_meta_get(observation_meta_t*)` returns the epoch,
      `oldest_seq` and `head_seq`.
      `db_observation_meta_regenerate_epoch()` is for D9 (L4/L11). It is not
      yet wired into the `db_core.c` restore paths.
    - `db_observation_pending_publications(observation_publication_t*, max)`
      returns a count. Each entry carries everything the L5 envelope needs:
      message_id, cause, the state and reason at that revision,
      camera/label/observed_at, asset and the change `created_at_ms`.
      `db_observation_publication_mark(seq, state, last_error)` moves a row
      out of pending. Passing `PENDING` records a retry. Every call
      increments `attempts`, and a row that is no longer pending returns
      `CAS_LOST`.
    - `db_observations_prune(before_ms, batch, observation_prune_result_t*)`.
  - Helpers:
    - `lightnvr_uuid_generate_v5()` (`utils/uuid.h`, mbedTLS `md` SHA-1).
      `observation_message_id()` matches the §5.4 vector.
    - `base64url_encode()` and the strict `base64url_decode()`
      (`utils/base64.h`).
    - `observation_cursor_encode()` / `observation_cursor_decode()` match the
      §7.4 example.
    - `observation_reason_is_valid()` implements the §4.3 enum.
  - `stream_config_t` gains `observation_evidence_enabled`,
    `observation_labels[512]`, `observation_max_age_days` and `int64_t
    observation_max_bytes`. All three save paths and both load paths in
    `db_streams.c` handle them. Budgets of 0 or less are saved and loaded as
    7 days / 2 GiB, so a zeroed config cannot violate the 0095 CHECKs.
    `config.c` defaults are set too.
  - Gotchas:
    - `event_identity_init()` must run before any mutation, because message
      IDs need the source. The source is read before the DB mutex is taken;
      `event_identity_init` holds its own mutex while calling
      `db_get_system_setting`.
    - Reads (list, get, pending publications, meta) open a private read-only
      connection in one read transaction (one WAL snapshot) and never take
      the global mutex. The watermark and scan are therefore consistent.
    - Observations whose capture camera has no `streams` row are excluded
      from list and get. `streams.camera_uuid` is immutable (trigger), so a
      soft-deleted (disabled) camera stays visible.
    - Prune deletes asset rows only when they are unreferenced and
      `unavailable`/`expired`. An unreferenced `ready`/`pending` asset keeps
      its row, so L4 must delete the file, expire the asset, and then prune
      or delete it.
    - Expiry keeps `path`, `bytes`, `width` and `height` on the asset row.
      `path` is internal and must never be serialized (L8).
    - Boxes are rounded to 6 decimals at insert. Clamping is the producer's
      job (L6/L7); out-of-range boxes are rejected with `INVALID`.
    - The rollback test injects its failure with a TEMP trigger on
      `observation_publications`, so production code has no test hook.
- **files edited/created**: `include/database/db_observations.h`, `src/database/db_observations.c`, `include/utils/uuid.h`, `src/utils/uuid.c`, `include/utils/base64.h`, `src/utils/base64.c`, `include/core/config.h`, `src/core/config.c`, `src/database/db_streams.c`, `tests/unit/test_db_observations.c`, `tests/unit/test_observation_codec.c`, `tests/unit/test_db_streams.c`, `tests/unit/CMakeLists.txt` (commit `49bda76e`)

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
- **status**: Completed
- **log**:
  - RED: `test_observation_evidence` failed to link (every
    `observation_evidence_*` symbol undefined). GREEN: 14/14, stable over 8
    runs, in `build-int01-L3` and `build-int01-L3-asan` (ASan/UBSan/LSan
    clean). `test_db_observations`, `test_observation_retention`,
    `test_observation_relay` and `test_shutdown_coordinator` also pass under
    ASan.
  - Run: `cd build-int01-L3 && make test_observation_evidence && ./bin/test_observation_evidence`.
  - The local build needs the apt FFmpeg headers. With the stale
    `/usr/local/include` headers, `jpeg_encoder_cache_create` fails ("Failed to
    open MJPEG codec for cache: Invalid argument"), every asset correctly ends
    `unavailable(encode_failed)`, and the ready/JPEG tests fail. The L3 build
    dirs put a symlink dir of `/usr/include/x86_64-linux-gnu/libav*` first via
    `CMAKE_C_FLAGS=-I<dir>`. CI is unaffected.
  - API (`include/core/observation_evidence.h`):
    - `observation_evidence_init(const observation_evidence_config_t*)`
      records the storage root and bounds only. Defaults: 64 items, 32 MiB
      global (`observation_global_queue_bytes`, read in `main.c`), 8 per
      camera, per-camera bytes = global.
    - L6: `observation_evidence_submit(const observation_batch_input_t*, uint8_t *rgb24, int w, int h, const observation_evidence_transform_t*, char asset_uuid_out[37])`.
      The buffer must carry `RGB24_SWS_TAIL_PADDING` slack, as
      `ffmpeg_frame_to_rgb24_padded()` returns it. Ownership is taken on
      **every** return.
    - L7: `observation_evidence_submit_jpeg(batch, uint8_t *jpeg, size_t size, int w, int h, transform, asset_uuid_out)`
      stores the exact bytes with origin `submitted_image`.
      `observation_evidence_submit_unavailable(batch, "no_matching_media", asset_uuid_out)`
      records ONVIF/external batches through the drop ring, so the caller
      never takes the DB mutex.
    - Results are `ACCEPTED`, `DROPPED_QUEUE_FULL`, `DROPPED_BUDGET`,
      `DISABLED` (not initialized or stopping; nothing recorded) and
      `INVALID`. The producer builds the batch (`batch_id`, `camera_uuid`,
      objects) and the worker inserts it. Do not call
      `db_observation_insert_batch` yourself.
    - L4: `observation_evidence_set_budget_exhausted(camera_uuid, bool)` and
      `observation_evidence_is_budget_exhausted()`. This is static memory, so
      it works before init and allocates nothing. Its signature matches L4's
      weak declaration.
    - L10: `observation_evidence_get_stats(observation_evidence_stats_t*)`
      covers accepted, dropped
      (`queue_full`/`budget`/`disabled`/`untracked`), ready, failures
      (encode/write/db), `disabled_at_encode`, interrupted, `bytes_written`,
      backlog items/bytes, drop-ring depth, high-water items/bytes and
      `budget_exhausted_cameras`. `allocated` and `worker_running` back the
      "no queue when disabled" check. Counters reset on init.
    - Helpers: `observation_evidence_transform_json()` (the §4.4 JSON) and
      `observation_evidence_relative_path()`.
      `observation_evidence_set_enabled_probe()` and the stock
      `observation_evidence_stream_enabled_probe` (installed by `main.c`)
      read `streams.observation_evidence_enabled` before encoding and record
      `unavailable(disabled)` when it is off.
  - Path layout: `observations/<camera_uuid>/<yyyy-mm-dd>/<asset_uuid>.jpg`,
    relative to `config.storage_path` and stored in `observation_assets.path`.
    The date is the **UTC** date of `observed_at_ms`. The temp file is
    `<final>.tmp`, files are 0640 and directories 0750. This matches L4's
    reconciler.
  - Design:
    - Admission uses atomic reservations, a Treiber stack (one consumer takes
      the whole list with a single exchange, so there is no ABA) and a
      preallocated 256-cell Vyukov ring for drop records. Wakeups use
      `sem_post`.
    - A producer can touch only two mutexes: the slot-registration lock (the
      first submit for a camera) and a trylock on lazy start. The worker holds
      neither.
    - The worker owns a private `jpeg_encoder_cache_create` encoder, because
      the global `jpeg_encoder_get_cached` table can evict an encoder another
      thread is using.
    - An item that is being processed still counts against the bounds.
  - Gotchas:
    - `main.c` order: L4 reconcile → L3 init → L5 relay start → detection.
      Shutdown: detection → L5 relay stop → L3 shutdown (2 s drain) → event
      bus/identity. Publications the worker writes during the drain are
      relayed at the next start.
    - Abandoned or never-encoded items are inserted directly as
      `unavailable(interrupted)` (or `disabled`), not pending→X. L2 allows
      these as initial states. Their origin, media type and transform are
      NULL (no media).
    - If the `pending→ready` commit fails after the rename, the final file is
      unlinked, so a ready file never sits behind a non-ready row.
    - `expires_at_ms` is not set on ready. L4/L8 derive it from the
      per-camera age budget.
    - Camera slots are never recycled (`MAX_STREAMS` of them). Past that, a
      camera is bounded by the global limits only, and its budget flag cannot
      be stored (logged).
- **files edited/created**: `include/core/observation_evidence.h`, `src/core/observation_evidence.c`, `tests/unit/test_observation_evidence.c`, `tests/unit/CMakeLists.txt`, `src/core/main.c` (commit `5247c8ab`)

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
- **status**: Completed
- **log**:
  - RED: `test_observation_retention` failed to compile without
    `core/observation_retention.h`, then failed to link with every new
    symbol undefined once the header existed. GREEN 8/8 in `build-int01-L4`
    and `build-int01-L4-asan` (ASan/UBSan clean, 3 repeated runs);
    `test_db_observations` still passes 15/15. In the
    `storage|db_|migration|observation|retention|backup|schema|stream|uuid|event_`
    ctest subset, three tests fail, none from L4:
    - `test_storage_reconcile_incomplete` segfaults inside FFmpeg mpeg4. The
      older `build-tls` binary segfaults the same way (stale local headers).
    - `test_api_handlers_event_routes` expects 13 event types and finds 14.
      That count comes from L5's uncommitted event type.
    - `test_api_handlers_observations` and `test_observation_evidence` are
      L8's and L3's in-progress tests.
  - Run: `cd build-int01-L4 && make test_observation_retention && ./bin/test_observation_retention`.
  - API (`include/core/observation_retention.h`):
    - `observation_retention_startup(storage_path, &result)`: the startup
      sweep. Returns counts for temp files deleted, stale files deleted,
      pending assets interrupted, ready assets with missing files, and
      whether the epoch was regenerated.
    - `observation_retention_startup_done()`.
    - `observation_retention_run(storage_path, now_ms, &result)`: one
      bounded pass. Returns 1 if startup has not run yet.
    - `observation_retention_maintenance_tick()`: the storage-manager hook.
    - `observation_retention_budget_exhausted(camera_uuid)`: non-blocking.
    - `observation_retention_budget_exhausted_count()`.
    - `observation_retention_camera_status(out, max)`: for L10. Returns
      retained bytes, max bytes, max age and the flag per camera, as of the
      last pass.
    - `observation_cursor_head_write/read/reconcile()`: the D9 sidecar.
  - Hooks:
    - `main.c` calls `observation_retention_startup()` right after "Storage
      manager initialized", under the
      `/* INT_01 startup order: reconcile (L4) -> evidence worker (L3) -> relay (L5) */`
      marker. That is after `event_identity_init()` and before L3, L5 and
      detection start.
    - `storage_manager.c` calls `observation_retention_maintenance_tick()`
      on every heartbeat (60 s). The tick runs a pass at most every 300 s,
      or on the next heartbeat when the previous pass hit a bound. The
      storage thread starts inside `init_storage_manager()`, before
      reconciliation, so the tick is a no-op until startup has completed.
    - `db_core.c` regenerates `observation_meta.epoch_uuid` after either
      `restore_database_from_backup()` call succeeds, once migrations have
      run. This closes the gap L2 noted.
  - Semantics chosen where the contract is loose:
    - An asset's age is the newest `observed_at_ms` among the observations
      that reference it, falling back to `captured_at` and then
      `created_at`. So a reference expires at its own media age, not at the
      30-day history prune, and a shared asset survives until its last
      reference expires (A3).
    - Byte pressure counts only `ready` bytes, protected ones included. It
      evicts the oldest unprotected assets first.
    - The flag is set only when a camera is still over budget and has no
      unprotected `ready` media left. If a pass bound stopped eviction, the
      flag is not set and `more_work` is set instead.
    - Bounds per pass: 256 evictions and 5,000 pruned change rows, pruned
      500 per transaction.
    - Expiry runs the CAS `ready→expired(retention_expired)` first, then
      unlinks the file. A lost CAS (for example a racing L8 delete) leaves
      the file to the winner. A crash between the two steps leaves a stale
      file, which the next startup sweep deletes.
    - Assets whose observations were all pruned while `ready` are reclaimed:
      expire, unlink, delete the row. L2's prune keeps those rows. Protected
      unreferenced assets are kept.
    - Startup also handles a pending asset whose final file exists (crash
      after rename, before the `ready` commit). It becomes `interrupted` and
      the file is deleted, because the transform and dimensions are only in
      worker memory.
    - A `ready` asset whose path resolves outside the evidence tree is
      treated as missing (`write_failed`).
  - Never touches recordings:
    - Paths resolve only to `<storage>/observations/...*.jpg`. Absolute
      paths, `observations/...` and paths relative to the evidence tree are
      accepted. Any `.`, `..` or empty segment is refused.
    - The sweep uses `lstat`, skips symlinks, and only deletes `*.jpg.tmp`
      and `<uuid>.jpg`.
    - Tested with a stored path that escapes the tree into `recordings/`.
  - Budget flag:
    - L3's header declares `observation_evidence_set_budget_exhausted()`,
      but its `.c` is not committed yet. L4 calls it through a weak
      reference on every pass, sending set and clear changes.
    - That call is a no-op until L3's object is linked, and needs no code
      change once it is. In a static archive, the weak reference does not
      pull the object in itself; L3's own `main.c` wiring does.
  - Overlap with L5 (D9 sidecar):
    - L5's uncommitted `observation_relay.c` also implements the sidecar
      check and writer, with the same path and the same
      `{"epoch","head_seq"}` format, so the two interoperate.
    - The check must happen in L4, because the reconciliation transitions
      advance the DB head and would hide a rewind from a later check. L4
      checks first and rewrites the sidecar after the sweep, so L5's
      startup check is then a harmless no-op.
    - Whichever task lands second could remove the duplicate.
  - Gotchas:
    - `db_core.c` regenerates the epoch with raw SQL on the init connection
      instead of calling `db_observation_meta_regenerate_epoch()`.
      `rebuild_recordings` links a hand-picked source list without
      `db_observations.c` and its event-identity dependencies.
    - The restore path has no unit test. L11's restore test covers it.
    - Startup returns -1 and leaves periodic eviction off when a DB read or
      transition fails, for example when `event_identity_init()` failed.
- **files edited/created**: `include/core/observation_retention.h`, `src/core/observation_retention.c`, `tests/unit/test_observation_retention.c`, `tests/unit/CMakeLists.txt`, `src/core/main.c`, `src/storage/storage_manager.c`, `src/database/db_core.c` (commit `7373d598`)

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
- **status**: Completed
- **log**:
  - Commit `442ab71b`. Built in `build-int01-L5` (MQTT+SOD, mirrors
    `build-tls`) and `build-int01-L5-asan` (Debug ASan/UBSan, mirrors
    `build-tls-asan`).
  - RED: `test_observation_relay` failed to link (every relay symbol and
    `event_producer_enqueue_durable` undefined). `test_event_envelope` failed
    3/13: registry count 13≠14, registry entry missing, and fixtures
    rejected with "event type is not registered".
  - GREEN in both builds, ASan clean: `test_observation_relay` 12/12 (stable
    over 5 runs), `test_event_envelope` 13/13, `test_event_outbox`,
    `test_event_router`, `test_event_bus`, `test_event_detection_adapter`,
    `test_mqtt_delivery_worker`, `test_event_route_suppression`,
    `test_db_event_routes`, `test_db_observations` 15/15 and
    `test_api_handlers_event_routes` 5/5. Its catalog count was bumped to
    14. ctest subset
    `db_|stream|config|migration|uuid|observation|schema|embedded|api_handlers|event_|detection|mqtt`:
    98/98, excluding the in-progress L3/L8 tests. Node contract tooling:
    59/59.
  - Run: `cd build-int01-L5 && make test_observation_relay test_event_envelope && ./bin/test_observation_relay && ./bin/test_event_envelope`.
  - **API** (`include/core/observation_relay.h`):
    - `observation_relay_start(const observation_relay_options_t*)` (NULL
      means defaults). It runs the D9 check, does a bounded synchronous
      drain (≤5 s), then starts the thread. It is idempotent.
      `observation_relay_stop()` leaves rows pending.
    - **Notify hook**: `observation_relay_notify()`. Start registers it
      through the new `db_observations_set_publication_hook()`, so
      `db_observation_insert_batch` and `db_observation_asset_transition`
      wake the relay automatically after commit (L3 and L4 need nothing).
      **L13** uses `db_observation_bump_revision_locked` inside its own
      transaction, so it must call `observation_relay_notify()` after its
      COMMIT. Otherwise the change waits for the 1 s poll.
    - **Stats for L10**: `observation_relay_get_stats(observation_relay_stats_t*)`.
      Fields: `running`, `passes`, `relayed`, `duplicates`,
      `relayed_no_route`, `expired`, `failed`, `enqueue_full`,
      `enqueue_errors`, `mark_errors`, `read_errors`, `cap_pauses`,
      `backoffs`, `sidecar_writes`/`_errors`, `epoch_regenerations`,
      `last_relayed_seq`, `oldest_pending_created_at_ms` (lag =
      now − this; 0 = none), `last_pass_pending`, `outbox_type_pending`,
      `consecutive_failures` and `backoff_until_ms`. The counts of pending,
      failed and expired publication rows are not in the stats; L10 should
      count them in `observation_publications`.
    - Also exported: `observation_relay_build_envelope(pub, source, &event, ...)`,
      `observation_relay_run_once(opts, &pass)` (one pass, no thread),
      `observation_relay_check_cursor_head()` and
      `observation_relay_write_cursor_head()`. `OBSERVATION_CHANGED_EVENT_TYPE`
      and `OBSERVATION_RELAY_OUTBOX_CAP` (2000) are defined there too.
    - **Wrapper** (`include/core/event_producers.h`):
      `event_producer_enqueue_durable(const event_envelope_t*, err, size)`
      returns `event_producer_enqueue_result_t`: `FAILED`(-1), `ENQUEUED`,
      `DUPLICATE`, `NO_ROUTE`, `MQTT_DISABLED` or `FULL`.
      `event_producer_enqueue_result_name()` gives the name. The old static
      `durably_enqueue_event` now wraps it, keeping its 0/-1 mapping
      (no_route and mqtt_disabled → 0), so the other producers behave
      identically. With routes, all-duplicate → `DUPLICATE`; only the
      disabled default broker matched → `MQTT_DISABLED`; any non-FULL
      failure → `FAILED`.
    - `db_event_outbox_count_undelivered_type(type, &count)` counts
      `pending` + `delivering` rows for D8.
  - **Relay behavior**:
    - It reads pending rows in seq order, up to 64 per pass.
    - FULL, an error, a mark error or the cap stops the batch at that row,
      so later rows never overtake it. FULL and error mark the row
      `PENDING` with `last_error`, which counts an attempt. A cap pause
      does not count one.
    - Backoff is 1 s doubling to 30 s, and notifies are ignored while
      backing off. A full batch loops immediately.
    - A row past the 3600 s delivery expiry (measured from the change's
      `created_at_ms`) → `expired`. An invalid envelope → `failed`, and the
      batch continues.
    - `event_bus_publish` runs only after a successful mark. A
      `CAS_LOST`/`NOT_FOUND` mark is skipped silently.
    - Envelope `time` = `change_created_at_ms / 1000`. `observed_at` is
      formatted `YYYY-MM-DDTHH:MM:SS.mmmZ`. Generated envelopes equal all
      8 C3 fixtures byte for byte (`cJSON_PrintUnformatted`).
  - **D9 (owned here)**:
    - The sidecar `<storage>/observations/.cursor_head.json` is
      `{"epoch","head_seq"}`, written via tmp + fsync + rename + dir fsync.
      It is written at most once per second, after passes that saw
      pending rows.
    - At start, if the sidecar epoch equals the DB epoch and
      `head_seq > DB head`, the relay calls
      `db_observation_meta_regenerate_epoch()`. A missing or other-epoch
      sidecar is rewritten without regenerating.
    - The in-process `restore_database_from_backup()` call sites in
      `db_core.c` are still not wired; they remain for L11 or a follow-up.
  - **Gotchas**:
    - **L4**: orphan-file deletion must skip `.cursor_head.json` and
      `.cursor_head.json.tmp` in `<storage>/observations/`, which is not a
      camera directory.
    - **Bus publish re-runs routes**: the MQTT compatibility subscriber
      re-evaluates routes and hits outbox `DUPLICATE`, the same as for
      system-health transitions. An outbox row pruned in between can
      therefore redeliver, which at-least-once allows.
    - **Detection predicates**: route predicates with
      `detection.labels_any` and similar only match
      `detection.object.v1`. A route that lists both types with a label
      predicate never matches observation events, which are then recorded
      as `relayed_no_route`.
    - **No `event_type` index**: `event_outbox` has none, so the D8 count
      scans at most about 10k rows. It runs only in passes that have
      pending work.
    - **Validator layering**: the validator uses
      `observation_reason_is_valid`/`observation_asset_state_parse` from
      `db_observations.c`, so `event_envelope.c` now includes
      `database/db_observations.h`. Both are in `lightnvr_lib`.
    - **Start placement in `main.c`**: right after the L4 reconcile block.
      L3's worker start must go between them. Relay stop comes before
      `event_bus_shutdown` on both shutdown paths.
- **files edited/created**: `include/core/observation_relay.h` (new), `src/core/observation_relay.c` (new), `tests/unit/test_observation_relay.c` (new), `src/core/event_envelope.c`, `include/core/event_producers.h`, `src/core/event_producers.c`, `include/database/db_event_outbox.h`, `src/database/db_event_outbox.c`, `include/database/db_observations.h`, `src/database/db_observations.c` (post-commit hook only), `src/core/main.c`, `tests/unit/test_event_envelope.c`, `tests/unit/test_api_handlers_event_routes.c`, `tests/unit/CMakeLists.txt`, `docs/EVENT_CONTRACT.md`

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
- **status**: Completed
- **log**:
  - RED: `test_detection_evidence` failed to link (every
    `detection_evidence_*` symbol undefined). `test_detection_observation_evidence`
    failed 3/5 ("Expected 2 Was 0", "Expected 4 Was 0": no observations from
    the UDT). GREEN: 8/8 and 5/5 in `build-int01-L6` (LiteRT on) and
    `build-int01-L6-asan` (ASan/UBSan clean). `test_detection_*`,
    `test_udt_recording`, `test_api_detection`, `test_db_detections`,
    `test_event_detection_adapter`, `test_observation_evidence`,
    `test_db_observations`, `test_db_streams` and `tests/test_stream_detection`
    pass in both. Failing for other reasons: `test_sod_unified` and
    `test_sod_voc` (pre-existing), plus `test_observation_relay` and
    `test_observation_diagnostics`, which come from another task's
    uncommitted edits.
  - Run: `cd build-int01-L6 && make test_detection_evidence test_detection_observation_evidence && ./bin/test_detection_evidence && ./bin/test_detection_observation_evidence`.
    The build dirs reuse L3's apt-FFmpeg header symlink dir via `CMAKE_C_FLAGS=-I…`.
    The ASan dir needs `-DENABLE_LITERT=OFF`, because xnnpack does not build
    under Debug/ASan.
  - Integration points (`src/video/unified_detection_thread.c`):
    - `udt_evidence_wanted` / `udt_submit_evidence` (around line 3063) are
      shared by both paths.
    - Single engine: `detect_on_decoded_frame` keeps the embedded-model RGB
      buffer (`evidence_rgb`, around line 3352) only when evidence is enabled.
      It submits after the threshold compaction (around line 3390).
    - Multi-engine: `detect_with_runtime_engines` records each engine's slice
      of `combined` and submits one batch per object engine, because a batch
      has a single detector provenance. The last engine gets the shared
      buffer; earlier ones get a padded copy. Motion slices are never
      submitted.
    - Detection stream: the background thread already calls
      `detect_on_decoded_frame`, so capture happens on the inference thread
      at the detection stream's resolution. The result slot still carries
      only `detection_result_t`.
    - The API branch is untouched (L7).
  - Settings: the ctx caches `observation_evidence_enabled` (atomic) and
    `observation_labels` (under `ctx->mutex`) at thread start.
    `unified_detection_set_observation_settings(stream_name, enabled, labels)`
    refreshes them and is called from `put_stream_worker_locked`
    (`api_handlers_streams_modify.c`). Nothing in this feature reads the DB
    on the detection/packet threads. `recording_id` comes from
    `detection_link_recording_id` (in-memory).
  - Sidecar API (`include/video/detection_evidence.h`):
    - `detection_evidence_build_batch`: clamped boxes (degenerate → null box),
      confidence clamped to [0,1], `object_index` = position among eligible
      objects, model basename, track only for `track_id >= 0` with
      namespace = engine key and session = `detection_evidence_boot_session()`
      (UUIDv4 per process).
    - `detection_evidence_label_eligible`: case-insensitive, trims blanks;
      empty means all labels.
    - `detection_evidence_transform_compute` and
      `detection_evidence_model_geometry`:
      - SOD is `resize` to the network size, via new `sod_model_input_size()`.
      - LiteRT is `letterbox`, mirroring `letterbox_rgb24()`.
      - RealNet or unknown is `none`.
      - The stored image is always the full decoded frame, and boxes are
        already normalized to it.
    - `detection_evidence_observed_at_ms`: wall-clock ms when still inside
      the frame's second, else `frame_ts * 1000`.
    - `detection_evidence_capture`: takes ownership of the RGB buffer on every
      return.
  - For L7: reuse `detection_evidence_build_batch` (pass
    `engine_type = "api"`). Do not call `detection_evidence_capture`, which
    accepts only `object`. Submit with `observation_evidence_submit_jpeg`, and
    for ONVIF use `observation_evidence_submit_unavailable`. Reuse
    `detection_evidence_boot_session()` for tracks. The single-engine API
    branch of `detect_on_decoded_frame` and the "API engines isolated"
    branch of `detect_with_runtime_engines` are where L7's signature changes
    land.
  - Gotchas:
    - Unity has no double support, so tests use a local `ASSERT_NEAR`.
    - `test_udt_recording` wraps `lightnvr_uuid_generate_v4`. Evidence is off
      there, so batch IDs are never generated.
- **files edited/created**: `include/video/detection_evidence.h`, `src/video/detection_evidence.c`, `include/video/sod_detection.h`, `src/video/sod_detection.c`, `include/video/unified_detection_thread.h`, `src/video/unified_detection_thread.c`, `src/web/api_handlers_streams_modify.c`, `tests/unit/test_detection_evidence.c`, `tests/unit/test_detection_observation_evidence.c`, `tests/unit/CMakeLists.txt` (commit `7c7082b6`)

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
- **status**: Completed
- **log**:
  - RED: against the pre-L7 sources, `test_api_detection`,
    `test_api_detection_observation_evidence` and
    `test_detection_current_frame` failed to compile (every new type and
    function undeclared). Mutation check: with the evidence hand-off and
    geometry mapping disabled, the new integration test failed 3/7 ("no
    ready asset", "Expected 1 Was 0", "Expected 0.4 Was 256"). GREEN:
    `test_api_detection` 27/27 and `test_api_detection_observation_evidence`
    7/7 (stable over 3 runs) in `build-int01-L7` (LiteRT on) and
    `build-int01-L7-asan` (ASan/UBSan clean). `test_detection_current_frame`,
    `test_detection_observation_evidence`, `test_detection_evidence`,
    `test_udt_recording`, `test_detection_system_onvif`,
    `test_onvif_poll_recovery`, `test_observation_evidence`,
    `test_db_detections`, `test_event_detection_adapter`,
    `test_detection_io_recovery` and `tests/test_stream_detection` pass in both.
  - Run: `cd build-int01-L7 && make test_api_detection_observation_evidence && ./bin/test_api_detection_observation_evidence`.
    The test runs a real loopback HTTP detector (multipart and DOODS2), a
    real migrated DB and the real L3 worker. Build the targets one at a
    time: a single multi-target `make -jN` races in this generator.
  - Signatures (`include/video/api_detection.h`):
    - `detect_objects_api_with_evidence(…, options, const api_detection_evidence_t *)`
      and `detect_objects_api_snapshot_with_evidence(…)`. The old
      `*_with_options` functions wrap them with NULL, so
      `detection.c`/`api_handlers` callers are unchanged.
      `api_detection_evidence_t` = `{camera_uuid, engine_key,
      evidence_enabled, labels_csv, observed_at_ms}`, built by the caller
      from the ctx cache. `observed_at_ms` 0 = stamped at call start, before
      the request, never after.
    - `finalize_detections` (static) now takes threshold, options, evidence,
      response meta and an owned `submitted_image_t`. With a `camera_uuid`
      it uses `store_detections_in_db_for_camera` and
      `event_producer_publish_detection`; without one it keeps the legacy
      stream-name calls.
    - `api_detection_parse_response_meta()` (the old
      `api_detection_parse_response` wraps it),
      `api_detection_apply_response_geometry()` and
      `api_detection_jpeg_dimensions()` (SOFn parse for go2rtc snapshots).
    - `include/video/detection_evidence.h`:
      `detection_evidence_capture_jpeg()` (engine_type `api`; takes the
      JPEG on every return), `detection_evidence_record_no_media()`
      (`onvif`/`external`; skips the `motion` label; `keep_boxes=false`
      means box null) and `detection_evidence_is_motion_label()`.
  - Letterbox extension (contract §4.4, "unless the response declares a
    server-side letterbox"). No existing server sends it; it is opt-in and
    documented in the header. The optional top-level `"letterbox":{width,
    height, scale | scale_x/scale_y, pad_x, pad_y, box_units:"normalized"|"pixels"}`
    sits next to `"model"` and `"model_version"`. Missing scale means the
    uniform min ratio; missing pads mean centred. Boxes are mapped onto the
    submitted image **before** zone filtering, so the detections table and
    MQTT also get stored-image boxes. Without the declaration nothing
    changes (mode `none`, inference = JPEG size). The C3 fixture D test
    reads `manifest.json`'s `inference_box` and
    `observations/http-letterbox.json`.
  - Evidence objects are the zone/object-filtered survivors with
    `confidence >= threshold`, the same set the detection thread keeps.
    Model: response `model`, else the DOODS2 `detector_name` unless it is
    `"default"`, else null (the URL is never used as a model). The
    transform source is the decoded frame for the frame path; for a go2rtc
    snapshot the JPEG is its own source.
  - ONVIF: `udt_record_onvif_observations()` is called from
    `onvif_detection_thread_func` after a successful poll. `observed_at` is
    the wall time before `PullMessages`, matching `detect_motion_onvif`'s
    event stamp. The engine key is the first `onvif` engine, else
    `legacy-primary`. Settings come from the ctx cache and the record goes
    through the drop ring, so the ONVIF thread never touches the DB mutex.
    `onvif_detection.c` already classifies object events (person, vehicle,
    bicycle, face, animal, else `motion`), so no change was needed there.
  - Gaps:
    - Multi-engine API engines are still isolated in
      `detect_with_runtime_engines`: they never run, so they produce no
      evidence. A comment there records this.
    - No external-event producer exists. `store_detection_result` has no
      callers, so `detection_evidence_record_no_media` with `external` is
      ready but not wired.
    - The ONVIF tests call `udt_record_onvif_observations` directly rather
      than through the polling thread.
  - For L11/X1/X2:
    - The A2 HTTP check is in place: the asset bytes equal the request body
      image, and the server sees exactly one request.
    - A server-declared letterbox changes the boxes on legacy detection rows
      too.
    - The fake server sniffs `Content-Type: application/json`, not just
      `application/json`, because lightNVR sends `accept: application/json`.
    - The API path holds the JPEG for the whole request, up to the 10 s
      timeout. That adds one JPEG of memory per in-flight API call. It is
      not counted against the L3 bounds until it is submitted.
- **files edited/created**: `include/video/api_detection.h`, `src/video/api_detection.c`, `include/video/detection_evidence.h`, `src/video/detection_evidence.c`, `src/video/unified_detection_thread.c`, `tests/unit/test_api_detection.c`, `tests/unit/test_api_detection_observation_evidence.c` (new), `tests/unit/test_detection_current_frame.c`, `tests/unit/CMakeLists.txt` (commit `9830adb6`)

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
- **status**: Completed
- **log**:
  - Commit `2dfe559f`. The four GET stubs are replaced; the PUT stays `501`
    (L14). Handler names and routes are unchanged, so `coverage.json` did not
    move (`coverage.mjs --check` OK). The `501` response is removed from the
    four GET operations in OpenAPI, the projections are regenerated, and
    `observations-contract.test.mjs` now expects `501` only on the PUT.
  - **Behaviour**:
    - **Authorization** uses `httpd_evaluate_camera_identity_action` on the
      capture-time `camera_uuid` for get and snapshot, so unknown,
      deleted-camera and unauthorized IDs are byte-identical
      `404 observation_not_found`. List and capabilities use
      `db_fleet_camera_load_readonly` and `authorization_filter_cameras`.
    - **List**: D1 comma lists (≤10 cameras, lowercased; ≤8 labels of
      1..31 bytes), RFC 3339 `since`/`until` (`Z` or an offset), `limit`
      1..500 and `cursor`/`head`. An empty or undecodable parameter is
      `400 invalid_request` with `field`. An explicit camera without
      `live.view`, unknown UUIDs included, is `403 forbidden` (audited as a
      denial). Otherwise the scope is the principal's `live.view` set, and an
      empty set gives an empty page. Errors map to `400 invalid_cursor` and
      `410 cursor_expired` with `oldest_cursor`/`oldest_observed_at`.
    - **Snapshot**:
      - `ready` streams the file through `http_serve_file` with
        `image/jpeg`, `Cache-Control: private, max-age=300` and
        `ETag: "<asset_id>"`; `If-None-Match` gives `304`.
      - `pending` is `202` with `Retry-After`; `unavailable` is
        `404 evidence_unavailable` with `reason`; `expired` is
        `410 evidence_expired` with `reason` and `expired_at` (the asset's
        `updated_at_ms`).
      - D4: a denied `snapshot.create` re-checks `live.view` and returns
        `403 forbidden` if it is held, otherwise `404`.
      - A `ready` row whose file is gone re-reads the asset: `410` if it has
        expired meanwhile, else `404 evidence_unavailable(write_failed)`.
        Nothing is substituted.
      - The stored path must be `observations/…/*.jpg` relative to
        `storage_path` (L3's format), or absolute under it, with no `.`/`..`
        segments. Anything else is `500` and is logged.
    - **Capabilities**:
      - `source` comes from `event_identity`, and `product_version` is
        `LIGHTNVR_VERSION_STRING`.
      - history and snapshot are `authorized` when the action is held on any
        camera, else `reason: not_authorized`. `annotate` is
        `supported: false, not_available_in_this_version`.
      - limits come from the `db_observations.h` constants.
        `history.retention_days` is read from `system_settings` (default
        30). `delivery.topic` is `<prefix>/v1/events/<type>/{camera_uuid}`.
      - `cameras` holds at most 1,024 entries, each with `evidence_enabled`
        from the stream config and the actions held.
      - `principal.writer_namespace` is the D3 normalization, or null for an
        invalid or reserved name.
    - **Audit**:
      - Snapshot: every fetch is recorded as `snapshot.create` with
        `target_type "observation"` and `target_uuid` = the observation ID.
        The details are camera_uuid, asset_id, evidence_state and the
        decision source, never a path. Allowed reads follow the per-action
        decision mode: `off` records nothing, `summarize` goes through
        `audit_log_authorization` on the camera (a summary row cannot carry
        an observation ID), and `record` writes the full row.
      - Get: goes through `audit_log_authorization(live.view)` as today.
      - List: audits only explicit-camera denials.
  - **Exported for L14** (`include/web/api_handlers_observations.h`):
    `observation_record_to_json()`, which follows the C3 conventions (pending
    nulls width/height/bytes/expires_at; no `origin` means every media field
    is null; no `path`), plus `observation_format_time()`,
    `observation_parse_time()` and `observation_writer_namespace()`
    (returns `-1` invalid / `-2` reserved).
  - **RED→GREEN**:
    - RED: the extended `test_api_handlers_observations` failed to link (18
      undefined references to the new helpers and serializer).
    - GREEN: 18/18 in `build-int01-L8` and in `build-int01-L8-asan`, ASan and
      LSan clean.
    - The tests run against a migrated DB seeded through the L2 API. They wrap
      `httpd_check_action_access`, `httpd_evaluate_camera_identity_action`,
      `authorization_filter_cameras`, `http_serve_file` and
      `audit_log_append`.
    - Coverage: every status code; the authorization intersection; D4;
      malformed parameters (17 cases plus 11 cameras); paging, `head` and
      expired-cursor resume; ETag/304; path containment; and the missing file.
    - Exact `cJSON_Compare` equality with 8 observation fixtures and with the
      `invalid-cursor`, `observation-not-found`, `snapshot-forbidden` and
      `evidence-unavailable` bodies. Key and type shape matches the page,
      capabilities, `cursor-expired`, `evidence-pending` and
      `evidence-expired` fixtures.
    - With `OBS_RESPONSE_DUMP_DIR` set, all 60 JSON bodies the test produces
      validate against the generated web validators (Observation,
      ObservationPage, ObservationError, IntegrationCapabilities).
  - **Validation**: `node --test tests/contracts/tooling/*.test.mjs` 59/59;
    `coverage.mjs --check` and `generate.mjs --check` OK; `lint.mjs` valid
    with the same 5 pre-existing warnings. `test_api_handlers_openapi` 8/8,
    `test_openapi_projection` 21/21, and the full `lightnvr` target builds.
  - **Gotchas for later tasks**:
    - **L9**: `openapi.json` now has no GET `501`s. Regenerate after your
      schema edits; `coverage.json` only moves if `libuv_api_handlers.c`
      does.
    - **L10**: the setup probe `GET /api/observations?camera_uuid=X&limit=1`
      works as is, and an unauthorized X is `403`, not an empty page.
      `Retry-After` is provisional: 1 s while the asset is under 5 s old,
      else 2 s. Once L3 exposes queue depth, wire `min(30, 2 × ahead)` into
      `retry_after_seconds()`.
    - **L11**: the unit tests wrap authz, so the Playwright suite must prove
      real scoped-token isolation. The DB-restore case surfaces as `410`
      with `oldest_cursor` in the current epoch. Capabilities
      `history.oldest_observed_at` is installation-wide (the same value the
      `410` body carries), not camera-scoped.
    - **L14**: reuse `observation_record_to_json()` and fill the two
      `classifications` lists inside it. Use `observation_writer_namespace()`
      for the D3 check, and flip `observations.annotate` in
      `handle_get_integration_capabilities()`; also add the
      `observations.annotate` action to each `cameras[].actions`. The PUT
      handler currently only authenticates and returns `501`.
    - In unit tests, `--wrap=audit_log_append` does not intercept appends made
      inside `audit_log.c` (such as via `audit_log_authorization`). Those
      write to the real audit table.
    - Concurrency: the L3 agent's plan commit swept in this task's staged
      files. It was split back out by a soft reset of that local, unpushed
      HEAD, so the L3 plan commit is now `0bf32a53`. Other agents should
      stage with explicit paths or `-p` and commit right after staging.
- **files edited/created**:
  - `src/web/api_handlers_observations.c`, `include/web/api_handlers_observations.h`
  - `tests/unit/test_api_handlers_observations.c`, `tests/unit/CMakeLists.txt` (wraps, `OBS_FIXTURE_DIR`, timeout)
  - `docs/api/openapi.json` (GET `501`s removed), generated `include/web/openapi_document_generated.h`, `web/generated/api/schema.d.ts`
  - `tests/contracts/tooling/observations-contract.test.mjs`

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
- **status**: Completed
- **log**:
  - RED: `test_put_stream_launch_failure` failed 5/24 on the new tests
    (fields missing from GET; PUT/POST accepted out-of-range values with
    202/201). The web spec failed on the missing util, then on the
    StreamsView/modal wiring and the en.json keys. GREEN: the C test passes
    24/24; `test_db_streams` 42/42; `test_delete_stream_launch_failure` 7/7;
    `test_stream_manager` 15/15. Web: jest 85 suites / 1018 tests,
    `npm run typecheck` and eslint on the changed files are clean.
    `streams.api.spec.ts` passes 9/9 against `build-int01-L9/bin/lightnvr`.
    `generate.mjs --check`, `lint.mjs`, `coverage.mjs --check` and the
    `tests/contracts/tooling` tests (59/59) pass.
  - Run: `cd build-int01-L9 && make test_put_stream_launch_failure && ./bin/test_put_stream_launch_failure`
    and `cd web && npx jest tests/observationEvidenceSettings.spec.js`.
  - API fields (stream GET list, detail and `/full`; POST and PUT bodies):
    - `observation_evidence_enabled`: boolean, default false.
    - `observation_labels`: a CSV string. Entries are trimmed; at most 16
      entries of 1..31 printable ASCII bytes; empty entries are rejected. An
      empty string means every label. Case is preserved, so matching must be
      ASCII case-insensitive.
    - `observation_max_age_days`: an integer, 1..90 (default 7).
    - `observation_max_bytes`: an integer, 67108864..1099511627776 (default
      2147483648).
  - Server behaviour:
    - Null, a wrong type or a fractional number returns 400 with an
      `observation_*` message, before any task is queued.
    - The validator is shared by POST and PUT
      (`stream_observation_settings_apply_json`), and the GET paths use
      `stream_observation_settings_to_json`.
    - Writes are gated by the existing `camera.configure` check. Reads are
      visible to `live.view`, like the other detection settings.
  - OpenAPI: the fields are optional on `StreamDetailsBase`, because the
    captured baseline payloads predate them. They are typed and bounded on
    `StreamConfigurationUpdate` and `StreamCreateRequest`.
  - UI: the "Observation evidence for integrations" accordion
    (`expandedSections.observation`) has a toggle (off), eligible labels, max
    age in days and a size limit in MiB. It notes that a bird-capable
    detector is required. `web/js/utils/observation-evidence.ts` maps the
    form to and from the wire and validates before save. Strings are in
    `en.json` only; other locales fall back to English.
  - Reload path:
    - A PUT persists through `update_stream_config()` and restarts nothing.
    - `get_stream_config*()` reads SQLite, so the L3 encoder's
      `observation_evidence_stream_enabled_probe` sees the change at once.
    - L6 (uncommitted at the time of L9) caches the flag and labels in the
      detection context at thread start and exposes
      `unified_detection_set_observation_settings(name, enabled, labels)`.
      No caller exists yet. Add this line to `put_stream_worker_locked()`
      after `update_stream_config()` succeeds:
      `unified_detection_set_observation_settings(task->stream_id, task->config.observation_evidence_enabled, task->config.observation_labels);`
      Its -1 return (no running thread) can be ignored.
    - L9 did not add the call, because the symbol was not yet in the tree.
  - Gotcha: when this was written, L6's in-progress `detection_evidence.c`
    in the shared working tree broke the full `lightnvr_lib` build. Build
    the L9 targets before L6 edits land, or after they are committed.
- **files edited/created**: `include/web/stream_observation_settings.h`, `src/web/stream_observation_settings.c`, `src/web/api_handlers_streams_get.c`, `src/web/api_handlers_streams_modify.c`, `tests/unit/test_put_stream_launch_failure.c`, `docs/api/openapi.json`, `include/web/openapi_document_generated.h`, `web/generated/api/schema.d.ts`, `web/generated/api/validators.js`, `web/js/utils/observation-evidence.ts`, `web/js/types/streams.ts`, `web/js/components/preact/StreamsView.tsx`, `web/js/components/preact/StreamConfigModal.tsx`, `web/public/locales/en.json`, `web/tests/observationEvidenceSettings.spec.js` (commit `e365b6b3`)

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
- **status**: Completed
- **log**:
  - RED: `test_observation_diagnostics` failed to link (every
    `observation_diagnostics_*`, `api_metrics_render_observations` and
    `observation_snapshot_retry_after_seconds` undefined). The new relay test
    `test_relay_sidecar_is_the_shared_l4_cursor_head` failed (`Expected 0 Was
    -1`: the relay's own writer could not create a missing directory).
    `test_handle_get_system_info_includes_inactive_observation_pipeline`
    failed with no `observation_pipeline` member. GREEN in `build-int01-L10`
    and `build-int01-L10-asan` (ASan/UBSan clean): diagnostics 5/5, relay
    13/13, retention 8/8, evidence 14/14, observations 25/25, metrics 2/2,
    system 32/32. Web: `npm run typecheck`, eslint, and jest
    `observationPipeline`/`locale-coverage`/`i18nLiteralRule`/`systemHealth`
    (53 passed).
  - Builds ran from a detached worktree of HEAD plus only L10's files, because
    parallel agents' uncommitted `src/video` and test files are globbed into
    `lightnvr_lib` and broke the shared tree's build.
  - **Module** `core/observation_diagnostics.{h,c}`:
    - `observation_diagnostics_collect(&d, now_ms)` reads L3/L5 stats, L4's
      per-camera status and flag count, and the new read-only
      `db_observation_diagnostics_get()`. It never starts the evidence worker.
    - `observation_diagnostics_to_json()` builds the system-info JSON.
    - `observation_diagnostics_relay_lag_seconds()` returns the lag.
    - `observation_diagnostics_free()` releases the snapshot.
  - **DB** `db_observation_diagnostics_get()` (`db_observations.c`) runs one
    WAL read transaction and returns:
    - the meta (oldest/head seq, epoch);
    - `MIN(observed_at_ms)` and the observation count;
    - publication counts for `pending`, `failed` and `expired`;
    - the oldest pending change's `created_at_ms`, via the partial pending
      index.

    Lag is computed from that DB value, not from the relay's last-pass stat.
  - **`/api/metrics`** (`api_metrics_render_observations()`, pure, in
    `api_handlers_metrics.c`). Every family is present with zero values while
    evidence is unused.
    - Admission: `lightnvr_observation_accepted_total` and
      `lightnvr_observation_dropped_total{reason=queue_full|budget_exhausted|disabled|untracked}`.
    - Evidence, all under `lightnvr_observation_evidence_`:
      - `unavailable_total`, `ready_total`, `bytes_written_total`;
      - `failures_total{reason=encode_failed|write_failed|db_error|disabled|interrupted}`;
      - `backlog_items`, `backlog_bytes`, `drop_ring_depth`;
      - `high_water_items`, `high_water_bytes`;
      - `queue_allocated`, `worker_running`;
      - per camera: `retained_bytes{camera_uuid}`, `budget_bytes{camera_uuid}`
        and `budget_exhausted{camera_uuid}`, plus the total
        `budget_exhausted_cameras`.
    - History, all under `lightnvr_observation_history_`: `observations`,
      `oldest_seq` (cursor = seq − 1), `head_seq`,
      `oldest_observed_timestamp_seconds` (no sample when empty) and
      `epoch_regenerations_total`.
    - Publications and relay: `lightnvr_observation_publications{state=pending|failed|expired}`,
      `lightnvr_observation_relay_lag_seconds`,
      `lightnvr_observation_relay_running` and
      `lightnvr_observation_relay_events_total{outcome=relayed|duplicate|no_route|expired|failed|outbox_full|enqueue_error|mark_error|read_error|cap_pause|backoff|sidecar_error}`.
    - The history and publication families are omitted when the DB read
      fails.
  - **`/api/system/info`** carries `observation_pipeline`:
    - Top level: `active`, `accepted`, `dropped{…}`,
      `budget_exhausted_cameras` and `cameras[]` (`camera_uuid`,
      `retained_bytes`, `max_bytes`, `max_age_days`, `budget_exhausted`).
    - `evidence`: `queue_allocated`, `worker_running`, backlog, high-water,
      `failures{…}` and more.
    - `history`: `oldest_seq`, `head_seq`, `oldest_observed_at` (ISO or
      null) and `epoch_regenerations`.
    - `publications`: `pending`, `failed`, `expired`, `relay_lag_seconds`
      and `relay_running`.

    `active: false` means the feature is unused. The System page shows the
    new `system/ObservationPipeline.tsx` card, with keys
    `system.observations.*` in all 18 locales.
  - **Retry-After**: `observation_snapshot_retry_after_seconds(age_ms,
    items_ahead)` returns 1 under 5 s, otherwise `min(30, 2 × ahead)`, always
    1..30. L3 has no per-asset position, so the handler passes
    `backlog_items` as the upper bound. The contract §8.2 is updated and no
    longer provisional.
    - The handler hunk in `api_handlers_observations.c` was swept into L14's
      commit `9e0e8ecd`, because L14 staged the whole file. HEAD did not
      build between `9e0e8ecd` and this commit.
  - **Sidecar consolidation**: the relay's own path, writer and reader
    (about 80 lines) are deleted.
    - `observation_relay_write_cursor_head()` now calls
      `observation_cursor_head_write()`.
    - `observation_relay_check_cursor_head()` now calls
      `observation_cursor_head_reconcile()` and updates the relay stats. The
      relay keeps its once-per-second dirty-write cadence.
    - The public names and `OBSERVATION_CURSOR_HEAD_FILE` are unchanged.
  - Contract §15 now lists the metric names.
  - **OpenAPI follow-up (not done)**: `SystemInformation` has
    `additionalProperties: true`, so validation passes and the UI uses a
    hand-written `ObservationPipelineDiagnostics` type in `types/system.ts`.
    L14 had uncommitted `openapi.json` edits at the time, so the schema was
    left alone. A follow-up should add an optional
    `observation_pipeline: ObservationPipelineDiagnostics` component that
    mirrors `observation_diagnostics_to_json()`, then regenerate.
  - **Gotchas**:
    - **X2**: per-camera rows come from L4's last pass, so they can be up to
      5 minutes old and cover only cameras that hold `ready` media. For live
      numbers during saturation, scrape `backlog_*`, `high_water_*` and
      `dropped_total`. High-water marks are per process run. Count failed and
      expired publications from the DB gauges; the relay counters reset on
      restart.
    - **The in-flight item counts toward `per_camera_max_items`**. With depth
      1, a second submit while the worker holds the first one is
      `DROPPED_QUEUE_FULL`.
    - **W8**: the setup probe stays `GET /api/observations?camera_uuid=X&limit=1`.
      An empty `items` means no detections yet, and a 403 or network error
      means a transport failure. `/api/metrics` and `/api/system/info` need
      `system.admin`, so YA-WAMF's scoped token cannot read them.
    - **L11**: the `failed`/`expired` publication counts scan
      `observation_publications`. That is cheap at the retention bound, but
      a scrape costs four small queries.
- **files edited/created**: `include/core/observation_diagnostics.h` (new), `src/core/observation_diagnostics.c` (new), `include/database/db_observations.h`, `src/database/db_observations.c`, `include/web/api_handlers_metrics.h`, `src/web/api_handlers_metrics.c`, `src/web/api_handlers_system.c`, `src/web/api_handlers_observations.c` (landed in `9e0e8ecd`), `include/core/observation_relay.h`, `src/core/observation_relay.c`, `tests/unit/test_observation_diagnostics.c` (new), `tests/unit/test_observation_relay.c`, `tests/unit/test_api_handlers_system.c`, `tests/unit/CMakeLists.txt`, `web/js/components/preact/system/ObservationPipeline.tsx` (new), `web/js/components/preact/SystemView.tsx`, `web/js/types/system.ts`, `web/tests/observationPipeline.spec.js` (new), `web/public/locales/*.json`, `docs/integrations/OBSERVATIONS_CONTRACT.md` (commit `9f6012a6`)

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
- **status**: Completed (YA-WAMF commit `ed8b400f` on `feature/lightnvr-provider`, not pushed)
- **log**:
  - RED: the three new pure test modules failed at collection with
    `ModuleNotFoundError: app.services.sources`. The startup test failed with
    `assert isinstance(provider, FrigateProvider)` while `main.py` was
    stashed. GREEN: 60 pure tests plus 1 startup test pass, and the 4 later
    identity cases bring it to 64.
  - Gates: W0 + `test_dead_modules` + the startup test give 53 passed. The
    pure lane (excluding W2's in-flight `test_source_identity.py`) gives 2004
    passed. Full suite: 7523 passed and 6 failed. The 6 failures were
    `inspect.getsource` tests on `detection_repository.py`/`database.py`,
    which W2 was editing during the run. All 24 tests in those files pass when
    rerun. Ruff check and format are clean on every W1 file.
  - Interface (W3/W4/W5 use these exact names), in `app/services/sources/base.py`:
    - `NvrSourceProvider` (runtime-checkable Protocol):
      - `identity() -> SourceIdentity(kind, instance)`
      - `capabilities() -> Capabilities`
      - `live_topics() -> list[str]`
      - `parse_live_message(topic, payload: bytes) -> list[SourceObservation]`
      - `async list_cameras() -> list[SourceCamera(camera_id, display_name)]`
      - `async history(cursor, filters: HistoryFilters) -> HistoryPage | RecoveryRequired`
      - `async fetch_snapshot(external_id, *, crop=True, quality=95, timeout=30.0) -> SnapshotResult`
      - optional, check capabilities first:
        - `fetch_clip(external_id, dest_path, *, timeout=20.0) -> (bool, reason)` (event_clip)
        - `fetch_range_clip(camera, after, before, *, timeout=20.0) -> (bytes|None, reason)` (range_clip)
        - `fetch_recording_frame(camera, frame_time, *, timeout=5.0) -> (bytes|None, reason)` (recorded_frame)
        - `write_classification(external_id, label, *, score=None) -> bool` (annotate)
    - `Capabilities` is a frozen dataclass. Every field defaults to False:
      `history`, `snapshots`, `annotate`, `track_lifecycle`, `recorded_frame`,
      `event_clip`, `range_clip`, plus `reasons`. Methods:
      - `supports(name)` raises `KeyError` on an unknown name.
      - `unsupported_reason(name)`.
      - `require(name)` raises `UnsupportedCapabilityError(capability, reason)`.
      - `Capabilities.from_contract_operations(doc["operations"])` treats an
        operation as usable only when `supported is True and authorized is True`.
        A missing or malformed entry gives the reason `not_declared`.
    - `CONTRACT_OPERATION_KEYS` maps each field to its contract operation key:
      `history→observations.history`, `snapshots→observations.snapshot`,
      `annotate→observations.annotate`. The other four map to themselves.
    - `SourceObservation` (frozen):
      - Always set: `source_kind`, `source_instance`, `external_id`,
        `event_key` (the `detections.frigate_event` value), `camera_name`,
        `source_camera_id`, `label`, `observed_at` (unix float or None).
      - `lifecycle` (`new|update|end` or None) and `is_false_positive`.
      - lightNVR-only, None for Frigate: `revision`, `message_id`,
        `change_cause`, `evidence_state`, `evidence_reason`, `asset_id`.
      - `raw`: the decoded source payload.
    - Other types:
      - `HistoryFilters(after, before, cameras: tuple, labels: tuple, has_snapshot=True, limit=100)`
      - `HistoryPage(observations, next_cursor, has_more)`
      - `RecoveryRequired(oldest_cursor, reason="cursor_expired")`
      - `SnapshotOutcome` (`READY`, `PENDING`, `UNAVAILABLE`, `EXPIRED`,
        `MISSING`, `TEMPORARILY_UNREACHABLE`)
      - `SnapshotResult(outcome, image, reason, cropped, retry_after_seconds)`
      - `SourceUnavailableError`
  - Registry, in `app/services/sources/registry.py`:
    - `build_source_provider(kind="frigate", *, app_settings=None)`; an
      unknown kind raises `ValueError`.
    - `install_source_provider(p)`.
    - `active_source_provider()` lazily falls back to Frigate, so callers that
      run before startup, or in tests without the lifespan, still get a
      provider.
    - `main.py` lifespan calls
      `install_source_provider(build_source_provider(app_settings=settings))`
      right after `_log_startup_diagnostics`, in test mode too.
    - W5 adds the `kind` selection from settings.
  - In `frigate_provider.py`:
    - `FrigateProvider(client, app_settings)` exposes `.client` (the
      `FrigateClient`) for Frigate-only work during W3.
    - `FRIGATE_CAPABILITIES` declares all seven capabilities.
    - `SOURCE_KIND_FRIGATE = "frigate"`, `frigate_source_instance(url)`.
  - Gotchas for W2–W5:
    - **Instance derivation is duplicated.** W2's uncommitted
      `app/utils/source_identity.frigate_source_instance` returns `host:port`
      or None, and its callers substitute `"unbound"`. W1's copy returns
      `"unbound"` directly. The two produce identical results on 16 probe
      inputs, including default ports made explicit, bare hosts, IPv6,
      credentials, a trailing dot and bad ports. W1 cannot import W2's
      uncommitted file, so once both land W3 should make the provider call
      W2's function and delete the copy.
    - Frigate `parse_live_message`:
      - reads only `{main_topic}/events`;
      - treats a missing `type` as `new`, matching `EventData`;
      - returns `[]` for an unrecognised type unless the message is a false
        positive, which gets `lifecycle=None`;
      - leaves label, camera and ingest gating to the pipeline;
      - never adds `__received_at_ts`. W4 keeps adding it before
        `_process_event_payload(obs.raw)`.
    - Frigate `history` is time-windowed, so the cursor must be None, and it
      takes at most one camera and one label (`ValueError` otherwise).
      Backfill keeps its own `before`-cursor paging. `has_more` is
      `len(events) >= limit`. `FrigateEventsFetchError` propagates.
      History items have `lifecycle=None`.
    - Frigate `fetch_snapshot` keeps the original `FrigateClient` reason
      string. Outcome mapping:
      - `snapshot_not_found` → `MISSING`;
      - timeout, request error or HTTP 5xx → `TEMPORARILY_UNREACHABLE`;
      - anything else → `UNAVAILABLE`.

      `cropped` equals the `crop` argument. lightNVR returns the full image
      with `cropped=False`, and the caller crops by box.
    - Frigate-only calls have no protocol method yet: thumbnail, clean and
      alignment snapshot, `get_event`, config/version and motion recording.
      W3 either adds capability-gated methods or keeps them behind
      `FrigateProvider.client`.
    - Clip and recording user settings (`clips_enabled`,
      `recording_clip_enabled`) are not capabilities. Check both.
    - YA-WAMF `test_dead_modules` requires every new module to be reachable.
      The `main.py` import covers `sources/*`, and W5's `lightnvr_provider.py`
      must be imported from the registry.
    - The full suite has `inspect.getsource` tests that fail while another
      agent is editing repository files in the same worktree. Rerun them
      before treating a failure as real.
    - No changelog fragment was added: W1 changes no user-visible behaviour,
      setting, endpoint or label (`changelog.d/README.md`).
- **files edited/created**:
  - YA-WAMF `backend/app/services/sources/__init__.py` (new)
  - YA-WAMF `backend/app/services/sources/base.py` (new)
  - YA-WAMF `backend/app/services/sources/frigate_provider.py` (new)
  - YA-WAMF `backend/app/services/sources/registry.py` (new)
  - YA-WAMF `backend/app/main.py` (installs the provider at startup)
  - YA-WAMF `backend/tests/pure/test_source_capabilities.py` (new)
  - YA-WAMF `backend/tests/pure/test_frigate_source_provider.py` (new)
  - YA-WAMF `backend/tests/pure/test_source_registry.py` (new)
  - YA-WAMF `backend/tests/test_source_provider_startup.py` (new)

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
- **status**: Completed (YA-WAMF commit `d3079b63` on `feature/lightnvr-provider`, not pushed)
- **log**:
  - The revision is `d4e1a7c93b52` (it revises `c1d734b9e201`) and is the
    single head. P1 needs no other YA-WAMF revision.
  - **`detections` columns**, all nullable:
    - `source_kind` VARCHAR(32), `source_instance` VARCHAR(255),
      `external_observation_id` VARCHAR(255) and `source_camera_id`
      VARCHAR(255).
    - Partial unique index `uq_detections_source_identity(source_kind,
      source_instance, external_observation_id) WHERE source_kind IS NOT NULL`.
    - For W7: `notification_intent`, constrained by
      `ck_detections_notification_intent` to
      `dispatch|suppressed_recovery|suppressed_policy`, and
      `notification_intent_at`.
    - For W7: `notification_outcome`, constrained by
      `ck_detections_notification_outcome` to
      `sent|failed|unknown|skipped`, and `notification_outcome_at`.
    - NULL intent means a legacy row or no intent recorded. `notified_at`
      still marks a successful send.
  - **`source_cursors` table**:
    - `source_kind` and `source_instance` form the primary key
      (`pk_source_cursors`).
    - Other columns: `cursor` TEXT, `epoch`, `filter_fingerprint`,
      `last_error`, and `updated_at` (default `CURRENT_TIMESTAMP`).
    - `filter_fingerprint` was added for the contract §16.5 camera and label
      filter fingerprint.
    - `recovery_state` is NOT NULL, defaults to `current`, and is constrained
      by `ck_source_cursors_recovery_state` to `current|recovery_required`.
  - **Choice of instance**: placeholder plus a startup bind, chosen as the
    safer option.
    - No existing migration reads settings, and the resolved `frigate_url`
      depends on environment overrides, the config file and defaults.
    - The migration therefore backfills `('frigate', 'unbound',
      frigate_event)`.
    - `init_db()` then calls `_bind_unbound_source_identity` after
      `_verify_schema`. That runs `bind_unbound_frigate_detections`
      (`app/repositories/source_identity_repository.py`), which:
      - gives NULL-source rows (old-binary inserts) the kind `frigate`,
        `external_observation_id = frigate_event` and the configured instance;
      - replaces `unbound` with the configured instance;
      - never touches a row that already has a real instance, so a later
        `frigate_url` change does not reassign history.
    - The instance comes from `app/utils/source_identity.frigate_source_instance`:
      - the lowercased host, with any trailing dot dropped, plus the port;
      - the scheme's default port (80 or 443) is made explicit;
      - credentials, path and query are dropped;
      - IPv6 hosts are bracketed;
      - a URL with no scheme is treated as http.
    - Examples: `http://Frigate.LAN:5000/` → `frigate.lan:5000`, and the
      default `http://frigate:5000` → `frigate:5000`.
    - An unusable URL returns None. The row keeps `unbound`, which can never
      equal a derived value because a derived value always has a `:port`.
      The bind is retried on the next start.
    - The bind never blocks startup.
  - **No table recreation**: the migration uses raw `ALTER TABLE ADD/DROP
    COLUMN`, not batch recreation. Batch recreation would drop the four
    `detections` triggers (initial-classification capture and media cleanup)
    and rebuild the parent of eight foreign keys. As a result, downgrade
    needs SQLite 3.35 or newer; it raises a clear error otherwise.
  - **RED → GREEN**:
    - New tests first failed at collection (`ModuleNotFoundError`).
    - With the implementation present and the migration file removed, 8 of 8
      migration tests failed.
    - With the migration restored, all pass.
    - The first full-suite run found two more problems:
      - 29 failures in hand-built `CREATE TABLE detections` fixtures
        (`test_detection_repository.py`, `test_canonical_identity_repair.py`),
        fixed by adding the columns to them;
      - 1 real regression: SQLite reports the source-tuple conflict *before*
        the `frigate_event` conflict, which broke `create()`'s idempotent
        duplicate skip.
    - The regression got its own RED test, then a fix:
      `_DUPLICATE_DETECTION_KEYS` treats either key as a duplicate.
  - **Final results**:
    - Full suite: 7556 passed, 60 skipped.
    - W0 gate, all migration tests, `scripts/ci_migration_path_check.py`,
      `ruff check` and `ruff format --check` from the repo root, and
      `docs_consistency_check.py` are all green.
  - **Gotchas for W4**:
    - Any `Detection` with `source_kind=None` is written as a Frigate row for
      the configured instance (`_source_identity_params`). lightNVR rows must
      set `source_kind='lightnvr'`, `source_instance` (the installation UUID),
      `external_observation_id` and `source_camera_id`.
    - A NULL `source_instance` would escape the unique index.
    - `INSERT OR IGNORE` paths (`insert_if_not_exists`,
      `upsert_if_higher_score`) silently ignore a source-tuple conflict.
    - `DETECTION_SELECT_COLUMNS` and `_row_to_detection` deliberately do not
      read the source columns, because callers append columns positionally.
      Only `DetectionRepository.get_by_source_identity()` fills them; extend
      deliberately.
  - **Gotchas for W6**:
    - There is one cursor row per `(kind, instance)`. A changed filter must
      reset the row to absent (§7.3).
    - A `410` sets `recovery_state='recovery_required'`.
    - `REQUIRED_COLUMNS` now lists the `source_cursors` columns.
  - **Gotchas for W7**:
    - Use the intent and outcome vocabularies above. Changing them needs a
      new revision, and adding a CHECK later would force a table recreation,
      so avoid that.
    - The migration does not backfill intent on legacy rows. NULL means
      legacy.
- **files edited/created** (YA-WAMF):
  - `backend/migrations/versions/d4e1a7c93b52_add_source_identity.py` (new)
  - `backend/app/utils/source_identity.py` (new)
  - `backend/app/repositories/source_identity_repository.py` (new)
  - `backend/app/db_schema.py`, `backend/app/database.py`,
    `backend/app/repositories/detection_repository.py`
  - `backend/tests/test_source_identity_migration.py` (new),
    `backend/tests/pure/test_source_identity.py` (new),
    `backend/tests/test_detection_repository.py`,
    `backend/tests/test_canonical_identity_repair.py`
  - `MIGRATION.md`, `changelog.d/lightnvr-source-identity.md` (new)

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
- **status**: Completed (YA-WAMF commit `ca1ee246` on `feature/lightnvr-provider`, not pushed)
- **log**:
  - RED: `tests/pure/test_no_direct_frigate_calls.py` listed 36 violations
    (every importer plus 7 `/api/events` builders in `proxy.py`,
    `media_probe.py` and `notification_orchestrator.py`); its self-test of the
    scanner passed. `tests/test_snapshot_only_source.py` failed 3/3: fake
    Frigate received `snapshot.jpg?crop=0`, `POST sub_label` and so on, and
    `high_quality_snapshot_service.enabled()` was True. GREEN: both pass, as do
    the new `test_source_media_routes.py` (12) and
    `pure/test_source_event_keys.py` (14). These two were written alongside the
    implementation, so they have no RED run.
  - Gates: W0 is 46/46 green after every service. Full suite: 7727 passed,
    60 skipped, 0 failed (W2 and W5 had committed by then, so the run is clean).
    `ruff check .` and `ruff format --check .` are clean repo-wide. OpenAPI
    and the frontend types were regenerated and `--check` passes. The
    docs-consistency and changelog checks pass. `npm run check` was not run,
    because `apps/ui/node_modules` is not installed; only generated types
    changed there.
  - New in `app/services/sources/`:
    - `access.py` is the only way shared code reaches the NVR:
      - `resolve_event_source(event_key)` returns `(provider, external_id)`,
        or None when the key's source kind or instance is not the active one;
      - `fetch_event_snapshot`, `fetch_event_snapshot_bytes` (the old
        `(bytes, reason)` shape), `fetch_event_thumbnail`,
        `external_snapshot_url`, `fetch_event_record` (needs `track_lifecycle`),
        `fetch_event_clip_bytes` and `download_event_clip` (`event_clip`),
        `fetch_range_clip` (`range_clip`), `fetch_recording_frame`
        (`recorded_frame`), `write_event_classification` (`annotate`);
      - `active_frigate_client()` is None unless Frigate is the active
        source; `configured_frigate_client()` is for owner tools about the
        configured Frigate itself.
    - `capability_gate.py`: `source_allows(workflow, *caps)` is true when the
      source declares any of the caps. Otherwise it records a `CapabilitySkip`
      (`status="skipped: unsupported by source"`, the source's reason such as
      `not_implemented`, a count and the time). It also bumps the Prometheus
      counter `source_capability_skips_total{workflow,capability,source_kind}`
      and logs at info once, then at debug. Related helpers: `record_skip`,
      `record_source_not_active`, `recorded_capability_skips()` (for W8
      health) and `clear_recorded_capability_skips()`. Reason constants are
      `unsupported_by_source` and `source_not_active`.
    - `event_keys.py`:
      - `event_key_for(kind, instance, id)` gives the Frigate id itself, or
        `lnvr:<inst>:<obs>` for lightNVR; `parse_event_key` reads it back.
      - `media_cache_key`: a Frigate key keeps its legacy name, byte for
        byte. A lightNVR key becomes `lnvr.<inst>.<obs>`. It is validated,
        never stripped, so `lnvr:ab:c` and `lnvr:a:bc` cannot collide. A
        Frigate id that sanitises to `lnvr.` is refused.
      - `event_key_from_media_key` inverts it, so `cache_file_event_id`
        resolves qualified owners.
      - `media_cache.sanitize_event_id`, which the archive also uses, now
        delegates to `media_cache_key`.
  - Protocol additions (W5 already implements them):
    - required: `fetch_thumbnail` and `external_snapshot_url`;
    - optional: `fetch_event` (Frigate-shaped event document, needs
      `track_lifecycle`) and `fetch_clip_bytes` (`event_clip`).
    - `FrigateProvider()` built without a client resolves the
      `frigate_client` singleton at call time. Tests can therefore patch
      `app.services.frigate_client.frigate_client`, and the old per-module
      `X.frigate_client` patches were rewritten to that.
    - It has `aclose()`, and `registry.close_active_source_provider()`
      replaces `frigate_client.close` in `main.py`.
    - `frigate_event_media_url(id, asset)` is the only builder of Frigate
      `/api/events` URLs outside the client.
    - `FrigateEventsFetchError` now subclasses `SourceUnavailableError`.
    - The duplicated instance derivation was removed: `frigate_source_instance`
      now calls W2's `app/utils/source_identity.frigate_source_instance` and
      falls back to `unbound`.
  - Capability gating, by workflow name:
    - `high_quality_snapshot`: `enabled()` needs `event_clip` or `range_clip`.
    - `full_visit` (`range_clip`): both the event-processor trigger and
      `FullVisitClipService._auto_generation_enabled`.
    - `video_classification` (`event_clip` or `range_clip`): in
      `trigger_classification` and `queue_classification`. The latter returns
      the new `"unsupported"`, which the manual video reclassify maps to HTTP
      409 (`errors.events.video_analysis_unsupported_by_source`) and batch
      analysis stops on.
    - `recording_snapshot` (`recorded_frame`).
    - `recording_frame_fallback` (`range_clip`).
    - `write_classification` (`annotate`).
    - `garden_capture` (`track_lifecycle`), checked at the top of
      `GardenCaptureService.process`.
    - `visit_film` (`range_clip` and `track_lifecycle`).
    - `garden_motion` (`range_clip` plus a Frigate client).
    - `backfill` (`history`; time-window replay is Frigate-only and is
      recorded as `time_window_history_is_frigate_only`).
    - `media_integrity_scan` (Frigate-only, returns `disabled` with a message).
    - Missing-media purges and batch analysis return `failed` or
      `unsupported` with plain messages when Frigate is not the source.
    - The archive and Garden media treat `unsupported_by_source` and
      `source_not_active` as unavailable or missing, not retryable.
  - Neutral routes in `routers/proxy.py`, with the same auth, share-link and
    guest rate-limit rules:
    - `GET /api/media/{source_kind}/{external_id}/snapshot` and `/thumbnail`.
    - `frigate` delegates to the bodies of the existing `/frigate/{id}/...`
      routes, now `_serve_frigate_snapshot` and `_serve_frigate_thumbnail`.
    - Another kind must be the active source. It is served from the cache
      (`lnvr.` key), then the archive, then `provider.fetch_snapshot(crop=False)`,
      cached as `source_snapshot`.
    - Outcomes: PENDING gives 202 with `Retry-After` (1–30) and
      `retry_after_seconds`; EXPIRED gives 410; MISSING and UNAVAILABLE give
      404; TEMPORARILY_UNREACHABLE gives 502.
    - The thumbnail is derived from the snapshot.
    - Documented in `docs/api.md`. New i18n keys are added in all nine
      locales.
  - Remaining Frigate-only exceptions, all through `active_frigate_client()`
    or `configured_frigate_client()`, never imports:
    - The `/frigate/...` streaming proxies (clip, recording clip, HLS,
      previews, `HEAD` checks) and `full_visit_clip_service`'s streamed
      recording fetch. They stream Frigate URLs with their own `httpx`
      clients and Frigate response semantics. W0's swapped-`httpx` harness is
      untouched.
    - The clean and alignment snapshot (HQ final frame,
      `recording_snapshot_input`, Garden crops): only Frigate's
      `snapshot-clean` fixes native coordinates.
    - Garden: event metadata, snapshots, clips and motion scanning follow
      Frigate's lifecycle and keep the original client calls. Garden capture
      is gated by `track_lifecycle`.
    - Retention and missing-media checks, timezone repair, the video-sample
      probe, and the owner tools for Frigate itself (connection test, config,
      camera status, latest frame, recording-clip capability, version).
  - Gotchas for W4/W5/W8:
    - The default Frigate provider resolves the client lazily. Tests that
      install a provider use `monkeypatch.setattr(registry, "_active_provider", p)`.
      The suite `conftest.py` now restores `_active_provider` and clears the
      recorded skips around every test.
    - The snapshot-only test's fake is Frigate-kind, because events are still
      Frigate-shaped until W4. W9 should repeat it with a lightNVR-kind
      provider once W4 routes `lnvr:` keys through the pipeline.
    - `resolve_event_source` refuses a key whose kind or instance differs from
      the active source, so W4 must store `lnvr:<inst>:<obs>` exactly as
      `event_key_for` builds it. W5 has its own identical `lightnvr_event_key`
      in `lightnvr_provider.py`; collapse it onto `event_keys`.
    - `notification_orchestrator` sends `snapshot_url=""` when the source has
      no external link (lightNVR). W7 owns links through the provider.
    - `enabled()` and the other gates record a skip on every check, including
      status polls, so the counter measures refusals, not distinct jobs. W8
      should read `recorded_capability_skips()` for its "unsupported" copy
      rather than the counter.
    - `tests/pure/test_auto_video_classifier_pressure.py` fails at `HEAD` when
      run alone (a pre-existing `derive_archive_state` stub gap). Its module
      reimport used to leak into later tests; it is now restored at teardown.
- **files edited/created**:
  - YA-WAMF `backend/app/services/sources/access.py`, `capability_gate.py`, `event_keys.py` (new)
  - YA-WAMF `backend/app/services/sources/base.py`, `frigate_provider.py`, `registry.py`, `backend/app/services/frigate_client.py`, `backend/app/main.py`
  - YA-WAMF routers: `ai.py`, `events.py`, `proxy.py` (neutral media routes), `settings.py`
  - YA-WAMF services: `archive_service`, `auto_video_classifier_service`, `backfill_service`, `classification_input_provenance`, `event_processor`, `full_visit_clip_service`, `garden_automatic_review`, `garden_capture_service`, `garden_crop_service`, `garden_identification`, `garden_media_cache`, `garden_media_policy`, `garden_motion`, `high_quality_snapshot_service`, `inaturalist_service`, `media_cache`, `media_integrity_scan`, `media_probe`, `notification_orchestrator`, `recording_snapshot_input`, `timezone_repair_service`, `visit_film_service`
  - YA-WAMF `backend/locales/*.json`, `backend/openapi.json`, `apps/ui/src/lib/api/generated/openapi.ts`, `docs/api.md`, `changelog.d/source-capability-gating.md`
  - YA-WAMF tests (new): `tests/pure/test_no_direct_frigate_calls.py`, `tests/pure/test_source_event_keys.py`, `tests/test_snapshot_only_source.py`, `tests/test_source_media_routes.py`
  - YA-WAMF tests (patch targets moved to the shared client): `tests/conftest.py`, `tests/pure/conftest.py` and 40 existing test modules

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
- **status**: Completed (YA-WAMF commit `710de873` on `feature/lightnvr-provider`, not pushed)
- **log**:
  - **RED→GREEN**: all three new test modules failed at collection
    (`ModuleNotFoundError: app.services.sources.evidence_crop`, then
    `observation_admission`, and `ImportError: mqtt_topic_matches`). After the
    change they give 141 passed together with `test_lightnvr_parse.py`. One test
    bug was fixed on the way: a fake watchdog recorded its topic only when the
    coroutine ran, and the task was cancelled before that.
  - **Gates**: W0, `pure/test_mqtt_service.py`,
    `test_live_notification_integration.py`, `test_dead_modules.py` and
    `pure/test_no_direct_frigate_calls.py` give 126 passed. Pure lane: 2165
    passed, 4 skipped. Rest of the suite (`--ignore=tests/pure`): 5651 passed,
    56 skipped, 0 failed, run with W8's in-flight hunks in the tree.
    `dead_modules_check.py`, `docs_consistency_check.py`, the changelog check,
    and `ruff check`/`ruff format` on every W4 file are clean. No Alembic
    revision was added; W2's `d4e1a7c93b52` is still the single head.
  - **Entry points (W6/W7 call these)**:
    - `EventProcessor.process_live_message(topic, payload)`: the MQTT service
      calls this for a non-Frigate source. It parses with the active provider,
      then admits each observation with `origin="live"`.
    - `EventProcessor.process_source_observation(obs, *, origin="live"|"recovery") -> AdmissionDecision`.
      This is the one admission path for live and recovery. W6 passes history
      items straight in: `change_cause=None` means the item already is the
      authoritative state, so it is not fetched again.
    - `AdmissionDecision(action, reason, detail)`, in
      `app/services/sources/observation_admission.py`:
      - `action` is `classify|wait|skip|retry`;
      - `.settled` is False only for `retry`. **W6 advances the cursor only
        past settled decisions.**
      - Reasons: `classified`, `duplicate_message`, `classification_only`,
        `stale_revision`, `already_admitted`, `evidence_pending`,
        `evidence_unavailable` / `evidence_expired` (`detail` = the lightNVR
        reason), `observation_missing`, `source_unreachable`, `source_refused`
        (a permanent `LightNvrApiError`), `source_not_active`,
        `label_not_ingested`, `classification_unavailable`, `filtered`
        (`detail` = the filter reason), `not_saved`.
    - Pure helpers: `ObservationLedger.screen/settle` and
      `decide_evidence(current, *, already_admitted)`. In
      `sources/evidence_crop.py`: `parse_normalized_box`,
      `parse_evidence_transform`, `inference_point_to_stored`, `pixel_box`,
      `classification_crop_region` and `evidence_regions`.
    - Orchestration lives in `app/services/source_ingestion.py`
      (`SourceIngestion`, `SourceObservationEvent`).
      `DetectionService.save_detection` gained the optional keyword arguments
      `source_kind`, `source_instance`, `external_observation_id` and
      `source_camera_id`. `_save_detection` forwards
      `event.detection_save_options` (which also carries
      `report_to_birdweather=False` for recovery). A Frigate save passes
      nothing new, so its call is identical.
  - **Dedupe storage**: there is no new table and no revision.
    `processing_job_state.event_id` has a foreign key to
    `detections.frigate_event`, so it cannot hold anything before a detection
    exists. The **`detections` row is the durable admission record**:
    - The row is unique on `frigate_event` and on W2's source tuple, and it is
      checked under a per-observation lock before any fetch.
    - Once a row exists, any later message, revision or recovery item is
      `already_admitted`, even after a restart.
    - Pending, unavailable and expired evidence writes nothing durable, as
      contract §16.2 requires.
    - `ObservationLedger` is a bounded in-process LRU of 4096 entries. It holds
      settled `(source, message_id)` pairs and the highest settled revision per
      observation, so a replay costs no request. Losing it only costs a refetch.
  - **Pipeline behaviour**:
    - A live trigger is followed by `fetch_observation`, which supplies the box,
      the transform, `stream_name_at_capture` and the current state.
    - Evidence goes through `access.fetch_event_snapshot(crop=False, workflow="source_ingest")`,
      so it is capability-gated. Images are cached per
      `(instance, asset_id)` (16 entries), which means siblings fetch once.
    - A `202` after the observation already said `ready` is a **retry**, not a
      wait, because no later ready change will arrive.
    - The crop is a square of the box's longer side × 1.1, centred and kept
      inside the image. Frigate's 300 px floor is dropped, because on a small
      frame it would swallow the sibling.
    - C3 crops: bird 0 is `(14,14,50,50)` and bird 1 is `(93,17,146,70)` on the
      160×80 JPEG. The letterbox box maps to `(512,90,768,234)` and its crop is
      `(499,21,781,303)`. The transform is audited but never re-applied. A null
      box classifies the whole image (`lightnvr_evidence`, `is_cropped=False`).
    - Classifier input: `input_source="lightnvr_evidence_crop"`,
      `is_cropped=True`, `source_image_sha256` = the evidence image. The stored
      `classification_input` carries the crop box, frame size and frame time,
      and a `frame_sha256` shared by siblings, which records the shared asset.
    - The cached snapshot (media key `lnvr.<inst>.<obs>`) is the crop. The
      notification attachment is the same crop.
    - `frigate_score` holds lightNVR's detector confidence. The event `type` is
      `new`. `camera_name` is `stream_name_at_capture`, and
      `source_camera_id` is the camera UUID.
    - Skips, waits and retries appear in `recent_outcomes` as
      `"skipped: <reason>"`, `"waiting: …"` or `"retry: …"`, with `detail`.
  - **Not run for lightNVR**: the Frigate lifecycle (`new/update/end`), end
    refinement, `_handle_terminal_event_enrichment`, recording waits, the
    staleness drop, Garden capture and domestic-bird routing. The last two are
    Frigate- and lifecycle-only Garden features. No `end` is synthesized.
  - **MQTT**:
    - With a non-Frigate provider the service subscribes only to
      `live_topics()` (plus BirdNET).
    - It does not subscribe to `frigate/events` or `frigate/available`, starts
      no watchdog and runs no stall reconnects.
    - Messages are matched with `mqtt_topic_matches` (`+` and `#`) and
      dispatched as `_schedule_source_message`, under the shared semaphore and
      the 90 s timeout.
    - `_refresh_source_capabilities()` runs at `start()` (also when no broker is
      configured) and before every reconnect. It is bounded at 30 s, and a
      failure is logged without blocking.
    - The Frigate path is byte-identical. Frigate's `parse_live_message` stays
      off the hot path, because per-event serialization and pending-payload
      merging need the raw payload.
  - **Multi-bird (verified by test)**: two same-species siblings give
    2 detections, 1 visit with `capture_count` 2, and a leaderboard window with
    `window_count` 2 and `window_visit_count` 1. Visits and leaderboard visits
    do not double count.
    - **Intended difference** (contract §16.3): a Frigate two-bird event is
      1 detection, while lightNVR yields 2.
    - **Deviation from decision 8**: no `bird_observations` row is written.
      Those rows need bird-scan candidate media (`candidate_id`,
      `clip_variant`, `frame_index`), and synthetic rows would feed the HQ and
      bird-scan machinery. A lightNVR detection therefore counts like a
      Frigate detection that has never been bird-scanned: `counted_birds` and
      `counted_captures` in the daily summary exclude it. W9 should check the
      multi-bird display, and decide whether to add index-0 rows with a real
      candidate.
  - **Other changes**:
    - `lightnvr_provider.lightnvr_event_key` is now `event_keys.lightnvr_event_key`
      (re-exported).
    - **Bug fixed**: `LightNvrSettings.source_instance` allowed `.` and `_`,
      which `event_keys` rejects (and `.` is the media-key separator). It is now
      `^[A-Za-z0-9][A-Za-z0-9-]{0,63}$`. The docs and a test case were updated.
  - **Gotchas**:
    - **W6**: a live `retry` is not retried by itself. Recovery must replay it,
      so only advance the cursor past settled decisions. A detection the owner
      hard-deletes would be admitted again if the cursor is reset. Hidden rows
      still count as admitted. History items have no `message_id` collision
      with live items for the same revision (W5 recomputes the ID), so the
      ledger dedupes the overlap.
    - **W7**:
      - Live admission enqueues `_enqueue_notification_flow` with
        `type="new"`, `changed=True` and `was_inserted=True`. `standard` and
        default modes therefore notify, but `final` does not yet, because it
        still waits for an `end` (W7 owns that).
      - Recovery (`origin="recovery"`) enqueues **no** notification; W7
        records `suppressed_recovery`.
      - Siblings notify separately, subject to cooldown.
    - **W8**: the UI or validation for `lightnvr_source_instance` must match
      the new regex.
    - **W9**: repeat the snapshot-only test with this pipeline.
      `write_frigate_sublabel` makes `write_event_classification` record an
      `annotate` capability skip on every save.
    - **W10**: replace the `annotate` skip through
      `access.write_event_classification`, which is already called after save.
- **files edited/created** (YA-WAMF):
  - `backend/app/services/source_ingestion.py` (new),
    `backend/app/services/sources/observation_admission.py` (new),
    `backend/app/services/sources/evidence_crop.py` (new)
  - `backend/app/services/mqtt_service.py`, `event_processor.py`,
    `detection_service.py`, `sources/lightnvr_provider.py`,
    `backend/app/config_models.py`
  - `backend/tests/pure/test_observation_admission.py` (new),
    `backend/tests/pure/test_source_live_dispatch.py` (new),
    `backend/tests/test_lightnvr_ingestion.py` (new),
    `backend/tests/pure/test_lightnvr_parse.py`
  - `docs/setup/environment-variables.md`, `changelog.d/lightnvr-ingestion.md` (new)

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
- **status**: Completed (YA-WAMF commit `88e09e4d` on `feature/lightnvr-provider`, not pushed)
- **log**:
  - **RED→GREEN**: both new test modules failed at collection with
    `ImportError: LightNvrSettings` (and the missing provider modules). After
    the change, 137 pass, then 138 with a protocol-conformance check. Two real
    bugs surfaced on the way:
    - httpx copies an `httpx.Cookies` object into a fresh jar and drops its
      policy, so a `Set-Cookie` came back on the next request. The client now
      passes a `CookieJar` with a reject-all policy directly.
    - Float milliseconds truncated `.481` to `.480` in `since`. The client now
      rounds to integer milliseconds.
  - **Gates**, run on the exact commit content in an isolated worktree at
    HEAD, because W3's in-flight edits break 46 pure tests in the shared tree:
    - W0, `test_dead_modules`, the startup test and the new tests: 191 passed.
    - Pure lane: 2101 passed.
    - Rest of the suite: 5593 passed, 56 skipped, 0 failed.
    - `export_openapi.py --check`, `export_openapi_types.py --check`,
      `dead_modules_check.py`, `docs_consistency_check.py`, the changelog
      check, and repo-wide `ruff check` and `ruff format --check`: clean.
  - **Public API**, `app/services/sources/lightnvr_client.py`:
    - `LightNvrClient(api_url, token, *, timeout=10.0, transport=None)`.
      Methods: `get(path, *, params, accept, max_bytes, timeout)`,
      `get_capabilities()`, `list_streams(page=, page_size=)`,
      `list_observations(params)`, `get_observation(id)`,
      `get_snapshot(id, timeout=)` and `aclose()`. It returns
      `LightNvrResponse(status_code, headers, body)`, with `.json_object()`
      and `.content_type`.
    - Errors: `LightNvrUnreachableError(reason, msg)` subclasses
      `SourceUnavailableError`, with reason `not_configured|timeout|request_error`.
      `LightNvrResponseTooLargeError` is raised past the limits (JSON 8 MiB,
      media 20 MiB, both instance-adjustable).
    - The client sends `Authorization: Bearer`, stores no cookies, never
      follows redirects, and redacts the token from any logged exception
      text.
  - **Public API**, `app/services/sources/lightnvr_provider.py`:
    - `LightNvrProvider(client, config, *, sleep=asyncio.sleep, clock=time.monotonic)`:
      - `identity()` returns `("lightnvr", source_instance override or
        capabilities.source.installation_uuid)`. Without an override it raises
        `SourceUnavailableError` until capabilities load.
      - `capabilities()` reads `capabilities_not_loaded` on every field until
        the document loads.
      - `apply_capabilities(doc)` is sync and pure-testable. It raises
        `SourceUnavailableError` and keeps the last good state when the doc
        is not `kind: lightnvr`, when `contract_versions.observations ≠ 1`,
        when `source.id` ≠ `urn:lightnvr:<installation_uuid>`, or when there
        is no `operations` block.
      - `async refresh_capabilities()`.
      - `capability_info` returns `LightNvrCapabilityInfo`: `source_urn`,
        `installation_uuid`, `product_version`, `writer_namespace`,
        `mqtt_enabled`, `delivery_topic`, the history page and filter limits,
        and `camera_uuids`.
      - `writer_namespace`.
      - `live_topics()` and `parse_live_message(topic, payload)`.
      - `observation_from_api(item)`.
      - `async list_cameras()`, `async history(cursor, filters)`,
        `async fetch_observation(id) -> SourceObservation | None` and
        `async fetch_snapshot(...)`.
      - `fetch_thumbnail` returns None and `external_snapshot_url` returns
        None.
      - `fetch_event`, `fetch_clip`, `fetch_clip_bytes`, `fetch_range_clip`,
        `fetch_recording_frame` and `write_classification` raise
        `UnsupportedCapabilityError`.
      - `aclose()`.
    - `LightNvrApiError(status_code, code)` means a permanent refusal. It is
      **not** a `SourceUnavailableError`.
    - Helpers: `lightnvr_event_key(instance, obs_id)` returns
      `lnvr:<instance>:<obs_id>`, `observation_message_id(source, obs_id, rev)`
      (the §5.4 UUIDv5), `is_canonical_uuid`, `build_lightnvr_provider(config)`.
      Constants: `SOURCE_KIND_LIGHTNVR`, `OBSERVATION_CHANGED_EVENT_TYPE`,
      `MESSAGE_ID_NAMESPACE`.
    - `registry.configured_source_kind(app_settings)`.
      `build_source_provider(kind=None, *, app_settings=None)` now picks
      lightNVR when `settings.lightnvr.enabled`, so `main.py` is unchanged.
  - **Settings**: `Settings.lightnvr: LightNvrSettings` has these fields:
    - `enabled` (False);
    - `api_url` (http/https only, no credentials, query or fragment);
    - `token` (`repr=False`);
    - `mqtt_topic_prefix` (`lightnvr`; plain levels, no wildcards);
    - `camera_uuids` (accepts a list or a comma string; lowercased and
      deduped, invalid entries dropped with a warning);
    - `source_instance` (`^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$`, otherwise a
      validation error);
    - `request_timeout_seconds` (10, from 1 to 60).

    Environment variables `LIGHTNVR__ENABLED|API_URL|TOKEN|MQTT_TOPIC_PREFIX|CAMERA_UUIDS|SOURCE_INSTANCE|REQUEST_TIMEOUT_SECONDS`
    are documented in `docs/setup/environment-variables.md`. An environment
    variable overrides the file per key. The settings API fields are
    `lightnvr_enabled|api_url|token|mqtt_topic_prefix|camera_uuids|source_instance`.
    The GET response returns `lightnvr_token` as `***REDACTED***`. A POST
    with the placeholder keeps the stored token (`should_update_secret`). The
    section is revalidated as a whole, and a bad value returns `422` without
    being stored. The token is also covered by `configured_secrets()`, through
    the `token` key.
  - **Mapping decisions**:
    - Live messages:
      - Accepted only on `{prefix}/v1/events/io.lightnvr.detection.observation_changed.v1/<camera_uuid>`
        and only after capabilities load.
      - The envelope must have `specversion` 1.0, the exact `type`,
        `source` equal to the capabilities URN, and `subject` equal to
        `camera/<topic uuid>` and to `data.camera_uuid`.
      - The `id` must equal the recomputed UUIDv5.
      - Payloads are capped at 64 KiB.
      - Unknown types, causes or states are dropped. An unknown
        `unavailable`/`expired` reason is kept as `[a-z0-9_]`.
      - Cameras outside `camera_uuids` are dropped.
    - Live `camera_name` is the camera UUID, because the event carries no
      stream name. History and `fetch_observation` items use
      `stream_name_at_capture`, with control characters stripped and capped
      at 63.
    - History items have `change_cause=None` and a recomputed `message_id`
      for `(source, id, current revision)`, so live/recovery dedup keys match.
      `lifecycle` is always None.
    - History outcomes:
      - `410 cursor_expired` → `RecoveryRequired(oldest_cursor)` (None if the
        cursor is unsafe);
      - `400 invalid_cursor` → `RecoveryRequired(None, "invalid_cursor")`;
      - 5xx, network error or an unreadable page → `SourceUnavailableError`;
      - other 4xx (for example `403` on a camera filter) → `LightNvrApiError`.
    - History filters:
      - comma-joined cameras and labels; a label containing a comma is a
        `ValueError`, sent before any request;
      - `since`/`until` in RFC 3339 with milliseconds;
      - `limit` clamped to `history_page_max`;
      - empty `filters.cameras` falls back to the configured `camera_uuids`;
      - more than 10 cameras → no server filter, filtered client-side;
      - `has_snapshot` is ignored, because lightNVR has no such filter.
    - Snapshot outcomes:
      - `200` image/jpeg → `READY` (`cropped=False`);
      - `202` → poll on Retry-After (header, then body
        `retry_after_seconds`, then a default of 2), clamped to 1–30. The
        budget is the `timeout` argument capped at 120 s, with at most 12
        attempts, then `PENDING` with `retry_after_seconds`;
      - `404 evidence_unavailable` → `UNAVAILABLE(reason)`;
      - any other `404` → `MISSING("observation_not_found")`;
      - `410` → `EXPIRED(reason)`;
      - `403` → `UNAVAILABLE("forbidden")`;
      - `501` → `UNAVAILABLE("not_implemented")`;
      - 5xx → `TEMPORARILY_UNREACHABLE` (`service_unavailable` or
        `http_5xx`, with Retry-After);
      - timeout or connect error → `TEMPORARILY_UNREACHABLE`
        (`timeout`/`request_error`);
      - a non-JPEG body → `UNAVAILABLE("unexpected_media_type")`; an
        oversized one → `UNAVAILABLE("snapshot_too_large")`;
      - a non-canonical ID → `MISSING("invalid_observation_id")` with no
        request.
    - Capabilities are narrowed to what YA-WAMF implements. `annotate`, clips,
      recordings and lifecycle stay False even if lightNVR declares them, with
      the reason `not_implemented_by_consumer`.
  - **Gotchas for W4/W6/W8/W10**:
    - **W4/W6: call `await provider.refresh_capabilities()` at startup and on
      MQTT reconnect.** Until it runs, `parse_live_message` returns `[]` and
      `identity()` raises (unless `source_instance` is set). `history()`,
      `fetch_observation()` and `list_cameras()` load it lazily, but `main.py`
      does not. A failed refresh keeps the last good document.
    - **W4**: a live observation is only a trigger. Call
      `fetch_observation(external_id)` for the box, transform,
      `stream_name_at_capture` and the current evidence state (contract
      §16.2.3), and drop `change_cause == "classification"`. Siblings share
      `asset_id`. Cache the full image once per asset and crop each
      observation by its own box with `cropped=False`.
    - **W4/W8: MQTT broker.** lightNVR events use the existing broker
      settings (`frigate.mqtt_*`); only the topic comes from
      `lightnvr.mqtt_topic_prefix`. Frigate availability and stall logic must
      stay Frigate-only. `capability_info.delivery_topic` lets W8 warn when
      the configured prefix disagrees with lightNVR.
    - **W3/W4: event key characters.** The event key contains `:`.
      `media_cache.sanitize_event_id` strips it, which is unique for
      UUID-based keys, but W3's source-identity cache keys should not rely on
      that.
    - **W6**: `RecoveryRequired.oldest_cursor` can be None, meaning restart
      without a cursor. `cursor="head"` is passed through for a live-only
      start. Malformed history items are skipped with a warning count and the
      cursor still advances. If W6 needs per-item skip reasons it must extend
      `HistoryPage`, which is in W1's `base.py`. A `LightNvrApiError` is
      permanent and a `SourceUnavailableError` is retryable.
    - **W8**:
      - Settings changes do not rebuild the active provider; restart, or
        add a rebuild.
      - `lightnvr_enabled` is already in the settings API. The UI selector
        should write it, and a failure must never flip it.
      - `list_cameras()` uses `GET /api/streams?summary=true&surface=live`,
        paged by 100 for up to 20 pages, which needs only `live.view`.
    - **W10**: read `provider.writer_namespace` (D3) and never hard-code
      `yawamf`. Replace the `write_classification` stub and drop `annotate`
      from `_consumer_capabilities`' unimplemented set.
    - **Shared worktree**: `routers/settings.py` and `registry.py` held W3's
      unstaged hunks. W5's versions were staged as HEAD plus W5 edits
      (`git hash-object -w` + `update-index --cacheinfo`), and W3's hunks were
      left in place. W3's registry hunk (`FrigateProvider(app_settings=…)`
      without a client) still applies on top.
- **files edited/created**:
  - YA-WAMF `backend/app/services/sources/lightnvr_client.py` (new)
  - YA-WAMF `backend/app/services/sources/lightnvr_provider.py` (new)
  - YA-WAMF `backend/app/services/sources/registry.py` (lightNVR kind selection)
  - YA-WAMF `backend/app/config_models.py` (`LightNvrSettings`), `backend/app/config.py`, `backend/app/config_loader.py`
  - YA-WAMF `backend/app/routers/settings.py`, `backend/app/services/settings_update.py` (API fields, redaction)
  - YA-WAMF `backend/openapi.json`, `apps/ui/src/lib/api/generated/openapi.ts` (regenerated)
  - YA-WAMF `docs/setup/environment-variables.md`, `changelog.d/lightnvr-provider.md`
  - YA-WAMF `backend/tests/pure/test_lightnvr_parse.py` (new), `backend/tests/test_lightnvr_provider.py` (new)

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
- **status**: Completed (YA-WAMF commit `7fb42e41` on `feature/lightnvr-provider`, not pushed)
- **log**:
  - **Intent lifecycle** (W2's columns and vocabularies; no Alembic revision):
    - `SourceIngestion` computes the intent with
      `notification_intent.single_shot_intent(origin, notify_mode, notify_on_insert)`:
      - `recovery` always gives `suppressed_recovery`;
      - `silent` gives `suppressed_policy`;
      - `final`, `standard` and `realtime` give `dispatch`;
      - any other mode follows `notify_on_insert`.
    - `DetectionService.save_detection(notification_intent=…)` forwards it
      to `upsert_if_higher_score`. Only the **insert** branch writes
      `notification_intent` and `notification_intent_at`, before the same
      commit, guarded by `notification_intent IS NULL`. So `was_inserted`
      means the intent is newly recorded, and only then does a `dispatch`
      enqueue. A Frigate save passes no intent and its upsert call is
      unchanged.
    - The orchestrator sees `notification_intent` on the event and takes
      `_deliver_recorded_intent`, never the lifecycle `_should_notify`. It:
      - re-reads `get_notification_state`; anything except
        `dispatch` with no outcome is a no-op, so a double enqueue cannot
        double send;
      - records `skipped` for a hidden or unconfirmed row;
      - does not wait for video, because a snapshot-only source has no
        clip;
      - sends with a `NotificationReport`, then calls
        `record_outcome` once (`WHERE notification_intent IS NOT NULL AND
        notification_outcome IS NULL`). `sent` also sets `notified_at`
        (`DetectionRepository.record_notification_outcome`, which extends
        `mark_notified`).
    - Outcomes:
      - `notify_detection` filters and cooldown give `skipped`;
      - any channel sent gives `sent`;
      - an ambiguous error gives `unknown`: `ReadTimeout`, `WriteTimeout`,
        `RemoteProtocolError`, `ReadError`, `TimeoutError`,
        `SMTPServerDisconnected`, or `CancelledError` (the dispatcher's
        `wait_for` deadline hitting a send in flight);
      - otherwise `failed` (for example `ConnectError` or an HTTP status);
      - a queue-full drop records `failed` (`not_queued`).
    - Transport results reach the report through a `ContextVar` that is set
      only while the channel tasks are created. `_note_delivery` is called
      in each `_send_*`. Frigate and Garden calls pass no report and are
      unaffected.
    - Every `unknown` is logged and recorded in `error_diagnostics_history`
      (`reason_code=notification_delivery_unknown`, severity `error`) and
      never retried.
  - **Crash-recovery choice: surface, never resend** (delivery is at most
    once, and exactly once is not promised).
    - The lifecycle phase `notification_intent_reconcile` runs in `main.py`
      after `db_init` and before the dispatcher, MQTT and source recovery.
    - It calls `reconcile_interrupted_intents(started_before=<process start, naive UTC>)`,
      which marks every `dispatch` intent with no outcome and recorded before
      this start as `unknown`.
    - Each one gets a diagnostics entry
      (`notification_intent_interrupted`, severity `warning`), and the count
      appears in `/health` under `notification_intents`
      (`interrupted_at_startup`, `delivery_unknown`, `last_reconciled_at`).
    - Rows recorded after the cutoff are left alone. A row cannot say
      whether the crash came before or during the send, so resending could
      duplicate the alert. A job dropped by `dispatcher.stop()` at shutdown
      therefore also ends up `unknown`, although it was never sent.
  - **Other changes**:
    - **Frigate is untouched**: rows keep NULL intent, and `final` still
      waits for `end`. A test asserts no `report` kwarg on the Frigate path.
    - For a single-shot sighting, `notify_detection` gets `event_type="end"`,
      so **email only on end** still emails: the first admission is also
      the end.
    - `notification_link`: with `link_target="frigate"`, a non-Frigate key
      (`lnvr:`) falls back to the YA-WAMF detection link.
    - Attachments: ingestion already passes the cached crop. The fallback
      stays `media_cache`, then `source_access.fetch_event_snapshot_bytes`,
      which goes through the provider. `snapshot_url` stays `""` for
      lightNVR.
  - **RED → GREEN**:
    - `tests/test_lightnvr_notification_intent.py` (17 tests) first failed
      at collection (`ImportError: notification_intent`).
    - With a stub module, 16 of 17 failed. `final` mode got "awaited 0
      times", intent columns were NULL, and the recovery, silent, crash,
      timeout, deadline, refusal, sibling-cooldown and link tests all failed.
      Only the Frigate-unchanged test passed, as intended.
    - After the change, all 17 pass, plus `tests/pure/test_notification_intent_policy.py`
      (25 cases).
    - Covered:
      - 10× replay gives one intent and one dispatch, also from a fresh
        processor and with recovery overlap;
      - `final` notifies once;
      - recovery gives `suppressed_recovery` and no dispatch, including on a
        later live message;
      - `silent` gives `suppressed_policy`;
      - a crash before dispatch gives `unknown` plus a diagnostic, with no
        resend on a second sweep or a redelivery;
      - the sweep ignores intents recorded after the cutoff;
      - a queue drop gives `failed`;
      - real `notify_detection` against a fake Telegram: `ReadTimeout`
        gives `unknown` plus a diagnostic and no resend; a 0.2 s dispatcher
        deadline mid-send gives `unknown`; `ConnectError` gives `failed`;
        OK gives `sent` with `notified_at`;
      - siblings with a 5 min cooldown give `sent` and `skipped`, and
        without a cooldown both are `sent`.
  - **Gates**:
    - W0 46/46.
    - Notification, ingestion and live-notification modules: 186 passed.
    - Pure lane: 2217 passed, 4 skipped.
    - Rest of the suite: 5686 passed, 56 skipped, 0 failed, run with W6's
      and W8's in-flight hunks in the tree.
    - `ruff check`/`ruff format` are clean on every W7 file. The repo-wide
      `ruff check` flags only W6's in-flight `source_recovery_service.py`
      (unused `HEAD_CURSOR`).
    - `docs_consistency_check.py` and `dead_modules_check.py` are clean.
  - **Commit hygiene**: the shared index already held W8's staged files. The
    commit was therefore built in a private index
    (`GIT_INDEX_FILE`, `read-tree HEAD`, `git add` W7 files, and
    `git apply --cached` with only the three W7 hunks of `main.py`). W6's
    `source_recovery` hunks in `main.py` stay unstaged, and W8's staged set
    is untouched.
  - **Gotchas**:
    - **W9**: assert intent and outcome rows with the `INTENT` query in
      `test_lightnvr_notification_intent.py`. Its fixtures are reused from
      `test_lightnvr_ingestion` by module attribute (a direct import trips
      ruff F811). `intent_status()` counters are process-global, so assert
      deltas. For A4, check one `dispatch` row and one notify call.
    - **W6**: recovery must call `process_source_observation(origin="recovery")`.
      That is what records `suppressed_recovery`; nothing else is needed. A
      recovered item for an already admitted row leaves the live row's
      intent and outcome alone.
    - **W8**: `/health.notification_intents` and the diagnostics reason
      codes `notification_delivery_unknown` and
      `notification_intent_interrupted` are there to surface. No UI was
      added.
    - **W10 / A13**: write-back retries never touch
      `notification_intent`, and a re-save of an existing row cannot record
      one (insert only). Do not call `_enqueue_notification_flow` from
      write-back.
    - **X1**: an unknown outcome is expected after a kill -9 during a send,
      and also for jobs that were still queued at a graceful stop. Do not
      treat it as a bug. Ambiguity is detected only from error type names,
      so a new transport must return `DeliveryResult.error_type` with the
      exception class name.
- **files edited/created** (YA-WAMF):
  - `backend/app/services/notification_intent.py` (new)
  - `backend/app/services/notification_orchestrator.py`,
    `notification_service.py`, `notification_links.py`,
    `source_ingestion.py`, `event_processor.py`, `detection_service.py`,
    `backend/app/repositories/detection_repository.py`, `backend/app/main.py`
    (W7 hunks only)
  - `backend/tests/test_lightnvr_notification_intent.py` (new),
    `backend/tests/pure/test_notification_intent_policy.py` (new)
  - `docs/features/notifications.md`,
    `changelog.d/lightnvr-notification-intent.md` (new)

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
- **status**: Completed (YA-WAMF commit `3cda8dfe` on `feature/lightnvr-provider`, not pushed)
- **log**:
  - **RED→GREEN**:
    - `tests/test_lightnvr_setup_api.py` (26 tests) failed at collection with
      `ImportError: source_setup_service`. Three new `test_setup_state.py`
      cases failed on the Frigate-only connection rules. After the change,
      both files pass (26 and 25).
    - `browser-tests/source-selection.spec.ts` failed 3/3 before the UI
      existed: the clip switch was not `aria-disabled`, and there was no
      source radio or lightNVR field. It now passes 5/5, with two Health
      cases added. `utils/source-features.test.ts` passes 8.
  - **Gates**, run on the exact commit content in an isolated worktree, because
    W6 and W7 had unstaged edits in the shared tree:
    - Full backend suite: 7816 passed, 60 skipped, 0 failed. This includes W0
      (`test_frigate_regression_baseline.py`), `test_dead_modules`, the pure
      lane (2165 passed in the shared tree) and the startup test.
    - Clean: `export_openapi.py --check`, `export_openapi_types.py --check`,
      `docs_consistency_check.py`, `assemble_changelog.py`, and repo-wide
      `ruff check` and `ruff format --check`.
    - Frontend: `npm run check` 0/0, `npm test` 2353 passed,
      `npm run lint:dead` clean.
    - Playwright chromium, full suite in the shared tree: 638 passed, 2
      skipped. `explorer-search:258` and `family-tree:595` failed under load
      and pass when rerun alone. `setup-wizard.spec.ts` passes 28/28.
    - `npm ci` was run in `apps/ui`, and `playwright install
      chromium-headless-shell` added build 1248, which `@playwright/test`
      1.64.1 needs.
  - **Endpoints**: all owner-only, all in `docs/api.md`.
    - `GET /api/setup/source` returns `SourceStatus`: `active_kind`,
      `configured_kind`, `restart_required`, `capabilities_loaded`,
      `capabilities_error` and `features[{name,supported,reason}]`.
      - `restart_required` is true when the saved kind differs from the
        running one, or when the running lightNVR client's URL differs from
        the saved `api_url`.
      - A lightNVR provider whose capabilities never loaded is refreshed once
        here.
    - `POST /api/setup/lightnvr/test` takes `{api_url, token, mqtt_topic_prefix,
      camera_uuids, mqtt_server/port/auth/username/password}` and returns
      `LightNvrConnectionTest{ok, results[], features[], latest_observation}`.
      - Each result is `{id, status: ok|warning|error|skipped, code, message,
        values}`, with ids `api_access`, `event_connectivity`,
        `evidence_access` and `features`, in that order.
      - Nothing is stored. A blank or `***REDACTED***` token means the saved
        token, and only when the on-screen URL normalises to the saved URL.
    - `POST /api/setup/lightnvr/cameras` takes `{api_url, token}` and returns
      `{cameras:[{camera_uuid,name}]}` from `provider.list_cameras()`. It
      answers `502` with a plain message when the list cannot be read.
    - `GET /api/diagnostics/source` returns `SourceDiagnostic{status,
      latest_observation, skipped_workflows[]}`. The skipped workflows come
      from `capability_gate.recorded_capability_skips()`.
  - **Check codes** (`app/services/source_setup_service.py`):
    - `api_access`:
      - from the capabilities GET: `unreachable`, `not_configured`,
        `unauthorized` (401), `forbidden` (403), `not_supported` (404),
        `http_error`, `invalid_document` (rejected by
        `apply_capabilities`);
      - `history_not_authorized`;
      - `camera_forbidden` (cameras in `values.cameras`): the contract setup
        probe `GET /api/observations?camera_uuid=X&limit=1`, run per selected
        camera (up to 10, or once unfiltered), answered 403. It is an error,
        never an empty page.
    - `event_connectivity`:
      - `broker_not_configured`;
      - `broker_unreachable`, from `mqtt_service.test_connection` with the
        shared `frigate.mqtt_*` broker;
      - `topic_unchecked` (warning; the API failed);
      - `lightnvr_mqtt_disabled`;
      - `topic_unknown` (warning);
      - `topic_prefix_mismatch` (values `configured` and `advertised`). The
        advertised prefix is `delivery.topic` before
        `/v1/events/<type>/`.
    - `evidence_access`:
      - `snapshot_not_supported` or `snapshot_not_authorized`, read from the
        raw `operations["observations.snapshot"]`;
      - `cameras_without_photos` (warning): a selected camera that is absent
        from `cameras[]`, has `evidence_enabled` false, or lacks
        `snapshot.create`.
    - `features` is always `ok` (`listed`). Unsupported features never fail
      the test.
    - `ok` means `api_access` is ok and no result is an error.
  - **Latest eligible observation**:
    - lightNVR has no newest-first order, so the search widens `since`
      windows (1 h, 24 h, 7 d, 30 d). It reads up to 3 pages each at
      `history_page_max` through `provider.history()` and takes the newest
      `observed_at` in the first non-empty window.
    - The labels are `frigate.ingest_labels`, at most 8, without commas.
    - The search covers only cameras whose probe passed.
    - `state` values:
      - `found`, with `photo_state`: ready, pending, missing (lightNVR
        `unavailable`, with `photo_reason`) or expired;
      - `none_yet`;
      - `error`: `unreachable`, `forbidden`, `refused` or `invalid_filter`;
      - `not_checked`.
  - **UI**:
    - New `components/settings/SourceSettings.svelte`, the top card on the
      Connection tab:
      - a Frigate/lightNVR radio that binds `lightnvr_enabled`;
      - address, token (`SecretInput`) and topic prefix fields;
      - the bird-capable detector note (`data-source-bird-detector`);
      - a test button that opens `DiagnosticDialog` with the 4 checks plus a
        latest-detection stage (`none_yet` counts as passed);
      - a camera picker (`data-lightnvr-cameras`) that keeps chosen-but-
        unlisted UUIDs visible;
      - a restart note (`data-source-restart`) for an unsaved change or a
        saved-but-not-running source.
    - `Settings.svelte` loads and saves `lightnvr_*`, sends the token only
      when typed, includes it in the dirty checks, and fetches
      `/api/setup/source` with each settings load.
    - `ConnectionSettings` (Fetch Video Clips, full-visit) and
      `DetectionSettings` (auto video) take an optional `sourceStatus` prop.
      An unsupported control is `aria-disabled`, shown unchecked without
      changing the saved value, and gets a neutral
      `SourceUnsupportedNote` (`data-source-unsupported=event_clip|range_clip|
      video_analysis`). An unread status disables nothing.
    - New `components/settings/SourceHealthCard.svelte` on the Health page
      (`Errors.svelte`). It renders only when lightNVR is active or
      configured, so Frigate installs look unchanged. It shows the latest
      detection and photo state, says "no bird detections yet" as its own
      sentence, and lists unsupported features with their skip counts and
      "do not affect health".
    - Wizard:
      - `ConnectionStep` has the source radio and the lightNVR fields, saves
        and then runs the lightNVR test inline. It writes `lightnvr_enabled`
        only when lightNVR is chosen or was saved, so the Frigate payload is
        unchanged.
      - `CamerasStep` uses lightNVR discovery and saves
        `lightnvr_camera_uuids`.
      - `ReviewStep` knows `connection_lightnvr_ready`, `lightnvr_url` and
        `lightnvr_token`.
    - `setup_state_service`: with lightNVR enabled, the connection section
      needs `api_url`, the token and the MQTT broker (no Frigate URL), and the
      cameras section counts `camera_uuids`.
    - The `settings.source.*` i18n block is in all 9 locales; French uses
      U+202F before `:`.
    - The test-only `browser-tests/settings-page-mock.ts` `openSettings` now
      takes a third `handlers` map. Playwright gives the last-registered route
      priority, so extra `page.route`s registered before `openSettings` are
      shadowed by its catch-all.
  - **Restart, not rebuild**: changing the source or the lightNVR settings
    still needs a restart (W5's design). The UI says so, and no hot provider
    rebuild was added.
  - **Gotchas for W9/X1**:
    - The connection test builds its own `LightNvrProvider` per call
      (`source_setup_service.make_lightnvr_client` is the patch point for a
      mocked transport). `probe_mqtt_broker` is the broker patch point. The
      active provider is never touched, except `/api/setup/source`, which
      lazily runs `refresh_capabilities()` once.
    - `/health` does not read capability skips. W8 adds a test that recorded
      skips leave `build_health_payload()["status"]` unchanged. Keep it that
      way.
    - A9 UI half: the browser fixture serves a snapshot-only `SourceStatus`
      through the `handlers` map. W9's lightNVR-kind acceptance can reuse
      `SNAPSHOT_ONLY` from `source-selection.spec.ts`.
    - The latest-observation search is bounded (at most 12 history requests)
      and approximate when a window has more than 3 pages. That is fine for
      a diagnostic; do not reuse it for recovery.
    - Live MQTT delivery is not observed end to end: event connectivity means
      the broker accepts a session and the prefixes agree. X1 should prove an
      actual `observation_changed.v1` arrives.
    - Shared worktree: W6 and W7 had unstaged hunks in the same
      `locales/*/settings.json` and `docs/api.md`. W8's index blobs were built
      as HEAD plus the W8 block (`hash-object` + `update-index --cacheinfo`),
      and their hunks were left unstaged in the tree.
- **files edited/created**:
  - YA-WAMF `backend/app/services/source_setup_service.py` (new)
  - YA-WAMF `backend/app/routers/setup.py`, `backend/app/routers/diagnostics.py`, `backend/app/services/setup_state_service.py`
  - YA-WAMF `backend/openapi.json`, `apps/ui/src/lib/api/generated/openapi.ts` (regenerated)
  - YA-WAMF `backend/tests/test_lightnvr_setup_api.py` (new), `backend/tests/test_setup_state.py`
  - YA-WAMF `apps/ui/src/lib/api/source.ts` (new), `apps/ui/src/lib/api/settings.ts`
  - YA-WAMF `apps/ui/src/lib/utils/source-features.ts` and `source-features.test.ts` (new)
  - YA-WAMF `apps/ui/src/lib/components/settings/SourceSettings.svelte`, `SourceHealthCard.svelte`, `SourceUnsupportedNote.svelte` (new); `ConnectionSettings.svelte`, `DetectionSettings.svelte`
  - YA-WAMF `apps/ui/src/lib/components/setup/ConnectionStep.svelte`, `CamerasStep.svelte`, `ReviewStep.svelte`
  - YA-WAMF `apps/ui/src/lib/pages/Settings.svelte`, `apps/ui/src/lib/pages/Errors.svelte`
  - YA-WAMF `apps/ui/src/lib/i18n/locales/{de,en,es,fr,it,ja,pt,ru,zh}/settings.json`
  - YA-WAMF `apps/ui/browser-tests/source-selection.spec.ts` (new), `apps/ui/browser-tests/settings-page-mock.ts`
  - YA-WAMF `docs/api.md`, `changelog.d/lightnvr-setup.md`

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
- **status**: Completed
- **log**:
  - Commit `3b86a6db`. Before the change, bit 19 was free (the max
    `bit_index` was 18, from 0078) and 0096 was the next migration number.
  - RED: `test_authorization` failed 2 of 52 (catalog vs DB and the bit
    layout), and `test_db_observation_classifications` did not link.
  - GREEN in `build-int01-L13` and `build-int01-L13-asan` (ASan + UBSan):
    - `test_db_observation_classifications` 12/12
    - `test_authorization` 52/52
    - `test_observation_migration` 4/4
    - `test_db_observations` 15/15
    - `test_sqlite_migration_boundaries` 21/21
    - `test_audit_log` 42/42
    - `test_api_handlers_observations` 18/18
    - `test_db_auth` 36/36
  - Contract tooling: `generate.mjs` and `generate.mjs --check` pass,
    `lint.mjs` is valid (only the existing warnings), and the
    `tests/contracts/tooling` tests pass 59/59. Web: jest
    `apiTokenPolicy.spec.js` passes 16/16.
  - **Schema.** Columns follow the contract rather than the plan's
    `writer_principal`:
    - `writer_namespace` plus `writer_user_id` (D3), with no FK to users.
    - Times are `_ms` integers: `classified_at_ms`, `created_at_ms` and
      `updated_at_ms`.
    - `observation_uuid` has an FK to `object_observations` with ON DELETE
      CASCADE, so pruning removes annotations.
    - A CHECK requires the key to be prefixed with `writer_namespace/`.
    - `model_version` and `taxonomy_id` are stored as NULL when absent (the
      API maps "" to null).
  - **DB API** (`include/database/db_observation_classifications.h`):
    - `db_observation_classification_put(obs_uuid, &input, &result)`:
      - The input carries `writer_namespace`, `writer_user_id`, the `name`
        segment, the request fields, `kind` and `expected_revision` (0 means
        absent).
      - The result holds `has_annotation`, the annotation (current or
        stored), `observation_revision`, `change_seq`,
        `observation_asset_uuid`, `evidence_state` and `limit`.
    - `db_observation_classifications_list(obs_uuid, &set)` returns
      automated and owner_confirmed lists, each ordered by `updated_at` DESC.
      It uses a read-only connection.
  - **Rule order:** observation found → writer check → evidence check
    (`input_asset_id`, `1 ≤ input_revision ≤ revision`, state ready or
    expired) → identical retry → owner lock → CAS → 8-key limit → store,
    then bump with `classification` and call `observation_relay_notify()`
    after COMMIT.
  - **Mapping for L14:**

    | DB result | HTTP response |
    | --- | --- |
    | `CREATED` | 201 |
    | `UPDATED` | 200 `changed:true` |
    | `NOOP` | 200 `changed:false` |
    | `REVISION_CONFLICT` | 409 `revision_conflict` + `current` when `has_annotation` |
    | `OWNER_LOCKED` | 409 `owner_decision_locked` + `current` |
    | `EVIDENCE_MISMATCH` | 422 + `observation_asset_id`, `evidence_state` |
    | `WRITER_MISMATCH` | 403 `namespace_forbidden` (the annotation is withheld) |
    | `LIMIT` | 409 `annotation_limit` + `limit` |
    | `INVALID` | 400 |
    | `NOT_FOUND` | 404 |
    | `ERROR` | 500 |

    The audit `result` strings match one to one.
  - **Enforcement flag (deviation from the contract tuple).** The catalog
    entry is currently `{..., true, false, false}`, meaning *unenforced*:
    per `authorization.h`, an action is not reported enforced until a handler
    routes it through the evaluator. As a result, the token UI hides it. L14
    must:
    - flip the last field to `true`;
    - update `web/tests/apiTokenPolicy.spec.js`, where the catalog entry is
      `enforced: false`;
    - move the key from `unenforcedKeys` to `enforcedKeys` in
      `tests/integration/specs/api-tokens.ui.spec.ts`.
  - **OpenAPI.** Only two OpenAPI action enums exist:
    - `AuthorizationAction`;
    - the `AdminSimulationRequest` camera-scoped action enum.

    The third `lpr.delete` hit is an operation requirement, not an enum.
  - **Migration test.** The 0095 up/down test in
    `test_observation_migration.c` now migrates through 0095 explicitly, not
    to the latest migration.
  - **L15 decision: no `observation_uuid` column on `detections`.** L6 is
    asynchronous: observation UUIDs are generated by the L3 worker's
    `db_observation_insert_batch`, after `store_detections_in_db_for_camera`
    has already written the detection rows on a separate path. Populating the
    column cheaply is therefore not possible. L15 should join on:
    - `detections.camera_uuid = object_observations.camera_uuid`;
    - `object_observations.observed_at_ms / 1000 BETWEEN detections.timestamp
      - 1 AND detections.timestamp + 1`, because `observed_at_ms` is the
      frame second × 1000, or wall-clock ms when that falls in the same
      second;
    - case-insensitive equal labels.

    Use `idx_object_observations_camera_time`, and pick the nearest
    `observed_at_ms` when siblings tie.
- **files edited/created**:
  - `db/migrations/0096_add_observation_classifications.sql`
  - `include/database/db_embedded_migrations.h`
  - `include/database/db_observation_classifications.h`
  - `src/database/db_observation_classifications.c`
  - `include/core/authorization.h`
  - `src/core/authorization.c`
  - `docs/api/openapi.json`
  - `include/web/openapi_document_generated.h`
  - `web/generated/api/schema.d.ts`
  - `web/generated/api/validators.js`
  - `tests/unit/test_db_observation_classifications.c`
  - `tests/unit/test_authorization.c`
  - `tests/unit/test_observation_migration.c`
  - `tests/unit/CMakeLists.txt`
  - `web/tests/apiTokenPolicy.spec.js`
  - `tests/integration/specs/api-tokens.ui.spec.ts`

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
- **status**: Completed
- **log**:
  - Commit `9e0e8ecd`. Built in `build-int01-L14` (mirrors `build-int01-L13`)
    and `build-int01-L14-asan` (Debug ASan/UBSan, `-DENABLE_LITERT=OFF`).
    `libuv_api_handlers.c` did not change, so `coverage.json` did not move
    (`coverage.mjs --check` OK).
  - **Rule order in the handler** (contract §10.4):
    1. Authenticate, parse the raw path (a malformed ID is `400 field:id`, not
       audited), then load the observation.
    2. Evaluate `observations.annotate` on the capture camera with
       `httpd_evaluate_camera_identity_action`. On a deny it re-checks
       `live.view`: held → `403 forbidden`, otherwise `404
       observation_not_found`, byte-identical to an unknown ID. This runs
       before the namespace and body checks, so a hidden observation with a
       bad namespace and a broken body is still that same `404`. Holding
       annotate without `live.view` is enough to write.
    3. Namespace: `observation_writer_namespace(username)` must succeed and
       equal the path namespace. A body `writer_namespace` or `writer`
       naming anyone else is `403 namespace_forbidden`; naming the caller is
       accepted and ignored. A non-string writer field is `400`.
    4. Body: at most `OBS_PUT_BODY_MAX` (4096) bytes, a JSON object, closed.
       Text must be valid UTF-8 without control characters. Optional
       `model_version` and `taxonomy_id` may be absent or null, and `""` is
       stored as null. `input_asset_id` is lowercased; integers must be
       integral and ≤ `INT_MAX`; `expected_revision: null` is `400`. Every
       400 carries `field` (an unknown key names itself; an invalid name
       segment gives `field: "name"`).
    5. `db_observation_classification_put`, with the L13 mapping exactly.
       `WRITER_MISMATCH` withholds the stored values.
  - **Audit**: `audit_log_operation(..., "observations.annotate",
    "observation", id, "annotate", outcome, context)`. The context holds
    `camera_uuid`, `key`, `kind`, `result`, `annotation_revision_before` and
    `annotation_revision_after` (null when unknown). `audit_events.outcome`
    only allows `allowed|denied|success|failure|error`, so the contract
    result lives in `context.result` and maps to an outcome:
    - `success`: created, updated, noop;
    - `denied`: forbidden, namespace_forbidden, and the hidden 404;
    - `error`: internal_error;
    - `failure`: everything else.

    The result set is the §12.2 list plus `forbidden`,
    `observation_not_found`, `invalid_request`, `annotation_limit` and
    `internal_error`. Contract §9, §11 and §12.2 are updated to match.
  - **Reads**: `observation_record_to_json(record, source, set)` gained a
    third argument (NULL means empty lists). The new
    `observation_classification_to_json()` never emits `writer_user_id`.
    GET uses `db_observation_classifications_list`. History pages use the
    new `db_observation_classifications_list_many(ids, n, visit, ctx)`, which
    uses one read-only connection and one prepared statement per page rather
    than one connection per item.
  - **Capabilities**: `observations.annotate` is `supported: true`.
    `authorized` requires both a writer namespace and the action on at least
    one camera. Otherwise `reason` is `reserved_writer_namespace` or
    `invalid_writer_namespace` (these win) or `not_authorized`. `cameras`
    now includes cameras where only annotate is held, and `actions` appends
    `observations.annotate`.
  - **Authz**: the catalog entry is now `{..., true, false, true}`
    (enforced). `test_authorization` asserts this.
    `web/tests/apiTokenPolicy.spec.js` and
    `tests/integration/specs/api-tokens.ui.spec.ts` now list it as enforced
    (the checkbox count went from 16 to 17).
  - **OpenAPI**: the PUT is camera `observations.annotate` (raw). The `501`
    and the P2 note are gone, and the 403/404/409/422 descriptions are
    tightened. The `not_implemented` code and the
    `not_available_in_this_version` reason stay in the enums (the P1
    capability fixture uses them). `observations-contract.test.mjs` now
    expects no `501` on any operation and checks
    `validateObservationClassificationWrite`.
  - **RED→GREEN**:
    - RED: with the extended tests and only the signature change,
      `test_api_handlers_observations` failed 11/25. The failures were the
      serializer against the annotated fixtures, both capabilities tests,
      and every PUT test (`501`).
    - GREEN: 25/25 in both builds, ASan and UBSan clean.
    - Also green in both builds: `test_authorization` 52,
      `test_db_observation_classifications` 12, `test_api_handlers_openapi`
      8, `test_openapi_projection` 21, `test_db_observations` 15,
      `test_event_envelope` 13, `test_observation_relay` 13, `test_audit_log`
      42 and `test_db_auth` 36. `test_authorization_snapshot` 10 passes in
      `build-int01-L14`.
  - **Unit coverage**:
    - C3 fixtures 01–05 run as one chain. Fixture IDs are mapped to the seeded
      ones; server `created_at`/`updated_at` are checked, then aligned, so
      the rest is exact `cJSON_Compare` equality.
    - Fixtures 06–08: no rows, no revision bump and no publication.
    - The serializer is exact against all 10 observation fixtures, including
      the classified and owner-confirmed ones.
    - A7 denial matrix: 404 identical to an unknown ID, 403, annotate-only
      allowed, and an evaluation failure is 500.
    - Namespace cases: reserved, invalid, body writer fields, and D3 reuse by
      user id 43.
    - 34 body-field cases plus whole-document, oversize and name cases.
    - The absent-key CAS (`409` without `current`), a 200 update and the
      9th-key `annotation_limit`.
    - GET and list return the annotations.
    - The staged publication has cause `classification` and message ID
      `observation_message_id(source, id, 3)`. Envelopes built twice match,
      and a no-op retry stages nothing.
    - Audit context fields are checked, and the body never appears in them.
  - **Playwright** (runnable locally): new
    `tests/contracts/observations.api.spec.ts`, 3/3 with
    `LIGHTNVR_CONTRACT_BIN=build-int01-L14/bin/lightnvr
    LIGHTNVR_CONTRACT_MQTT=1 npx playwright test -c
    tests/contracts/playwright.config.ts --project=api
    tests/contracts/observations.api.spec.ts`. Against the old
    `build-int01-L8` binary it fails 3/3.
    - It seeds ready observations by direct SQL into the isolated fixture DB.
    - It covers A12, and A7-annotate with a policy user `yawamf`, the fixture
      viewer, and scoped tokens with and without the action.
    - It covers the MQTT case: a real sink receives exactly one
      `classification` envelope whose `id` equals the §5.4 UUIDv5, and a
      no-op retry adds nothing.

    `discovery.api.spec.ts`, `baseline.spec.ts` and
    `eventsDelivery.api.spec.ts` still pass (18/18). `permissionCases.ts`
    needed no change.
  - **Other validation**: `generate.mjs` and `--check` pass; `lint.mjs` is
    valid with the same 5 warnings; `node --test
    tests/contracts/tooling/*.test.mjs` 59/59; `tsc -p
    tests/contracts/tsconfig.json` is clean; jest `apiTokenPolicy.spec.js`
    16/16.
  - **Gotchas for later tasks**:
    - **L15**: read annotations from `GET /api/observations/{id}` (or a
      history page), never from the PUT. Each list is already ordered by
      `updated_at` desc. Show `owner_confirmed[0]` over `automated[0]`, and
      keep the top-level `label`/`confidence` (the detector's) visible.
      `observation_classification_to_json()` is the C serializer if a
      detection API join needs it.
    - **W10**: read `principal.writer_namespace` and
      `operations["observations.annotate"].authorized` from capabilities.
      The recipe user needs a role with `observations.annotate`. Built-in
      Administrator has it; nothing else does by default, and legacy
      user/viewer roles never do. A scoped token must list the action
      explicitly. A `403 namespace_forbidden` on an existing key means
      another user id created it, which is permanent. A `409 revision_conflict`
      without `current` means `expected_revision` was non-zero on an absent
      key. A retry after a lost response is a `200 changed:false` whatever
      `expected_revision` says.
    - **L11 / X3**: `observations.api.spec.ts` seeds rows by SQL, bypassing
      L3, and its MQTT case is skipped unless `LIGHTNVR_CONTRACT_MQTT=1`.
      L11 may fold it into the broader suite. Observation events need a
      route listing `io.lightnvr.detection.observation_changed.v1` with no
      label predicate (L5 gotcha). Classification events carry the
      detector `label`, not the species.
    - **Audit consumers**: filter on `action = observations.annotate` and read
      `details.context.result`. The `outcome` column is the coarse 5-value
      enum.
- **files edited/created**:
  - `src/web/api_handlers_observations.c`, `include/web/api_handlers_observations.h`
  - `src/database/db_observation_classifications.c`, `include/database/db_observation_classifications.h` (`_list_many`)
  - `src/core/authorization.c`, `tests/unit/test_authorization.c`
  - `docs/api/openapi.json`; generated `include/web/openapi_document_generated.h`, `web/generated/api/schema.d.ts`
  - `docs/integrations/OBSERVATIONS_CONTRACT.md` (§9, §11 and §12.2 only)
  - `tests/unit/test_api_handlers_observations.c`, `tests/unit/CMakeLists.txt` (`--wrap=audit_log_operation`)
  - `tests/contracts/observations.api.spec.ts` (new), `tests/contracts/tooling/observations-contract.test.mjs`
  - `web/tests/apiTokenPolicy.spec.js`, `tests/integration/specs/api-tokens.ui.spec.ts`

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
