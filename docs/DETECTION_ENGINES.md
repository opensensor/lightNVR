# Multiple Detection Engines

LightNVR supports an ordered set of detection engines per stream. The initial
runtime composition is `any_of`: motion, local object, ONVIF, and external
triggers can coexist, and any positive result refreshes the stream's existing
detection-recording state. A decoded video frame is converted to RGB once and
shared by all due local frame engines; each engine retains its own threshold
and interval.

Migration `0077` projects every existing non-empty `streams.detection_model`
into a read-only `legacy-primary` engine. Existing stream create/update APIs
continue to control that row. The new collection API replaces only custom
engines, so adding motion to an existing object-model stream does not overwrite
the legacy model.

## API

Both endpoints require camera-scoped `camera.configure`:

- `GET /api/streams/{stream_name}/detection-engines`
- `PUT /api/streams/{stream_name}/detection-engines`

The stream name must be URL encoded. `PUT` atomically replaces every custom
engine and preserves `legacy-primary`. An empty array removes the custom set.
The response reports `restart_required: true`; restart the stream or LightNVR
before expecting the new runtime set to take effect.

For an existing stream whose legacy primary is an object model, add motion:

```http
PUT /api/streams/North%20Drive/detection-engines
Content-Type: application/json

{
  "engines": [
    {
      "key": "motion-fast",
      "type": "motion",
      "model_path": "motion",
      "enabled": true,
      "threshold": 0.2,
      "interval_seconds": 1,
      "sort_order": -10,
      "config": {}
    }
  ]
}
```

Engine keys are stable caller-selected identifiers. Supported types are
`motion`, `object`, `onvif`, `api`, and `external`; there may be at most eight
rows including the compatibility engine. Relative object model paths resolve
under the configured models directory. `config` must be a bounded JSON object
and must not contain credentials.

For `api` engines, `model_path` is the detection endpoint URL and `config` may
carry request options that override the global `[api_detection]` settings:
`format` (`light-object-detect` or `doods2`), `backend` (light-object-detect)
and `detector_name` (doods2). See the wire formats in
[CONFIGURATION.md](CONFIGURATION.md#wire-formats).

```json
{
  "key": "doods",
  "type": "api",
  "model_path": "http://doods:8080/detect",
  "enabled": true,
  "threshold": 0.4,
  "interval_seconds": 2,
  "config": {"format": "doods2", "detector_name": "tensorflow"}
}
```

The override takes effect when this is the stream's only engine (leave the
stream's legacy detection model empty); see the runtime boundaries below.

## Runtime boundaries

- Motion and local object engines share decoded frames and run at their own
  cadence.
- ONVIF remains an asynchronous PullPoint source and may run beside frame
  engines.
- External API motion triggers remain available regardless of the custom set.
- The recording policy is currently fixed to `any_of`; per-engine Boolean
  policies and gating are not implemented.
- Additional HTTP `api` engines can be stored but are intentionally isolated
  when several engines are active because the legacy API detector persists
  results internally. A single legacy API detector works as before. Splitting
  inference from persistence is required before safe multi-API fan-out.
- Engine lifecycle still follows the stream's existing
  `detection_based_recording`/schedule lifecycle. This release does not create
  a separate always-on analytics-only coordinator or UI editor.

## Observation evidence (INT_01)

A camera can keep durable observations for integrations such as YA-WAMF: one
row per detected object, with its label, confidence and box, plus the image
the detector ran on. It is **off by default**. Turn it on per camera with
`observation_evidence_enabled`, and bound it with `observation_labels`,
`observation_max_age_days` and `observation_max_bytes` on the stream API
(`camera.configure`), or in the camera's **Observation evidence for
integrations** settings. Changes apply without a restart. Defaults and bounds
are in the
[observations contract](integrations/OBSERVATIONS_CONTRACT.md#13-budgets-and-configuration-provisional).

What each engine contributes:

| Engine | Observations | Evidence image |
| --- | --- | --- |
| `object` (SOD) | one per eligible object | the decoded detection frame; transform `resize` to the network size |
| `object` (TFLite on LiteRT) | one per eligible object | the decoded detection frame; transform `letterbox` |
| `api` (single HTTP detector) | one per eligible object that passes zones, object filters and the threshold | the exact JPEG submitted to the detector; transform `none`, or a server-declared letterbox |
| `onvif` | one per classified camera event (not plain `motion`) | none: `unavailable(no_matching_media)`, no box |
| `motion` | none | none |
| Additional `api` engines in a multi-engine set | none (they are isolated and do not run) | none |

- The image is the low-resolution inference input, not a recording frame.
  Boxes are normalized to it, and the `transform` records how it maps to the
  source and inference sizes
  ([contract §4.4](integrations/OBSERVATIONS_CONTRACT.md#44-transform)).
- In a multi-engine set, each local object engine submits its own batch, so
  each observation carries a single engine's provenance. Objects from one
  frame share one image.
- Detection threads never block on evidence. A frame is copied into a bounded
  queue and encoded by one worker thread; when the queue or a budget is full
  the observation is recorded as `unavailable` with a reason (`queue_full`,
  `budget_exhausted`) instead. If even the drop-record ring is full, the drop
  is only counted (`untracked` in the diagnostics).
- Detection rows and their MQTT events are unchanged. The recording timeline
  links a detection to the observation from the same camera, label and second
  (±1 s) to show its classification.

The deployment recipe is [YA-WAMF with lightNVR](integrations/YA_WAMF.md).
