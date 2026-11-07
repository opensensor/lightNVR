# Pre- and Post-Event Buffering

Detection-triggered and motion-triggered recordings include video from **before** the
event and keep recording for a while **after** it, so a clip shows what led up to the
event and how it ended.

## How it works

For each stream with detection-based recording (any engine: external API, SOD, LiteRT or
ONVIF events), one detection thread reads the camera and keeps the last few seconds of
compressed packets in an in-memory ring buffer. The thread moves through these states,
which appear in the LightNVR log (`/api/health` summarizes them per stream as
`detector_operating`):

| State | Meaning |
|---|---|
| `BUFFERING` | Connected and filling the ring buffer, waiting for a detection |
| `RECORDING` | A detection arrived: the buffer was written out, starting at a keyframe, and live packets follow |
| `POST_BUFFER` | No detection for the grace period; recording continues for the post-detection time |
| `RECONNECTING` | The camera dropped; the thread reconnects by itself |

A new detection during `POST_BUFFER` returns the stream to `RECORDING`, so one continuous
event produces one clip rather than several.

```
BUFFERING ──detection──▶ RECORDING ──grace period, no detections──▶ POST_BUFFER
    ▲                        ▲                                            │
    │                        └────────────── new detection ───────────────┤
    └───────────────────────── post-buffer time elapsed ──────────────────┘
```

Packets are stored compressed, never decoded, so memory use follows the camera's bitrate.
The buffer pool is sized automatically from each detection stream's resolution, frame
rate and pre-detection time.

## Settings

Per stream, in **Streams → Edit → AI Detection Settings**:

- **Pre-Detection Buffer (sec)**: video kept before the detection. The ring buffer holds
  between 5 and 30 seconds, so values above 30 behave as 30.
- **Post-Detection Buffer (sec)**: how long recording continues after the post-buffer
  starts.

Defaults for streams that don't set their own, and the grace period, are in
`lightnvr.ini`; see [Configuration](CONFIGURATION.md#api-detection-settings):

```ini
[api_detection]
pre_detection_buffer = 5
post_detection_buffer = 10

[detection]
grace_period = 2   ; seconds without detections before the post-buffer starts
```

### ONVIF events

Camera events reach LightNVR a few seconds after they happen (event polling and keyframe
timing). Streams with an ONVIF engine keep an extra 5 seconds in the buffer, and the clip
start is anchored to the event's own timestamp, not the time LightNVR acted on it. See
[ONVIF Detection](ONVIF_DETECTION.md).

## Troubleshooting

- **Clip starts later than the pre-buffer setting:** the clip has to start on a keyframe.
  A camera with a long keyframe interval (GOP) can lose up to one interval; set the
  camera's I-frame interval to about 1-2 seconds.
- **Events split into several clips:** raise `grace_period` or the post-detection buffer.
- **Memory pressure on small devices:** lower the pre-detection buffer, or use the
  camera's substream for detection streams.

Implementation history is in
[internal/PRE_DETECTION_BUFFER_IMPLEMENTATION.md](internal/PRE_DETECTION_BUFFER_IMPLEMENTATION.md).
