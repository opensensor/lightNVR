# Pre-Detection Buffer Implementation

## Status: IMPLEMENTED

The pre-detection buffer is now implemented in the **Unified Detection Thread** (`src/video/unified_detection_thread.c`).

## Current Implementation

### What Works
- **Pre-detection buffering**: Configurable `pre_detection_buffer` seconds of video are captured before detection events
- **Post-detection buffering**: Configurable `post_detection_buffer` seconds continue recording after last detection
- **Detection-based recordings**: Properly marked in the database with `trigger_type='detection'`
- **Annotation mode**: When both `record=true` and `detection_based_recording=true`, detections are stored with `recording_id` linking to continuous recordings (no separate MP4s created)
- **HLS keepalive**: HLS registers as a go2rtc consumer to keep streams active for reliable detection snapshots

### Key Implementation Details
- The unified detection thread reads packets directly and maintains an AVPacket ring buffer
- Buffer duration is controlled by `pre_detection_buffer` config field (default: 5 seconds)
- When detection occurs, buffered packets are flushed to the MP4 writer
- Post-detection buffer (`post_detection_buffer`) controls how long to continue after last detection

## Architecture

### Unified Detection Thread

The unified detection thread (`src/video/unified_detection_thread.c`) implements a per-stream detection system with:

- **Circular AVPacket buffer** for pre-detection video capture
- **State machine**: INITIALIZING → CONNECTING → BUFFERING → RECORDING → POST_BUFFER → ...
- **HLS keepalive**: Registers as go2rtc consumer to ensure reliable detection snapshots
- **Annotation mode**: Links detections to continuous recordings without creating separate MP4s

### Key Files

- `src/video/unified_detection_thread.c` - Main implementation
- `include/video/unified_detection_thread.h` - API and context structure
- `src/video/go2rtc/go2rtc_consumer.c` - HLS consumer registration
- `src/video/stream_manager.c` - Recording mode logic
- `src/database/db_detections.c` - Detection storage with recording_id

The unified detection thread integrates with the recording mode system:

| `record` | `detection_based_recording` | Detection Thread Behavior |
|----------|----------------------------|---------------------------|
| `false` | `false` | Not started |
| `true` | `false` | Not started (continuous recording only) |
| `false` | `true` | Creates MP4s on detection (`annotation_only=false`) |
| `true` | `true` | Links to continuous recording (`annotation_only=true`) |

## Configuration

### Stream Config Fields
```c
typedef struct {
    // ... existing fields ...
    int pre_detection_buffer;   // Seconds to buffer before detection (default: 5)
    int post_detection_buffer;  // Seconds to record after last detection (default: 3)
    // ... existing fields ...
} stream_config_t;
```

### Recommended Defaults
- `pre_detection_buffer`: 5-10 seconds
- `post_detection_buffer`: 3-10 seconds
- Maximum: 30 seconds (to limit resource usage)

## Related Documentation

- [ARCHITECTURE.md](ARCHITECTURE.md) - Overall system architecture and recording modes
- [SOD_INTEGRATION.md](SOD_INTEGRATION.md) - SOD object detection integration
- [ONVIF_MOTION_RECORDING.md](ONVIF_MOTION_RECORDING.md) - ONVIF motion detection
