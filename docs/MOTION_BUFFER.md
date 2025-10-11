# Motion Buffer System

## Overview

The Motion Buffer System provides circular buffering for video packets to enable pre-event recording in motion-triggered recordings. When motion is detected, the buffer is flushed to include video from before the motion event started.

## Architecture

### Components

1. **Motion Buffer Pool** - Global pool managing buffers for all streams
2. **Motion Buffer** - Per-stream circular buffer for video packets
3. **Buffered Packet** - Individual packet storage with metadata
4. **Integration** - Hooks into ONVIF motion recording system

### Buffer States

The motion recording system uses four states:

- **IDLE** - No buffering, no recording
- **BUFFERING** - Actively buffering packets, waiting for motion
- **RECORDING** - Motion detected, actively recording
- **FINALIZING** - Motion ended, post-buffer active

### State Transitions

```
IDLE → BUFFERING (when buffer is enabled)
BUFFERING → RECORDING (when motion is detected)
RECORDING → FINALIZING (when motion ends)
FINALIZING → BUFFERING (when post-buffer timeout expires)
FINALIZING → RECORDING (if new motion detected during post-buffer)
```

## Features

### Circular Buffer

- **Configurable Duration**: 5-30 seconds of pre-event video
- **Automatic Packet Management**: Oldest packets automatically dropped when full
- **Memory Efficient**: Only stores packet references, not decoded frames
- **Thread-Safe**: Mutex-protected operations

### Memory Management

- **Pool-Based Allocation**: Centralized memory management
- **Per-Stream Limits**: Configurable memory limits per buffer
- **Global Limits**: Total memory limit across all buffers
- **Statistics Tracking**: Real-time memory usage monitoring

### Packet Storage

Each buffered packet stores:
- AVPacket data (video/audio)
- Timestamp
- PTS/DTS values
- Stream index
- Keyframe flag
- Data size

## API Reference

### Initialization

```c
// Initialize the buffer pool
int init_motion_buffer_pool(size_t memory_limit_mb);

// Cleanup the buffer pool
void cleanup_motion_buffer_pool(void);
```

### Buffer Management

```c
// Create a buffer for a stream
motion_buffer_t* create_motion_buffer(
    const char *stream_name,
    int buffer_seconds,
    buffer_mode_t mode
);

// Destroy a buffer
void destroy_motion_buffer(motion_buffer_t *buffer);

// Get buffer by stream name
motion_buffer_t* get_motion_buffer(const char *stream_name);
```

### Packet Operations

```c
// Add a packet to the buffer
int motion_buffer_add_packet(
    motion_buffer_t *buffer,
    const AVPacket *packet,
    time_t timestamp
);

// Peek at oldest packet (without removing)
int motion_buffer_peek_oldest(
    motion_buffer_t *buffer,
    AVPacket **packet
);

// Remove and return oldest packet
int motion_buffer_pop_oldest(
    motion_buffer_t *buffer,
    AVPacket **packet
);

// Flush all packets to a callback
int motion_buffer_flush(
    motion_buffer_t *buffer,
    int (*callback)(const AVPacket *packet, void *user_data),
    void *user_data
);

// Clear all packets
void motion_buffer_clear(motion_buffer_t *buffer);
```

### Statistics

```c
// Get buffer statistics
int motion_buffer_get_stats(
    motion_buffer_t *buffer,
    int *count,
    size_t *memory_usage,
    int *duration
);

// Check if buffer is ready
bool motion_buffer_is_ready(motion_buffer_t *buffer);

// Get keyframe count
int motion_buffer_get_keyframe_count(motion_buffer_t *buffer);

// Get total memory usage
size_t motion_buffer_get_total_memory_usage(void);
```

## Integration with Motion Recording

### Feeding Packets

Video packets should be continuously fed to the buffer:

```c
// Called for every video packet
int feed_packet_to_motion_buffer(
    const char *stream_name,
    const AVPacket *packet
);
```

### Buffer Flush on Motion Detection

When motion is detected, the buffer is automatically flushed:

