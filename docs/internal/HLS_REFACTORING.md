# HLS Architecture Refactoring

## Overview

This document describes the refactoring of the HLS (HTTP Live Streaming) architecture in the NVR software. The refactoring addresses several issues with the previous architecture, including unnecessary thread layering, complex state management, and resource waste.

## Previous Architecture Issues

The previous architecture had the following problems:

1. **Unnecessary Thread Layering**:
   - A manager thread (`hls_stream_thread`) that just created another worker thread (`hls_writer_thread`)
   - This created twice as many threads as needed
   - Complicated synchronization and error handling

2. **Complex State Management**:
   - State was split between multiple contexts (`hls_stream_ctx_t` and `hls_writer_thread_ctx_t`)
   - Made it hard to track the actual state of a stream
   - Led to race conditions and memory leaks

3. **Resource Waste**:
   - Extra threads consumed system resources
   - The manager thread mostly just waited, doing very little actual work

## New Architecture: One Self-Managing Thread Per Stream

The new architecture uses a single, self-managing thread per stream that:

1. **Handles Everything for One Stream**:
   - Directly manages the RTSP connection
   - Processes frames and writes HLS segments
   - Handles its own error recovery

2. **Has a Single, Clear State**:
   - One context structure with all needed information
   - Clear state transitions
   - Simplified synchronization

3. **Provides a Clean API**:
   - Simple start/stop functions
   - Clear status reporting
   - Easy configuration

## Implementation Details

### Unified Context Structure

The new architecture uses a single context structure (`hls_unified_thread_ctx_t`) that combines the functionality of both the previous `hls_stream_ctx_t` and `hls_writer_thread_ctx_t`:

```c
typedef struct {
    // Stream identification
    char stream_name[MAX_STREAM_NAME];
    char rtsp_url[MAX_PATH_LENGTH];
    char output_path[MAX_PATH_LENGTH];
    
    // Thread management
    pthread_t thread;
    atomic_int running;
    int shutdown_component_id;
    
    // Stream configuration
    int protocol;  // STREAM_PROTOCOL_TCP or STREAM_PROTOCOL_UDP
    int segment_duration;
    
    // HLS writer (embedded directly instead of pointer)
    hls_writer_t *writer;
    
    // Connection state tracking
    atomic_int_fast64_t last_packet_time;
    atomic_int connection_valid;
    atomic_int consecutive_failures;
    atomic_int thread_state;  // Uses hls_thread_state_t values
} hls_unified_thread_ctx_t;
```

### State Machine

The unified thread uses a clear state machine to manage the stream lifecycle:

1. `HLS_THREAD_INITIALIZING`: Initial state when the thread starts
2. `HLS_THREAD_CONNECTING`: Attempting to connect to the RTSP stream
3. `HLS_THREAD_RUNNING`: Successfully connected and processing frames
4. `HLS_THREAD_RECONNECTING`: Lost connection and attempting to reconnect
5. `HLS_THREAD_STOPPING`: Thread is in the process of stopping
6. `HLS_THREAD_STOPPED`: Thread has stopped

### Simplified API

The API has been simplified to just a few key functions:

- `start_hls_stream(const char *stream_name)`: Start HLS streaming for a stream
- `stop_hls_stream(const char *stream_name)`: Stop HLS streaming for a stream
- `restart_hls_stream(const char *stream_name)`: Force restart of HLS streaming for a stream
- `is_hls_stream_active(const char *stream_name)`: Check if HLS streaming is active for a stream

## Benefits of the New Architecture

1. **Reduced Resource Usage**:
   - Half as many threads (one per stream instead of two)
   - Less memory usage due to simplified context structure
   - Less CPU usage due to reduced thread context switching

2. **Improved Reliability**:
   - Simplified error handling and recovery
   - Clearer state management
   - Reduced chance of race conditions and memory leaks

3. **Better Maintainability**:
   - Simpler code structure
   - Clearer responsibility boundaries
   - Easier to debug and extend

4. **Enhanced Performance**:
   - Reduced latency due to fewer thread handoffs
   - More efficient resource utilization
   - Better scalability with more streams

## Files Changed

1. New files:
   - `include/video/hls/hls_unified_thread.h`: Header for the unified thread implementation
   - `src/video/hls/hls_unified_thread.c`: Implementation of the unified thread

2. Modified files:
   - `include/video/hls_streaming.h`: Updated to use the unified thread architecture
   - `src/video/hls_streaming.c`: Updated to use the unified thread architecture

3. Deprecated files (no longer used but kept for reference):
   - `src/video/hls/hls_stream_thread.c`: Old manager thread implementation
   - `src/video/hls/hls_api.c`: Old API implementation
   - `src/video/hls_writer_thread.c`: Old worker thread implementation

## Conclusion

The refactored HLS architecture provides a more efficient, reliable, and maintainable solution for HLS streaming. By eliminating unnecessary thread layering and simplifying state management, the new architecture addresses the key issues with the previous implementation while providing a cleaner API for the rest of the system to use.
