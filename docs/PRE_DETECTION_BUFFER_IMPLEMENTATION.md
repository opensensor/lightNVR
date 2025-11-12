# Pre-Detection Buffer Implementation Plan

## Problem Statement

Currently, when object detection triggers a recording, the recording starts **from the detection event forward**. This means you often miss the critical moments leading up to the detection event (e.g., a person walking into frame, a car approaching, etc.).

**User Request:** Implement a configurable ring buffer per stream that continuously captures video, so when a detection occurs, we can include the pre-detection footage (e.g., 5-10 seconds before the detection).

## Current State

### What Works
- Detection-based recordings are properly marked in the database with `trigger_type='detection'`
- Recordings start when detection occurs and stop when no more detections are found
- MP4 files are created with proper keyframe alignment
- The `pre_detection_buffer` field exists in `stream_config_t` but is not implemented

### What Doesn't Work
- No pre-detection buffering - recordings start from the detection event forward
- You miss the moments leading up to the detection

## Existing Infrastructure

### Motion Detection Has This Feature
The ONVIF motion detection system (`src/video/motion_buffer.c`) already has a sophisticated ring buffer implementation:

- **Circular buffer** that stores AVPackets in memory
- **Configurable duration** (5-30 seconds)
- **Thread-safe** operations with mutex protection
- **Memory-efficient** packet storage using AVPacket cloning
- **Automatic packet management** (oldest packets dropped when full)
- **Flush mechanism** to write buffered packets to recording when motion is detected

**Key Files:**
- `include/video/motion_buffer.h` - Buffer API
- `src/video/motion_buffer.c` - Buffer implementation (553 lines)
- `src/video/onvif_motion_recording.c` - Integration with motion detection

## Why It's Not Simple for Object Detection

The motion detection buffer works because:
1. It has a **dedicated thread** that continuously reads from the RTSP stream
2. It feeds packets into the ring buffer in real-time
3. When motion is detected, it flushes the buffer to the recording

For object detection, the architecture is different:
1. Detection works on **HLS segments** (not direct RTSP stream)
2. There's no continuous packet-level stream to buffer
3. The MP4 recording system creates its own RTSP connection when recording starts

## Implementation Options

### Option 1: Reuse Motion Buffer Infrastructure (Complex)
**Approach:** Create a dedicated buffering thread per stream that continuously reads from RTSP and feeds packets into a ring buffer.

**Pros:**
- Reuses existing, tested code
- True packet-level buffering
- Precise control over buffer duration

**Cons:**
- Requires **2 RTSP connections per stream** (one for buffering, one for recording)
- Significant memory overhead (storing AVPackets in RAM)
- Complex thread management
- Architectural changes to detection system

**Estimated Effort:** 2-3 days of development + testing

### Option 2: HLS Segment-Based Buffer (Simpler)
**Approach:** Keep a rolling window of the last N HLS segments, and when detection occurs, concatenate those segments into the beginning of the MP4 recording.

**Pros:**
- Works with existing HLS-based detection
- No additional RTSP connections needed
- Simpler implementation
- Lower memory overhead (segments already on disk)

**Cons:**
- Less precise (limited to segment boundaries, typically 2-6 seconds)
- Requires segment concatenation logic
- May have timestamp discontinuities

**Estimated Effort:** 1-2 days of development + testing

### Option 3: go2rtc Native Buffering (Future)
**Approach:** Leverage go2rtc's built-in buffering capabilities if available.

**Pros:**
- Offloads buffering to go2rtc
- Potentially more efficient
- Centralized buffer management

**Cons:**
- Requires go2rtc API support (need to verify)
- May not be available in current go2rtc version
- Less control over buffer behavior

**Estimated Effort:** Unknown (depends on go2rtc capabilities)

### Option 4: Hybrid Approach (Recommended)
**Approach:** Start with Option 2 (HLS segment-based) as a quick win, then migrate to Option 1 if more precision is needed.

**Phase 1 (Quick Win):**
1. Track the last N HLS segments per stream (configurable, default 3-5 segments = 6-30 seconds)
2. When detection occurs, copy those segments to a temp location
3. Use ffmpeg to concatenate them with the new MP4 recording
4. Clean up temp files

**Phase 2 (If Needed):**
1. Implement dedicated buffering thread using motion_buffer infrastructure
2. Migrate from HLS-based to packet-based buffering
3. Optimize memory usage and thread management

## Resource Considerations

### Memory Usage (Option 1 - Packet Buffer)
- **Per stream:** ~10-50 MB for 10 seconds of 1080p video
- **16 streams:** ~160-800 MB total
- **Mitigation:** Configurable buffer size, memory limits, disk fallback

### CPU Usage
- **Buffering thread:** Minimal (just reading and storing packets)
- **Detection:** No change (still uses HLS segments)
- **Recording:** Slight increase during buffer flush

### Disk I/O (Option 2 - HLS Segments)
- **Minimal:** Segments already written to disk
- **Concatenation:** One-time operation when detection occurs
- **Cleanup:** Delete temp files after concatenation

## Recommended Implementation Plan

### Phase 1: HLS Segment-Based Buffer (1-2 days)
1. **Track HLS segments** per stream in a circular array
   - Store last N segment paths (configurable via `pre_detection_buffer`)
   - Update array as new segments are created
   
2. **On detection event:**
   - Copy last N segments to temp directory
   - Create ffmpeg concat demuxer file
   - Start MP4 recording with concatenated input
   
3. **Cleanup:**
   - Delete temp files after successful concatenation
   - Handle errors gracefully

### Phase 2: Testing & Refinement (1 day)
1. Test with various buffer sizes (5, 10, 15, 30 seconds)
2. Verify timestamp continuity
3. Test with multiple simultaneous detections
4. Memory and disk usage profiling

### Phase 3: Documentation & Configuration (0.5 days)
1. Update configuration documentation
2. Add web UI controls for buffer size
3. Add logging for buffer operations

## Configuration

### Stream Config (Already Exists)
```c
typedef struct {
    // ... existing fields ...
    int pre_detection_buffer;  // Seconds to keep before detection (0-30)
    int post_detection_buffer; // Seconds to keep after detection
    // ... existing fields ...
} stream_config_t;
```

### Recommended Defaults
- `pre_detection_buffer`: 10 seconds (2-5 HLS segments)
- `post_detection_buffer`: 30 seconds (existing)
- Maximum: 30 seconds (to limit resource usage)

## Next Steps

1. **Decision:** Choose implementation approach (recommend Option 4 - Hybrid)
2. **Prototype:** Implement Phase 1 (HLS segment-based buffer)
3. **Test:** Verify functionality with real detection events
4. **Evaluate:** Determine if Phase 2 (packet-based buffer) is needed
5. **Deploy:** Roll out to production after testing

## Notes

- The `motion_buffer.h` include has been added to `detection_recording.c` for future use
- Structure fields are commented out with TODO markers
- Initialization logs note that pre-detection buffer requires architectural changes
- This document serves as a roadmap for implementation

## References

- `src/video/motion_buffer.c` - Existing ring buffer implementation
- `src/video/onvif_motion_recording.c` - Example of buffer integration
- `src/video/detection_recording.c` - Detection recording system
- `PRD-onivf-motion-detect.md` - Motion detection PRD with buffer details
- `PHASE2_IMPLEMENTATION_SUMMARY.md` - Motion buffer implementation summary