1. Motion event triggers `start_motion_recording_internal()`
2. Function checks if buffer exists and hasn't been flushed
3. Calls `motion_buffer_flush()` with callback
4. Callback writes packets to recording file
5. Sets `buffer_flushed` flag to prevent duplicate flush

### Overlapping Events

The system handles overlapping motion events intelligently:

- **During RECORDING**: Updates `last_motion_time` to extend recording
- **During FINALIZING**: Transitions back to RECORDING
- **Result**: Single continuous recording instead of multiple files

## Configuration

### Per-Stream Configuration

```c
motion_recording_config_t config = {
    .enabled = true,
    .pre_buffer_seconds = 10,    // 10 seconds of pre-event video
    .post_buffer_seconds = 15,   // 15 seconds after motion ends
    .max_file_duration = 600,    // Max 10 minutes per file
    .retention_days = 30
};

enable_motion_recording("camera_name", &config);
```

### Buffer Modes

- **BUFFER_MODE_MEMORY**: Store packets in RAM (default)
- **BUFFER_MODE_DISK**: Store packets on disk (placeholder)
- **BUFFER_MODE_HYBRID**: Memory with disk fallback (placeholder)

## Memory Considerations

### Memory Usage Calculation

For a single stream:
```
Memory = FPS × buffer_seconds × avg_packet_size × 1.2
```

Example (15 FPS, 5 second buffer, 50KB avg packet):
```
Memory = 15 × 5 × 50KB × 1.2 = 4.5 MB
```

### Recommendations

- **Low Memory Systems** (< 256MB RAM): 5 second buffer, 2-3 streams max
- **Medium Systems** (256-512MB RAM): 10 second buffer, 4-8 streams
- **High Memory Systems** (> 512MB RAM): 30 second buffer, 16 streams

### Memory Pool Limits

The default pool limit is 50MB. Adjust based on system resources:

```c
// Initialize with custom limit (100MB)
init_motion_buffer_pool(100);
```

## Performance

### Benchmarks

- **Add Packet**: < 1ms (packet clone + metadata)
- **Flush Buffer**: ~10-50ms (depends on packet count)
- **Memory Overhead**: ~100 bytes per packet + packet data

### Optimization Tips

1. **Reduce Buffer Duration**: Shorter buffers use less memory
2. **Lower FPS**: Fewer packets to buffer
3. **Keyframe-Only Buffering**: Store only keyframes (future enhancement)
4. **Compression**: Use higher compression for lower bitrates

## Troubleshooting

### Buffer Not Filling

**Symptoms**: Buffer shows 0 packets
**Causes**:
- `feed_packet_to_motion_buffer()` not being called
- Buffer not enabled in configuration
- Stream not active

**Solution**: Verify packet feeding integration

### High Memory Usage

**Symptoms**: System running out of memory
**Causes**:
- Too many streams with large buffers
- High bitrate streams
- Memory leak

**Solution**:
- Reduce buffer duration
- Lower stream bitrate
- Check for memory leaks with valgrind

### Buffer Flush Not Working

**Symptoms**: Pre-event video not in recording
**Causes**:
- Callback function not writing packets
- Buffer already flushed
- Buffer empty when motion detected

**Solution**: Verify callback implementation and buffer state

## Future Enhancements

### Planned Features

1. **Disk-Based Buffering**: Full implementation for low-memory systems
2. **Keyframe-Only Mode**: Store only keyframes to reduce memory
3. **Adaptive Buffering**: Automatically adjust buffer size based on available memory
4. **Buffer Compression**: Compress buffered packets to save memory
5. **Multi-Stream Optimization**: Share buffers between similar streams

### API Extensions

```c
// Future APIs (not yet implemented)
int motion_buffer_set_keyframe_only(motion_buffer_t *buffer, bool enabled);
int motion_buffer_set_compression(motion_buffer_t *buffer, bool enabled);
int motion_buffer_set_adaptive_size(motion_buffer_t *buffer, bool enabled);
```

## See Also

- [ONVIF Motion Recording](ONVIF_MOTION_RECORDING.md)
- [ONVIF Detection](ONVIF_DETECTION.md)
- [Product Requirements Document](../PRD-onivf-motion-detect.md)

