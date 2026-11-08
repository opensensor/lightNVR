# A14 operator pilot: lightNVR + YA-WAMF on a real feeder camera

PRD INT_01 A14: "Operator installs from the documented recipe using scoped
credentials, sees a real feeder visit, exercises reconnect recovery, and
records the tested source/model versions."

This is a manual checklist. Follow
[`docs/integrations/YA_WAMF.md`](../../../docs/integrations/YA_WAMF.md)
step by step, without workarounds. Wherever a step is wrong or missing, write
what you did instead under **Notes**, and fix the recipe in the same change.
When everything is done, tick the boxes, fill in the tables and commit this
file. Then move the matrix row "Real-camera operator pilot (A14)" from
**pending** to **tested**.

**Status: not started.** X1 ran the recipe on synthetic fixtures only (see
`README.md` and `results/`). Nobody has run this pilot yet.

## 0. Record what you are testing

| Item | Value |
| --- | --- |
| Date, operator | |
| lightNVR commit / version string (`GET /api/system/info` → `version`) | |
| lightNVR build options (MQTT, SOD, LiteRT) | |
| YA-WAMF commit (`/health` → `version`) and image flavour | |
| MQTT broker and version | |
| Camera make/model, main and detection stream resolution | |
| Detector engine (SOD / LiteRT / HTTP light-object-detect / DOODS2) | |
| Detector model file name, version and SHA-256 | |
| Detector labels that include `bird` | |
| YA-WAMF classifier model id and version (Settings → Detection) | |
| Host OS, CPU or accelerator | |

## 1. Install from the recipe

- [ ] Prerequisites: lightNVR built with MQTT, a broker both apps reach, YA-WAMF from `feature/lightnvr-provider`, an administrator login.
- [ ] Back up lightNVR (`POST /api/system/backup`) and YA-WAMF `/data` before the first start of the new builds.
- [ ] Step 1: the feeder stream uses a bird-capable detector (`detection_based_recording: true`, `detection_model`). The model's labels include `bird`.
- [ ] Step 2: evidence is on for the feeder camera only (`observation_evidence_enabled: true`, `observation_labels: "bird"`). Recorded `camera_uuid`: ____________
- [ ] Step 3: the `[mqtt]` block is set and lightNVR restarted. If any event route exists, the observation route was added with **no** predicate.
- [ ] Step 4: user `yawamf` → role (`live.view`, `snapshot.create`, optional `observations.annotate`) → feeder-only policy → scoped token. The token expiry is recorded: ____________
- [ ] Step 5: YA-WAMF environment set (`LIGHTNVR__*`, `FRIGATE__MQTT_*` pointing at the same broker), then restarted.
- [ ] Step 6.1: **Test lightNVR connection** shows API access, event connectivity and evidence access as ok, and features as unsupported-video only.
- [ ] Step 6.2: the token probe returns `200`; a camera outside the token scope returns `403`.
- [ ] Step 6.3: `mosquitto_sub` shows `created`/`pending`, then `evidence`/`ready`, for a visit.

## 2. A real feeder visit

- [ ] A bird visits. lightNVR shows the detection, and `GET /api/observations?camera_uuid=…` lists it with evidence `ready`.
- [ ] YA-WAMF shows the detection with the right camera, a species, and a photo cropped around the bird.
- [ ] The photo in YA-WAMF is the low-resolution detection image (not a later recording frame), and the crop matches where the bird was.
- [ ] With two birds in frame at once (if it happens): two YA-WAMF detections, one per bird.

| Observation id | Detector label / confidence | YA-WAMF species / score | Crop correct? |
| --- | --- | --- | --- |
| | | | |

## 3. Reconnect recovery

- [ ] Stop the broker for at least one visit, then start it again. YA-WAMF catches up (from the cursor, the outbox, or both) and shows the missed visit once.
- [ ] Restart YA-WAMF while lightNVR keeps detecting. Visits made while it was down appear once after it starts.
- [ ] Restart lightNVR during a visit. YA-WAMF converges, and lightNVR's `observation_pipeline` (System page) shows no stuck pending items.
- [ ] YA-WAMF Health shows the source and recovery state, with no unexpected gap.

| Interruption | Visits during the outage | Visits in YA-WAMF afterwards | Time to converge |
| --- | --- | --- | --- |
| broker stopped | | | |
| YA-WAMF restarted | | | |
| lightNVR restarted | | | |

## 4. Outcomes and diagnostics

- [ ] lightNVR `GET /api/metrics` (as admin): `lightnvr_observation_*` present, with no unexplained `dropped` reasons.
- [ ] YA-WAMF `/health` → `source_recovery` and `notification_intents` look sane.
- [ ] Optional P2: write-back on; lightNVR shows the YA-WAMF species next to the detection.

## 5. Sign-off

- [ ] Versions recorded in section 0 and copied to the supported-version matrix in `docs/integrations/YA_WAMF.md`.
- [ ] Recipe corrections (if any) committed.

**Notes** (anything that differed from the recipe):

