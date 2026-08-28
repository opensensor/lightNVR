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
