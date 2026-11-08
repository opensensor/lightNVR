# INT_01 X1/X3: lightNVR + YA-WAMF end-to-end suite

This suite runs the deployment recipe
[`docs/integrations/YA_WAMF.md`](../../../docs/integrations/YA_WAMF.md)
against real processes, then checks PRD INT_01 acceptance A1–A6 and A8 (X1,
P1) and A12, A13 and A7-annotate (X3, P2 classification write-back) across
both applications. A14, the operator pilot on a real camera, is a manual
checklist: [`A14_PILOT_CHECKLIST.md`](A14_PILOT_CHECKLIST.md).

## What runs

| Process | What it is |
| --- | --- |
| Mosquitto 2.x | A real broker on a loopback port. It is stopped and restarted for A5. |
| lightNVR | The binary under test (`--lightnvr-bin`), with the recipe's `[mqtt]` block. |
| YA-WAMF backend | The branch under test, unmodified, started by `harness/yawamf_launcher.py` with a pinned classifier (below). |
| Feeder source | Three 640×360, 10 fps H.264 MPEG-TS scenes rendered by ffmpeg and served over HTTP: one static red bird, one red bird crossing the frame at 240 px/s, and a red and a blue bird together. |
| Detector | A loopback HTTP detector in the light-object-detect wire format. It finds the red and blue fixture birds in the submitted JPEG by colour. Each camera reports birds only for the number of frames the scenario arms, so observation counts are exact. |

The orchestrator (`run_e2e.py`) runs recipe steps 1–6 verbatim through the
public APIs: streams, evidence settings, the `mqtt:default` route, user → role →
policy → scoped token, the YA-WAMF environment, the connection test and the
token probes. It records each step's outcome in the results JSON (`recipe`).

### Pinned models

- **Detector:** `x1-colour-detector 1.0`, the loopback detector described
  above. No bird-capable neural model runs in this suite.
- **YA-WAMF classifier:** `x1-colour-rule-v1`. The launcher replaces
  `ClassifierService.classify_async_live` only. A crop in which at least 10%
  of pixels are saturated red is *Prunella modularis* 0.93; otherwise, at least
  10% saturated blue is *Erithacus rubecula* 0.93. Anything else (a crop that
  missed the bird) gets no result. Every
  call is logged with the crop's size and mean colour, so the suite can count
  calls and confirm each crop really contains its bird.

### A2 with a local detector (optional `A2-local`)

`--sod-model` and `--bird-image` add a seventh camera. It runs a real local
SOD model on a real bird photo that crosses a grey frame at 120 px/s. The
photo is not committed, because its licence (CC BY-SA 4.0) does not fit this
repository. X1 used:

- model: VOC `tiny20.sod` (SHA-256 prefix recorded in the results), whose
  classes include `bird`;
- photo: Wikimedia Commons
  `File:Northern_cardinal_male_(84894).jpg`, as the 330 px thumbnail
  (`sha256 85bf66b1c51ad1aeb08d3d90f1a39a59f6cb6161d91f7e7b51d4d187847f3cdf`).

There is no submitted JPEG to compare against, because the stored image is
the decoded inference frame. Instead, the suite finds the red bird in each
stored image and requires the observation box to contain the bird's centre
with IoU ≥ 0.2. The bird must also have moved at least 10% of the frame width
between the stored frames, and the evidence origin must be
`inference_input`.

### Not covered here

- YA-WAMF's real classifier model. The colour rule above stands in for it.
- LiteRT/TFLite (this build has `ENABLE_LITERT=OFF`), DOODS2, and ONVIF
  evidence end to end. They are covered by unit tests
  (`test_detection_evidence`, `test_detection_observation_evidence`,
  `test_api_detection_observation_evidence`).

