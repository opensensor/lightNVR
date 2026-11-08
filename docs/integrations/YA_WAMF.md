# YA-WAMF with lightNVR: deployment recipe

This recipe connects lightNVR (recording and bird detection) to
[YetAnother-WhosAtMyFeeder](https://github.com/Jellman86/YetAnother-WhosAtMyFeeder)
(YA-WAMF: species identification, photographs, visits and notifications). It
is the minimal deployment PRD
[INT_01 §9](../prd/INT_01_Event_Enrichment_YAWAMF.md#9-rollout-and-compatibility)
asks for: API permissions, broker routes, bird detection, retention,
diagnostics and rollback.

The wire behaviour is normative in the
[observations contract](OBSERVATIONS_CONTRACT.md). This page links to it and
does not restate it.

> **Status.** Both sides are on unreleased branches. X1 has run this recipe end
> to end against real processes, with synthetic fixtures, a pinned detector
> and a pinned classifier
> ([`tests/integration/yawamf_e2e`](../../tests/integration/yawamf_e2e/README.md)).
> The A14 pilot on a real camera has not happened yet. See the
> [supported-version matrix](#supported-version-matrix).

## Contents

1. [How the pieces fit](#how-the-pieces-fit)
2. [Prerequisites](#prerequisites)
3. [Step 1: a bird-capable detector](#step-1-a-bird-capable-detector)
4. [Step 2: turn on evidence per camera](#step-2-turn-on-evidence-per-camera)
5. [Step 3: MQTT and event routes](#step-3-mqtt-and-event-routes)
6. [Step 4: a lightNVR user and scoped token for YA-WAMF](#step-4-a-lightnvr-user-and-scoped-token-for-ya-wamf)
7. [Step 5: configure YA-WAMF](#step-5-configure-ya-wamf)
8. [Step 6: verify](#step-6-verify)
9. [Optional: send species back to lightNVR (P2)](#optional-send-species-back-to-lightnvr-p2)
10. [Retention and budgets](#retention-and-budgets)
11. [Diagnostics](#diagnostics)
12. [Delivery guarantees](#delivery-guarantees)
13. [Rollback](#rollback)
14. [Supported-version matrix](#supported-version-matrix)

## How the pieces fit

1. A lightNVR object detector reports a `bird` on a decoded frame.
2. If evidence is on for that camera, lightNVR stores a durable **observation**
   for each eligible object: its label, confidence and box, and one JPEG
   **evidence** image shared by all objects found in that frame.
3. Each revision of an observation is published as
   `io.lightnvr.detection.observation_changed.v1` on
   `<topic_prefix>/v1/events/io.lightnvr.detection.observation_changed.v1/<camera_uuid>`
   ([contract §6](OBSERVATIONS_CONTRACT.md#6-change-event-iolightnvrdetectionobservation_changedv1)).
4. YA-WAMF treats the event as a trigger only. It reads the observation with
   `GET /api/observations/{id}`, fetches the image with
   `GET /api/observations/{id}/snapshot`, crops each bird by its own box and
   classifies it.
5. When YA-WAMF misses events (a broker outage or a restart), it pages
   `GET /api/observations` with a cursor
   ([contract §7](OBSERVATIONS_CONTRACT.md#7-history-and-cursor-get-apiobservations)).
6. Optionally (P2), YA-WAMF writes the species back as a classification
   annotation, which lightNVR shows beside the detection.

**What the evidence is.** It is the low-resolution image the detector actually
ran on: the decoded detection frame at the detection stream's resolution for
local models, or the exact JPEG lightNVR sent to an HTTP detector. It is not a
frame from the recording. Boxes are normalized to this stored image, and the
`transform` records how it maps to the source and inference sizes
([contract §3.2](OBSERVATIONS_CONTRACT.md#32-box-and-coordinate-space),
[§4.4](OBSERVATIONS_CONTRACT.md#44-transform)). YA-WAMF never replaces
missing evidence with a current frame.

## Prerequisites

- lightNVR built from a commit listed in the
  [matrix](#supported-version-matrix), with MQTT support (`-DENABLE_MQTT=ON`,
  the default for full builds). Migrations `0095` and `0096` run automatically on first start.
- An MQTT broker both applications can reach, for example Mosquitto.
- YA-WAMF built from the `feature/lightnvr-provider` branch listed in the
  matrix.
- An administrator login on lightNVR. Steps 2–4 need `camera.configure`,
  `events.configure` and `users.manage`, which the built-in Administrator role
  holds.

## Step 1: a bird-capable detector

YA-WAMF only receives what lightNVR's detector reports. The model must output a
`bird` class, or another label that YA-WAMF acts on (YA-WAMF's
`frigate.ingest_labels`, default `bird`). A model that only finds people or
cars produces nothing to identify.

| Detector | Gives usable evidence | Notes |
| --- | --- | --- |
| Local SOD model (`.sod`) | yes | The image is the decoded frame; the transform mode is `resize`. VOC-class models include `bird`. |
| Local TFLite model on LiteRT (`.tflite`) | yes | Needs a `-DENABLE_LITERT=ON` build and `[detection_engine] enabled = true` ([LiteRT](../CONFIGURATION.md#in-process-litert-engine)); the transform mode is `letterbox`. COCO-class models include `bird`. |
| HTTP API detector (light-object-detect or DOODS2) | yes | The image is the JPEG that was submitted. See the [wire formats](../CONFIGURATION.md#wire-formats). |
| ONVIF camera events | no | Observations are recorded with evidence `unavailable(no_matching_media)` and no box. YA-WAMF skips them with that reason. |
| Motion | no | Motion results never create observations. |

- The detector runs inside the stream's detection lifecycle, so set
  `detection_based_recording: true` and `detection_model` on the stream (with
  `record: true` as well if you want continuous recording plus detection
  annotations). See [SOD integration](../SOD_INTEGRATION.md) and
  [Configuration](../CONFIGURATION.md#api-detection-settings).
- In the stream API, `detection_threshold` is a percentage from 0 to 100
  (`50` means 50%). A fraction such as `0.5` is read as 0.5% and lets almost
  every low-confidence box through, and each one becomes an observation.
- With several engines on one stream, each local object engine submits its own
  observations. Extra HTTP `api` engines are isolated in multi-engine mode and
  produce no evidence ([Detection engines](../DETECTION_ENGINES.md#observation-evidence-int_01)).

## Step 2: turn on evidence per camera

Evidence is **off by default** on every camera. Turn it on only for feeder
cameras. While no camera has it on, lightNVR allocates no queue, starts no
worker and writes no images
([contract §13](OBSERVATIONS_CONTRACT.md#13-budgets-and-configuration-provisional)).

In the UI, open the camera's settings and expand **Observation evidence for
integrations**. Or use the stream API, which needs `camera.configure` on that
camera:

```http
PUT /api/streams/feeder-east
Content-Type: application/json

{
  "observation_evidence_enabled": true,
  "observation_labels": "bird",
  "observation_max_age_days": 7,
  "observation_max_bytes": 2147483648
}
```

| Field | Default | Bounds |
| --- | --- | --- |
| `observation_evidence_enabled` | `false` | boolean |
| `observation_labels` | `""` (every label) | at most 16 comma-separated labels of 1–31 printable ASCII bytes; matched case-insensitively |
| `observation_max_age_days` | `7` | 1–90 |
| `observation_max_bytes` | `2147483648` (2 GiB) | 67108864 (64 MiB) to 1099511627776 (1 TiB) |

Out-of-range values return `400`. A valid request answers `202 Accepted` and
the change applies asynchronously, without a restart; confirm it with
`GET /api/streams/{name}/full`. Stream names may contain only letters, digits,
`.`, `-` and `_`. Set
`observation_labels` to the labels YA-WAMF acts on, so other objects use no
evidence budget.

Record each feeder camera's `camera_uuid` from `GET /api/streams`. YA-WAMF
identifies cameras by UUID, not by name.

## Step 3: MQTT and event routes

1. Enable the default broker in `lightnvr.ini` and restart lightNVR
   ([MQTT settings](../MQTT_INTEGRATION.md#2-configure-lightnvr)):

   ```ini
   [mqtt]
   enabled = true
   broker_host = mqtt
   broker_port = 1883
   topic_prefix = lightnvr
   qos = 1
   ```

   `topic_prefix` must equal YA-WAMF's `LIGHTNVR__MQTT_TOPIC_PREFIX`. lightNVR
   advertises the topic it uses in `delivery.topic` of
   `GET /api/integrations/capabilities`, and YA-WAMF's connection test reports
   `topic_prefix_mismatch` when they differ. YA-WAMF reads the topic from the
   default `[mqtt]` prefix, so use the `mqtt:default` destination.

2. **Event routes.** With no event routes stored, lightNVR publishes every
   normalized event, observation changes included, and nothing else is needed.
   Once **any** route exists, only events that match an enabled route are
   published. In that case add a route for the observation event
   ([event routes](../API.md#event-routes); needs `events.configure`):

   ```http
   POST /api/event-routes
   Content-Type: application/json

   {
     "name": "YA-WAMF observations",
     "destination": "mqtt:default",
     "event_types": ["io.lightnvr.detection.observation_changed.v1"],
     "camera_scope": {
       "type": "selector",
       "selector": {
         "version": 1,
         "expression": {"op": "camera_uuid", "values": ["<feeder camera_uuid>"]}
       }
     }
   }
   ```

   Do **not** add a detection predicate (`labels_any`, `zone_ids_any`,
   `min_confidence`). Predicates only match `io.lightnvr.detection.object.v1`,
   so a route with one never matches observation events, and those changes are
   recorded as `relayed_no_route` instead of being published. For the same
   reason, do not add the observation type to an existing route that has a
   label predicate. Leave suppression at zero: debounce, cooldown or grouping
   would drop observation revisions, which YA-WAMF then recovers only through
   history.

Existing detection topics, snapshots and Home Assistant topics are unchanged
([MQTT integration](../MQTT_INTEGRATION.md#observation-change-events-int_01)).

## Step 4: a lightNVR user and scoped token for YA-WAMF

YA-WAMF authenticates with one scoped API token. Give it a dedicated user.

1. **Create the user** (Users page, or `POST /api/auth/users`), for example
   username `yawamf` with the Viewer role (`"role": 2`). The username matters
   for P2: lightNVR derives the writer namespace for annotations from it
   (lowercased, characters outside `[a-z0-9_.-]` replaced by `-`). `lightnvr`,
   `system`, `owner` and `local` are reserved and give no namespace
   ([contract §10.2](OBSERVATIONS_CONTRACT.md#102-writer-namespace-and-identity)).
2. **Create a role** with the actions YA-WAMF needs
   (`POST /api/authorization/roles`, [roles](../API.md#manage-authorization-roles)).
   Read `policy_version` from `GET /api/authorization/roles` first and send it
   as `expected_policy_version`; without it the request fails with `400`:

   ```json
   {
     "expected_policy_version": 12,
     "name": "YA-WAMF",
     "description": "YA-WAMF feeder integration",
     "actions": ["live.view", "snapshot.create", "observations.annotate"]
   }
   ```

   | Action | Needed for |
   | --- | --- |
   | `live.view` | camera list, observation history and single reads |
   | `snapshot.create` | evidence images (`/snapshot`) |
   | `observations.annotate` | P2 write-back only |

   Only the built-in Administrator role holds `observations.annotate` by
   default; no other built-in role has it.
3. **Grant the role to the user on the feeder cameras only**
   (`PUT /api/authorization/users/{user_id}`,
   [user policy](../API.md#manage-a-users-authorization-policy)), with a
   `camera_uuid` selector, a tag selector or a shared collection as the scope.
   This replaces the Viewer grant, so the user sees nothing else. As with the
   role, send the `policy_version` from `GET /api/authorization/users/{user_id}`
   as `expected_policy_version`, with `"mode": "policy"` and the complete
   `grants` list.
4. **Issue a scoped token** for that user from **Users → Manage API access**,
   or with `POST /api/authorization/users/{user_id}/tokens`
   ([scoped tokens](../API.md#manage-scoped-api-tokens)):

   ```json
   {
     "description": "YA-WAMF feeder integration",
     "expires_at": 1819075200,
     "actions": ["live.view", "snapshot.create", "observations.annotate"],
     "scope": {
       "type": "selector",
       "selector": {"version": 1, "expression": {"op": "camera_uuid", "values": ["<feeder camera_uuid>"]}}
     }
   }
   ```

   The token must list each action explicitly; it can do only the
   intersection of its own actions and the user's current grants. Leave
   `observations.annotate` out if you will not use write-back. Expiry is at
   most 366 days, so plan a rotation. The `secret` is shown once.

An observation on a camera the token cannot view is answered exactly like an
unknown ID (`404 observation_not_found`). A token with `live.view` but not
`snapshot.create` gets `403` on `/snapshot`
([contract §12.1](OBSERVATIONS_CONTRACT.md#121-mapping-confirms-plan-decision-7)).

## Step 5: configure YA-WAMF

Set these in YA-WAMF's environment, or in **Settings → Connection** (the
source card), which writes the same settings. They are documented in YA-WAMF's
`docs/setup/environment-variables.md`.

| Variable | Value |
| --- | --- |
| `LIGHTNVR__ENABLED` | `true`. Frigate stays the default until this is set. |
| `LIGHTNVR__API_URL` | lightNVR base URL, for example `http://lightnvr:8080` (`http` or `https`, no credentials). |
| `LIGHTNVR__TOKEN` | the scoped token secret from step 4. |
| `LIGHTNVR__MQTT_TOPIC_PREFIX` | the lightNVR `[mqtt] topic_prefix` (default `lightnvr`). |
| `LIGHTNVR__CAMERA_UUIDS` | comma-separated feeder camera UUIDs; empty means every camera the token may view. |
| `LIGHTNVR__SOURCE_INSTANCE` | optional: a stable name (letters, digits and `-`, at most 64) used instead of the lightNVR installation UUID, so history stays attached across a lightNVR reinstall. |
| `LIGHTNVR__REQUEST_TIMEOUT_SECONDS` | optional, 1–60 (default 10). |
| `LIGHTNVR__WRITEBACK_ENABLED` | optional, default `false`; see [P2](#optional-send-species-back-to-lightnvr-p2). |

- YA-WAMF reaches lightNVR's events through its existing broker settings
  (`FRIGATE__MQTT_SERVER`, `FRIGATE__MQTT_PORT`, `FRIGATE__MQTT_AUTH`,
  `FRIGATE__MQTT_USERNAME`, `FRIGATE__MQTT_PASSWORD`). Point them at the same
  broker lightNVR publishes to.
- `LIGHTNVR__CAMERA_UUIDS` needs YA-WAMF `b89846d0` or later. Older builds of
  the branch do not start when the variable holds a UUID.
- **Restart YA-WAMF** after changing the source or any lightNVR setting. The
  running provider is not rebuilt in place, and the settings page says when a
  restart is pending. A failed test or save never switches providers.
- YA-WAMF applies its source-identity migration (Alembic `d4e1a7c93b52`) on
  the first start of this build. Back up `/data` first; YA-WAMF's
  `MIGRATION.md` ("Source identity for each detection") covers what changes
  and how to roll back.
- **First start is live-only.** With no stored cursor, YA-WAMF starts at
  `cursor=head` and does not import retained lightNVR history. To import a
  range, use **Settings → Data → Backup & maintenance tools → Scan History**.
  Items imported this way do not send the normal notification.

## Step 6: verify

1. In YA-WAMF, **Settings → Connection → Test lightNVR connection** reports
   four results:
   - API access: the token is accepted, history is authorized, and each chosen
     camera answers the setup probe;
   - event connectivity: the broker answers and the topic prefixes agree;
   - evidence access: snapshots are authorized and every chosen camera keeps
     evidence;
   - features: video clips and recordings are listed as unsupported. That is
     expected and does not affect health.

   It also shows the latest eligible detection and its photo state. "No bird
   detections yet" means lightNVR answered but has reported no bird in the last
   30 days; it is not an error.
2. The same setup probe by hand, with the YA-WAMF token:

   ```bash
   curl -H "Authorization: Bearer $TOKEN" \
     "http://lightnvr:8080/api/integrations/capabilities"
   curl -H "Authorization: Bearer $TOKEN" \
     "http://lightnvr:8080/api/observations?camera_uuid=<feeder camera_uuid>&limit=1"
   ```

   - `200` with an empty `items` list: authorized, no eligible detections yet.
   - `403`: the token lacks `live.view` on that camera. It is never reported
     as an empty list.
   - In the capabilities document, check `principal.writer_namespace`,
     `operations`, `delivery.mqtt_enabled` and `delivery.topic`, and that each
     feeder camera has `evidence_enabled: true`
     ([contract §9](OBSERVATIONS_CONTRACT.md#9-capability-document-get-apiintegrationscapabilities)).
3. Watch the topic while a bird visits:

   ```bash
   mosquitto_sub -h mqtt -t 'lightnvr/v1/events/io.lightnvr.detection.observation_changed.v1/#' -v
   ```

   A new observation usually publishes `created` with evidence `pending`, then
   `evidence` with `ready`.

## Optional: send species back to lightNVR (P2)

1. Make sure the token's user holds `observations.annotate` on the feeder
   cameras and the token lists it (step 4). Capabilities should show
   `operations["observations.annotate"].authorized: true`.
2. Set `LIGHTNVR__WRITEBACK_ENABLED=true` (or **Send species to lightNVR** in
   **Settings → Connection**) and restart YA-WAMF.

YA-WAMF then writes `<writer_namespace>/species` with `kind: automated` for a
model result and `kind: owner_confirmed` for an owner correction. An owner
decision is never replaced by an automated result. While lightNVR is
unreachable, results wait as "sync pending". Turning the switch on does not
send detections saved while it was off. lightNVR shows the newest owner
decision, otherwise the newest automated result, beside the detection on the
timeline and in the recording player; the detector's own label and confidence
stay visible ([contract §10](OBSERVATIONS_CONTRACT.md#10-classification-annotations-p2)).

Without `observations.annotate`, results stay "sync pending" and are given up
after about two days of retries.

X3 ran this end to end (A12, A13 and A7-annotate; see the
[matrix](#supported-version-matrix)). Known gap: only a new detection and an
owner correction (**Edit species**, single or bulk) are sent. A species
changed later by **Reclassify** or by video classification stays local until
the owner corrects or confirms it.

## Retention and budgets

([contract §14](OBSERVATIONS_CONTRACT.md#14-retention-and-storage))

- **Evidence images** expire per camera, by `observation_max_age_days` or
  `observation_max_bytes`, whichever comes first, oldest first. An expired
  observation stays readable, and `/snapshot` answers
  `410 evidence_expired`. Evidence lives under `<storage_path>/observations/`
  and is independent of recording retention in both directions.
- **History** (observation metadata and the change log) is kept for 30 days
  (`system_settings.observation_history_retention_days`). This release has no
  API or UI setting for it.
- **Cursor expiry.** A cursor older than retained history, or one from before
  a database restore, gets `410 cursor_expired` with `oldest_cursor`
  ([contract §7.5](OBSERVATIONS_CONTRACT.md#75-expiry-rules-410-cursor_expired)).
  YA-WAMF then marks the source `recovery_required`, resumes from
  `oldest_cursor` and shows the gap on its Health page until you acknowledge
  it.
- When a camera's byte budget is used up and only protected images remain, new
  evidence is rejected as `budget_exhausted` and flagged in diagnostics.
- The budget defaults are provisional until the X2 saturation gate measures
  them.

## Diagnostics

| Where | What | Who can read it |
| --- | --- | --- |
| YA-WAMF connection test and Health page | API, broker, evidence and feature checks; latest detection and photo state; recovery state; write-back counts | YA-WAMF owner |
| YA-WAMF `GET /api/diagnostics/source`, `/health` | source status, skipped workflows, `source_recovery`, `notification_intents`, `classification_writeback` | YA-WAMF owner |
| lightNVR `GET /api/observations?camera_uuid=X&limit=1` | the setup probe above | the YA-WAMF token |
| lightNVR `GET /api/metrics` | `lightnvr_observation_*` Prometheus families: accepted, dropped by reason, backlog and high-water marks, evidence failures, retained bytes and budgets per camera, oldest and head cursor seq, publications by state, relay lag | `system.admin` |
| lightNVR `GET /api/system/info` → `observation_pipeline`, and the **Observation pipeline** card on the System page | the same snapshot as JSON | `system.admin` |

The metric names are listed in
[contract §15](OBSERVATIONS_CONTRACT.md#15-diagnostics), and the JSON shape is
`ObservationPipelineDiagnostics` in [`openapi.json`](../api/openapi.json).
YA-WAMF's scoped token cannot read the last two rows; use an administrator
login. `active: false` means no camera has produced evidence in this process
run and nothing is retained. Per-camera rows come from the last retention pass,
so they can be up to five minutes old.

What to look for:

- `dropped{reason="queue_full"}` or `budget_exhausted` rising: the camera
  produces more evidence than the budgets allow.
- `publications{state="pending"}` and `relay_lag_seconds` rising: the outbox
  is not draining, usually because the broker is down. The cursor API still
  serves these changes.
- Many `relay_events_total{outcome="no_route"}`: routes exist but none
  matches the observation type (see [step 3](#step-3-mqtt-and-event-routes)).

## Delivery guarantees

- **MQTT delivery is at least once.** A change can be delivered again, after a
  retry, a reconnect or a restart, with the same envelope `id`. YA-WAMF
  deduplicates on `source` + `id` and admits each observation at most once
  ([contract §5.5](OBSERVATIONS_CONTRACT.md#55-relay-delivery),
  [§16.2](OBSERVATIONS_CONTRACT.md#162-admission)).
- Undelivered changes expire from the outbox after one hour. The history
  cursor is the durable recovery path.
- **YA-WAMF notifications are at most once.** A notification whose outcome is
  unclear (a timeout after sending, or a crash during sending) is recorded as
  `unknown` and shown in diagnostics, never resent. After a hard stop, some
  notifications can show `unknown`; that is expected.

## Rollback

- **Stop the integration** without losing data:
  - in YA-WAMF, set `LIGHTNVR__ENABLED=false` (Frigate becomes the source
    again) and restart;
  - in lightNVR, set `observation_evidence_enabled: false` on each camera.
    No new evidence is collected. Existing observations, evidence images and
    annotations are kept and expire under the normal retention rules.
  - Revoke the YA-WAMF token (`DELETE /api/authorization/users/{user_id}/tokens/{token_uuid}`).
- **Database rollback drops data.** The `-- migrate:down` sections of
  `0096_add_observation_classifications.sql` and
  `0095_add_object_observations.sql` drop every annotation, observation and
  evidence row; evidence files stay on disk. Take a lightNVR backup
  (`POST /api/system/backup`, [System backup](../API.md#system-backup)) before
  upgrading, and restore it rather than running the down sections when you
  need to go back. Running an older lightNVR binary against a database with
  0095/0096 applied has not been tested.
- **YA-WAMF.** Follow its `MIGRATION.md` for migration `d4e1a7c93b52`. In
  short: restore the backup taken before the update. An older YA-WAMF image is
  not supported once a non-Frigate source has been used.

## Supported-version matrix

This publishes [contract §18](OBSERVATIONS_CONTRACT.md#18-supported-version-matrix-stub).
Neither branch is released or pushed.

| Component | Version | Contract |
| --- | --- | --- |
| lightNVR | branch `feat/int01-event-evidence`; X1 ran a binary built from `e294f956` and X3 one built from `bcf5290f` (version string 0.47.3; the last behaviour change is `ddea7404`, later commits are docs, tests and plan updates) | `observations` 1, `classifications` 1, event `io.lightnvr.detection.observation_changed.v1` |
| YA-WAMF | branch `feature/lightnvr-provider` at `1cbbae74`, on v2.23.1 (`53540f48`), for both X1 and X3. It needs `b89846d0` or later to start with `LIGHTNVR__CAMERA_UUIDS` set, and `de28e1e4` or later for write-back (P2) | `observations` 1, `classifications` 1 |
| MQTT broker | Mosquitto 2.0.22 (X1) | n/a |
| Frigate (YA-WAMF source) | as supported by YA-WAMF v2.23.1 | n/a, unchanged |

What has been tested, and how:

| Area | Status | Covered by |
| --- | --- | --- |
| lightNVR observation, evidence, relay, retention, API and annotation units (C, including ASan/UBSan builds) | tested | `tests/unit/test_db_observations`, `test_observation_evidence`, `test_observation_relay`, `test_observation_retention`, `test_observation_crash_boundaries`, `test_api_handlers_observations`, `test_db_observation_classifications`, and others listed in the plan |
| Local detectors (SOD, LiteRT) producing evidence | tested (unit) | `test_detection_evidence`, `test_detection_observation_evidence` |
| HTTP API detector (light-object-detect and DOODS2 formats), ONVIF unavailable evidence | tested (unit, loopback detector) | `test_api_detection_observation_evidence` |
| lightNVR binary end to end: HTTP detector → observation → MQTT, history, crash and restart, restore, per-camera isolation, expiry outcomes (A1, A2, A5–A8) | tested (Playwright contract suite, in-process MQTT sink) | `tests/contracts/observations*.api.spec.ts` |
| Annotation PUT and display (A12, A7-annotate, L15) | tested (Playwright contract and UI) | `tests/contracts/observations.api.spec.ts`, `tests/integration/specs/detection-classification.ui.spec.ts` |
| YA-WAMF ingestion, recovery, notification intent, setup, write-back (A1, A3, A4, A9, A10) | tested (pytest against a fake lightNVR and fake broker driven by the shared fixtures; Playwright UI) | `backend/tests/test_lightnvr_acceptance.py` and the W5–W10 test modules in YA-WAMF |
| Frigate regression after the YA-WAMF migration | tested | `backend/tests/test_frigate_regression_baseline.py` in YA-WAMF |
| Both applications together through a real broker, this recipe run step by step (X1: A1, A2, A3, A4, A5, A6, A8) | tested, in host-process mode: real Mosquitto, the lightNVR binary and the YA-WAMF backend as host processes. The container mode in the same directory is documented but has not been run | `tests/integration/yawamf_e2e/run_e2e.py`; results in `tests/integration/yawamf_e2e/results/x1-host-20261108.json` |
| MQTT broker | tested: Mosquitto 2.0.22 (local-only mode, QoS 1, no persistence), including a broker outage | X1 |
| HTTP bird detector | tested with a pinned stand-in: the loopback `x1-colour-detector 1.0` in the light-object-detect wire format, which finds synthetic red and blue birds by colour. A real HTTP bird model (light-object-detect or DOODS2) is **pending** | X1 |
| Local bird detector | tested once: SOD VOC `tiny20.sod` (classes include `bird`) on a real bird photo crossing the frame (X1 `A2-local`; optional, because the photo is not committed). LiteRT/TFLite end to end is **pending** | X1 |
| YA-WAMF classifier | tested with a pinned stand-in: `x1-colour-rule-v1`, which replaces `classify_async_live` only and names a crop by its saturated red or blue pixels. YA-WAMF's real bird model is **pending** | X1 |
| P2 end to end (A12, A13, A7-annotate) with `LIGHTNVR__WRITEBACK_ENABLED=true` | tested, in the same host-process mode (X3). The species shows in YA-WAMF, in lightNVR's annotation, `GET /api/observations/{id}` and the detection results. Replays and stale revisions keep one annotation. An owner correction arrives as `owner_confirmed`, and a later automated result is refused. The classification change causes no classifier call or notification-intent change. A 47 s lightNVR outage leaves the YA-WAMF detection intact, and the write-back completes 14 s after lightNVR returns. A token without `observations.annotate` gets `403`. The X1 scenarios pass in the same run with write-back on | `tests/integration/yawamf_e2e/run_e2e.py` (`p2_scenarios.py`); results in `tests/integration/yawamf_e2e/results/x3-host-20261108.json`, RED run without write-back in `x3-host-20261108-red-nowriteback.json` |
| Saturation and performance gate (A11), final budget defaults | **pending** | X2 |
| Real-camera operator pilot (A14) | **pending**; checklist ready | [`A14_PILOT_CHECKLIST.md`](../../tests/integration/yawamf_e2e/A14_PILOT_CHECKLIST.md) |