Both remain pending in the
[supported-version matrix](../../../docs/integrations/YA_WAMF.md#supported-version-matrix).

## Scenarios

| ID | What the suite does | What must hold |
| --- | --- | --- |
| A1 | One frame with one bird | One observation (`created`/`pending`, then `evidence`/`ready` on the real broker with deterministic message IDs). One YA-WAMF detection keyed `lnvr:<installation>:<observation>`, with the camera UUID, label and species. One classifier call. |
| A2 / A2-local | Three frames of the moving bird (HTTP detector); optionally a real SOD model on a real photo (above) | Each stored snapshot is byte-equal to one image the detector received, and its box is that detector box. The box covers the bird in its own image and misses it in the other frames (the bird moved at least 64 px). YA-WAMF identifies every crop as the red bird. |
| A3 | One frame with two birds | Two sibling observations share one batch and one asset. Each box covers its own bird. YA-WAMF creates two detections with the right species each, from two classifier calls. |
| A4 | Republish every captured A1/A3 message 10× to the broker while `POST /api/backfill` runs recovery | No new detection, no new classifier call, and the notification intent is unchanged. The backfill reports `new_detections: 0`. |
| A5 | Broker stopped; 100 observations (50 two-bird frames); both apps stopped; broker, lightNVR and YA-WAMF started again | All 100 observations appear exactly once in YA-WAMF. The outbox also delivers all 200 revisions once the broker returns. |
| A6 | SIGKILL lightNVR mid-burst, then restart. Separately, SIGKILL YA-WAMF mid-burst, then restart | lightNVR: startup order (reconcile → evidence → relay drain → relay), no pending asset, no `.jpg.tmp`, every ready row has its file. YA-WAMF converges to exactly the ready observations, with no duplicates and nothing admitted for evidence that is not ready. |
| A8 | Temporary: lightNVR frozen (SIGSTOP) while YA-WAMF starts with missed history, then resumed. Expired: observations backdated 8 days while lightNVR is stopped, so the first retention pass expires them. Unavailable: ready files deleted while stopped, so startup reconciliation marks them `unavailable(write_failed)`. Expired cursor: file-level DB restore, so lightNVR regenerates the epoch | Nothing is admitted while lightNVR is unreachable, and everything is admitted after it resumes. Snapshots answer `410 evidence_expired` and `404 evidence_unavailable`. YA-WAMF admits none of those and reports the reasons (`evidence_expired`, `evidence_unavailable`) in recovery and backfill. The old cursor answers `410 cursor_expired`, and YA-WAMF shows `recovery_required` until it is acknowledged. |
| A12 | One bird on `feeder-wb`; YA-WAMF runs with `LIGHTNVR__WRITEBACK_ENABLED=true` (the recipe's optional P2 step). Then: every captured message for it republished 10×, 5 identical annotation PUTs, a stale (`expected_revision: 0`) and a future revision; an owner correction (`PATCH /api/events/{id}`); an automated PUT over the owner decision; YA-WAMF's detection reverted to an automated result with its write-back job re-queued | The species is on YA-WAMF's row, in lightNVR's `observation_classifications` (`yawamf/species`, `automated`), in `GET /api/observations/{id}` and on `GET /api/detection/results/{stream}` (`observation.classification`), while the detector's `bird` label and confidence are unchanged. The classification change is published once (revision 3, deterministic ID) and causes no classifier call, no new row, no intent change and no new write-back job. Replays and retries are `200 changed:false`, stale and future revisions `409 revision_conflict`; one row per key stays at annotation revision 1. The correction arrives as `owner_confirmed` (revision 2, confidence 1). A later automated result is refused (`409 owner_decision_locked`) and YA-WAMF's job ends terminal with `dropped:owner_decision_locked`. |
| A13 | One bird on `feeder-wb-outage`, written back; lightNVR stopped; owner correction in YA-WAMF while it is down; lightNVR started again | The correction is saved and served by YA-WAMF while lightNVR is down, and the write-back job is `retryable` with `retry:source_unreachable` ("sync pending"). After lightNVR returns, the job succeeds and lightNVR shows `owner_confirmed` with the corrected species. No classifier call, duplicate row or notification-intent change. |
| A7-annotate | Two more tokens for the same user: one without `observations.annotate`, one with it but scoped to another camera | Without the action: `403 forbidden` on an existing and on a new key. Another camera's scope: `404 observation_not_found`. Neither writes a row or bumps the revision. The recipe token creates `yawamf/x3probe` (`201`) and its retry is `200 changed:false`; `yawamf/species` is untouched. |

## Running it

### Host-process mode (what X1 executed)

Requirements: a lightNVR build with MQTT, `mosquitto` and `mosquitto_pub`/`mosquitto_sub` 2.x,
ffmpeg with libx264, and a YA-WAMF checkout whose `backend/.venv` was
created from its requirements (it provides Pillow and numpy).

```bash
../YetAnother-WhosAtMyFeeder/backend/.venv/bin/python tests/integration/yawamf_e2e/run_e2e.py \
  --lightnvr-bin build/bin/lightnvr --yawamf ../YetAnother-WhosAtMyFeeder \
  [--scenarios A1,A3] [--workdir /path/to/keep] [--run-name NAME] [--no-writeback] \
  [--sod-model /path/to/tiny20.sod --bird-image /path/to/cardinal.jpg]
```

- A full run takes about 5 minutes. The exit status is 0 only when every
  selected scenario passed.
- A4 replays the messages A1 and A3 captured, so run A1 and A3 with it.
- YA-WAMF starts with `LIGHTNVR__WRITEBACK_ENABLED=true`, so the P1
  scenarios also run with write-back on. `--no-writeback` starts it without
  (the P1 recipe only); A12, A13 and A7-annotate then fail, which is how X3
  recorded its RED run.
- A12's automated-after-owner step waits for YA-WAMF's write-back worker,
  which polls every 60 s. A13 waits for the first retry, 30 s after the
  failed attempt.
- `results/<run>.json` holds versions, recipe-step outcomes and per-scenario
  evidence. Next to it are the MQTT capture, the detector log and the
  classifier log.
- Mosquitto runs without a config file (`mosquitto -p <port>`, the 2.x
  local-only mode). On Ubuntu, AppArmor lets `/usr/sbin/mosquitto` read
  configs only under `/etc/mosquitto`.

### Container mode (documented CI/nightly path; not executed in X1)

```bash
YAWAMF_SRC=/path/to/YetAnother-WhosAtMyFeeder \
  docker compose -f tests/integration/yawamf_e2e/compose/docker-compose.yml run --rm e2e
```

This builds one image with every process the suite owns: lightNVR from this
checkout, YA-WAMF's backend with its pinned requirements, Mosquitto and ffmpeg.
It then runs the same `run_e2e.py`. Everything is in one container because the
suite SIGKILLs, SIGSTOPs and restarts processes and reads both SQLite
databases. X1 committed host-mode results only; record the first container
run in the matrix.

## Gotchas found while running the recipe

- Stream names may contain only letters, digits, `.`, `-` and `_`. The
  recipe's earlier `Feeder East` example was rejected.
- `PUT /api/streams/{name}` answers `202`, and the change applies
  asynchronously.
- `POST /api/authorization/roles` and `PUT /api/authorization/users/{id}`
  need `expected_policy_version` from a prior GET.
- YA-WAMF's connection test (`POST /api/setup/lightnvr/test`) needs `api_url`
  in the body. A missing `token` keeps the saved token.
- In the stream API, `detection_threshold` is a percentage (`50`, not
  `0.5`). With `0.5`, every low-confidence SOD box became an observation.
- YA-WAMF did not start when `LIGHTNVR__CAMERA_UUIDS` held a UUID
  (pydantic-settings JSON-decoded the list). Fixed in YA-WAMF `b89846d0`.
- After recovery skipped an observation for an expired or unavailable photo,
  Scan History counted it as "already existed". Fixed in YA-WAMF `fcf449f2`.
- A live message that arrived while lightNVR was down (SIGKILL in A6) was
  retried only by the next 5-minute recovery interval. It took 297 s in
  `results/x1-host-20261108-prefix-a6.json`. Since YA-WAMF `37c0877f`, a live
  retry requests recovery straight away. Since `1cbbae74`, a burst of live
  retries no longer multiplies the backoff: one run took 121 s instead of
  12 s before that fix.
- On a first start, YA-WAMF begins at `cursor=head`. Start it before creating
  observations, or seed `source_cursors`.
- YA-WAMF ignores live messages until its capabilities fetch succeeds. When
  it starts with lightNVR unreachable, recovery catches up later.
- MQTT reconnect backoff reaches about 35 s on both sides after a broker
  outage or a SIGKILL. Keep timeouts generous.
- (X3) An owner correction is `PATCH /api/events/{event}` with
  `{"display_name": ...}`. YA-WAMF has no `GET /api/events/{event}`; use
  `GET /api/events?event_id=`.
- (X3) A token's camera selector must equal one of its user's grants, not a
  subset of it (`403 API token scope exceeds live.view authority`). The
  other-camera A7 probe is therefore a second user, `yawamf-east`, holding the
  same role on one camera.
- (X3) YA-WAMF writes back only after a new detection and after an owner
  correction. **Reclassify** and video classification change the species
  without queuing a write-back.
- Wait for `recordings/observations/.cursor_head.json` to reach the head
  before taking a file-level backup (it is written at most once a second).
